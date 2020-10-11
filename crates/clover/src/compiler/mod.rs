use crate::runtime::assembly::*;
use crate::runtime::opcode::*;
use crate::runtime::object::Object;
use crate::ast::*;
use crate::ast::token::Token;
use std::ops::Deref;
use std::collections::HashMap;

const MAX_LOCAL_COUNT : usize = 65536;

pub enum ScopeType {
    Normal,
    Function
}

pub struct Scope {
    pub scope_type: ScopeType,
    pub instructions: Vec<Instruction>,
    pub parameter_count: u16,
    pub locals: HashMap<String, usize>,
    pub frees: Vec<FreeVariableIndex>,
}

impl Scope {
    pub fn new(scope_type: ScopeType) -> Scope {
        Scope {
            scope_type,
            instructions: Vec::new(),
            parameter_count: 0,
            locals: HashMap::new(),
            frees: Vec::new()
        }
    }

    pub fn add_local(&mut self, name: String) -> Result<usize, String> {
        let index = self.locals.len();

        if self.locals.contains_key(&name) {
            return Err(format!("local variable [{}] already exists", name));
        };

        self.locals.insert(name, index);

        if self.locals.len() > MAX_LOCAL_COUNT {
            Err(format!("can not declare more than {} local variables(with free variables).", MAX_LOCAL_COUNT))
        } else {
            Ok(index)
        }
    }

    pub fn get_local_index(&self, name: &str) -> Option<usize> {
        if let Some(&index) = self.locals.get(name) {
            Some(index)
        } else {
            None
        }
    }

    pub fn add_free(&mut self, name: String, upper_index: usize) -> Result<usize, String> {
        let index = self.add_local(name)?;

        self.frees.push(FreeVariableIndex {
            upper_index: upper_index as u16,
            local_index: index as u16
        });

        Ok(index)
    }

    pub fn to_function(&self) -> Function {
        Function {
            parameter_count: self.parameter_count,
            local_variable_count: self.locals.len() as u16,
            free_variables: self.frees.clone(),
            instructions: self.instructions.clone()
        }
    }
}

pub struct Compiler {
    scopes: Vec<Scope>,
    constants: Vec<Object>,
    assembly: Option<Assembly>
}

macro_rules! unwrap_token {
    ($token_pattern: pat, $token_data: expr, $execute: block) => {
        if let $token_pattern = $token_data.token {
            Ok($execute)
        } else {
            Err("token value not match".to_string())
        }
    }
}

impl Compiler {

    pub fn new() -> Compiler {
        Compiler {
            scopes: Vec::new(),
            constants: Vec::new(),
            assembly: None
        }
    }

    fn compile_local_statement(&mut self, data: &LocalStatementData) -> Result<(), String> {

        if let Some(expression) = data.expression.as_ref() {
            self.compile_expression(expression)?;
        } else {
            self.emit_opcode(OpCode::PushNull);
        };

        unwrap_token!(Token::Identifier(local_name), data.identifier.clone(), {
            let index = self.current_scope().add_local(local_name)?;
            self.emit(OpCode::LocalSet.to_instruction((index | 0x0001000000000000) as u64))
        })
    }

    fn generate_return_instruction(&self) -> Instruction {
        let depth = self.nearest_function_depth();
        OpCode::Return.to_instruction(depth as u64)
    }

    fn compile_return_statement(&mut self, data: &ReturnStatementData) -> Result<(), String> {
        self.compile_expression(&data.expression)?;

        self.emit(self.generate_return_instruction());
        Ok(())
    }

    fn compile_expression_statement(&mut self, data: &ExpressionStatementData) -> Result<(), String> {
        self.compile_expression(&data.expression)?;

        self.emit_opcode(OpCode::Pop);

        Ok(())
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), String> {

        match statement {
            Statement::Local(data) => self.compile_local_statement(data.deref()),
            Statement::Return(data) => self.compile_return_statement(data.deref()),
            Statement::Expression(data) => self.compile_expression_statement(data.deref())
        }
    }

    fn compile_integer_literal_expression(&mut self, data: &IntegerLiteralExpressionData) -> Result<(), String> {
        unwrap_token!(Token::Integer(value), data.data, {
            let constant_index = self.add_integer_constant(value) as u64;
            self.emit(OpCode::PushConstant.to_instruction(constant_index))
        })
    }

    fn compile_float_literal_expression(&mut self, data: &FloatLiteralExpressionData) -> Result<(), String> {
        unwrap_token!(Token::Float(value), data.data, {
            let constant_index = self.add_float_constant(value) as u64;
            self.emit(OpCode::PushConstant.to_instruction(constant_index))
        })
    }

    fn compile_string_literal_expression(&mut self, data: &StringLiteralExpressionData) -> Result<(), String> {
        unwrap_token!(Token::String(value), data.data.clone(), {
            let constant_index = self.add_string_constant(value) as u64;
            self.emit(OpCode::PushConstant.to_instruction(constant_index))
        })
    }

    fn compile_boolean_literal_expression(&mut self, data: &BooleanLiteralExpressionData) -> Result<(), String> {
        match data.data.token {
            Token::True => {
                self.emit(OpCode::PushBoolean.to_instruction(1));
                Ok(())
            },
            Token::False => {
                self.emit(OpCode::PushBoolean.to_instruction(0));
                Ok(())
            },
            _ => Err("token value not match".to_string())
        }
    }

    fn compile_null_literal_expression(&mut self, _: &NullLiteralExpressionData) -> Result<(), String> {
        self.emit_opcode(OpCode::PushNull);
        Ok(())
    }

    fn compile_identifier_expression(&mut self, data: &IdentifierExpressionData) -> Result<(), String> {
        unwrap_token!(Token::Identifier(name), data.data.clone(), {

            if let Some(local_index) = self.ensure_local(&name)? {
                self.emit(OpCode::LocalGet.to_instruction(local_index as u64))
            } else {
                let constant_index = self.add_string_constant(name) as u64;
                self.emit(OpCode::EnvironmentGet.to_instruction(constant_index))
            }

        })
    }

    fn compile_assign_expression(&mut self, data: &InfixExpressionData) -> Result<(), String> {

        self.compile_expression(&data.right)?;

        match &data.left {
            Expression::Identifier(identifier_data) => {
                if let Token::Identifier(identifier) = &identifier_data.deref().data.token {
                    if let Some(local_index) = self.ensure_local(identifier)? {
                        self.emit(OpCode::LocalSet.to_instruction(local_index  as u64))
                    } else {
                        let constant_index = self.add_string_constant(identifier.clone()) as u64;
                        self.emit(OpCode::EnvironmentSet.to_instruction(constant_index))
                    }
                }

                Ok(())
            },
            _ => Err("not implemented".to_string())
        }



    }

    fn compile_infix_expression(&mut self, data: &InfixExpressionData) -> Result<(), String> {
        match data.infix.token {
            Token::Assign => { return self.compile_assign_expression(data); },
            _ => {}
        };

        self.compile_expression(&data.left)?;
        self.compile_expression(&data.right)?;

        match data.infix.token {
            Token::Plus => self.emit_opcode(OpCode::Add),
            Token::Minus => self.emit_opcode(OpCode::Sub),
            Token::Star => self.emit_opcode(OpCode::Multiply),
            Token::Slash => self.emit_opcode(OpCode::Divide),

            _ => { return Err(format!("unknown operator [{:?}]", data.infix.token).to_string()); }
        }

        Ok(())
    }

    fn compile_function_expression(&mut self, data: &FunctionExpressionData) -> Result<(), String> {

        let mut parameters = Vec::new();

        for parameter in &data.parameters {
            if let Token::Identifier(parameter_name) = &parameter.token {
                parameters.push(parameter_name.clone());
            }
        }

        let scope = self.compile_scope(&data.body, &parameters, ScopeType::Function)?;

        let function_index = self.apply_scope_to_assembly(&scope);

        self.emit(OpCode::Closure.to_instruction(function_index as u64));

        Ok(())
    }

    fn compile_call_expression(&mut self, data: &CallExpressionData) -> Result<(), String> {
        for expression in &data.parameters {
            self.compile_expression(expression)?;
        };

        self.compile_expression(&data.function)?;

        self.emit(OpCode::Call.to_instruction(data.parameters.len() as u64));

        Ok(())
    }

    fn compile_class_expression(&mut self, data: &ClassExpressionData) -> Result<(), String> {
        self.emit_opcode(OpCode::PushNewMap);

        for member in data.members.iter() {
            if let Token::Identifier(identifier) = &member.identifier.token {
                let constant_index = self.add_string_constant(identifier.clone()) as u64;
                self.emit(OpCode::PushConstant.to_instruction(constant_index));

                if let Some(expression) = &member.expression {
                    self.compile_expression(expression)?;
                } else {
                    self.emit_opcode(OpCode::PushNull);
                };

                self.emit(OpCode::InstanceSet.to_instruction(1));

            } else {
                return Err("unknown token when compile class".to_string());
            };
        };

        // TODO : implement extents

        Ok(())
    }

    fn compile_instance_get_expression(&mut self, data: &InstanceGetExpressionData) -> Result<(), String> {
        self.compile_expression(&data.instance)?;

        self.compile_expression(&data.index)?;

        self.emit_opcode(OpCode::InstanceGet);

        Ok(())
    }

    fn compile_expression(&mut self, data: &Expression) -> Result<(), String> {

        match data {
            Expression::IntegerLiteral(data) => self.compile_integer_literal_expression(data.deref()),
            Expression::FloatLiteral(data) => self.compile_float_literal_expression(data.deref()),
            Expression::BooleanLiteral(data) => self.compile_boolean_literal_expression(data.deref()),
            Expression::StringLiteral(data) => self.compile_string_literal_expression(data.deref()),
            Expression::NullLiteral(data) => self.compile_null_literal_expression(data.deref()),
            Expression::Identifier(data) => self.compile_identifier_expression(data.deref()),
            Expression::Infix(data) => self.compile_infix_expression(data.deref()),
            Expression::Function(data) => self.compile_function_expression(data.deref()),
            Expression::Call(data) => self.compile_call_expression(data.deref()),
            Expression::Class(data) => self.compile_class_expression(data.deref()),
            Expression::InstanceGet(data) => self.compile_instance_get_expression(data.deref()),
            _ => Err("not implemented".to_string())
        }
    }

    fn current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    fn emit(&mut self, instruction: Instruction) {
        self.current_scope().instructions.push(instruction);
    }

    fn emit_opcode(&mut self, opcode: OpCode) {
        self.emit(opcode.to_instruction(0));
    }

    fn replace_instruction(&mut self, index: usize, instruction: Instruction) {
        self.current_scope().instructions[index] = instruction;
    }

    fn apply_scope_to_assembly(&mut self, scope: &Scope) -> usize {
        let index = self.assembly.as_ref().unwrap().functions.len();
        self.assembly.as_mut().unwrap().functions.push(scope.to_function());
        index
    }

    fn compile_scope(&mut self, codes: &Codes, parameters: &[String], scope_type: ScopeType) -> Result<Scope, String> {
        self.push_scope(scope_type);

        self.current_scope().parameter_count = parameters.len() as u16;

        for parameter_name in parameters {
            self.current_scope().add_local(parameter_name.clone())?;
        };

        for statement in codes {
            self.compile_statement(statement)?;
        };

        if let Some(instruction) = self.current_scope().instructions.last() {
            match instruction.opcode() {
                OpCode::Pop => {
                    // last statement is a expression statement, just return that expression
                    let instruction_index = self.current_scope().instructions.len() - 1;
                    self.replace_instruction(instruction_index, self.generate_return_instruction());
                },
                OpCode::Return => {
                    // do nothing
                },
                _ => {
                    // last statement is not a expression statement, return null
                    self.emit_opcode(OpCode::PushNull);
                    self.emit(self.generate_return_instruction());
                }
            };
        } else {
            // do not have any statement, return null
            self.emit_opcode(OpCode::PushNull);
            self.emit(self.generate_return_instruction());
        };

        Ok(self.pop_scope())
    }

    fn push_scope(&mut self, scope_type: ScopeType) {
        let scope = Scope::new(scope_type);
        self.scopes.push(scope);
    }

    fn pop_scope(&mut self) -> Scope {
        self.scopes.pop().unwrap()
    }

    fn nearest_function_depth(&self) -> usize {
        let mut depth = 1;

        for scope in self.scopes.iter().rev() {
            if let ScopeType::Function = scope.scope_type {
                return depth;
            }

            depth += 1;
        }

        return 0;
    }

    fn find_local_variable(&mut self, name: &str) -> Option<(usize, usize)> {
        for i in (0..(self.scopes.len())).rev() {
            if let Some(scope) = self.scopes.get(i) {
                if let Some(index) = scope.get_local_index(name) {
                    return Some((i, index));
                }
            }
        };

        None
    }

    fn ensure_local(&mut self, name: &str) -> Result<Option<usize>, String> {
        if let Some(&index) = self.current_scope().locals.get(name) {
            return Ok(Some(index));
        };

        if let Some((scope_index, local_index)) = self.find_local_variable(name) {
            let mut current_local_index = local_index;

            for i in (scope_index + 1)..self.scopes.len() {
                current_local_index = self.scopes[i].add_free(name.to_string(), current_local_index)?;
            };

            Ok(Some(current_local_index))
        } else {
            Ok(None)
        }
    }

    // TODO : move this there add constant to generic or marco

    fn add_integer_constant(&mut self, value: i64) -> usize {
        // TODO : may be add a hash map to increase search performance
        for i in 0..self.constants.len() {
            if let Object::Integer(stored_value) = self.constants[i] {
                if value == stored_value {
                    return i;
                };
            };
        };

        let constant_index = self.constants.len();
        self.constants.push(Object::Integer(value));
        constant_index
    }

    fn add_float_constant(&mut self, value: f64) -> usize {
        // TODO : may be add a hash map to increase search performance
        for i in 0..self.constants.len() {
            if let Object::Float(stored_value) = self.constants[i] {
                if value == stored_value {
                    return i;
                };
            };
        };

        let constant_index = self.constants.len();
        self.constants.push(Object::Float(value));
        constant_index
    }

    fn add_string_constant(&mut self, value: String) -> usize {
        // TODO : may be add a hash map to increase search performance
        for i in 0..self.constants.len() {
            if let Object::String(stored_value) = &self.constants[i] {
                if value == *stored_value {
                    return i;
                };
            };
        };

        let constant_index = self.constants.len();
        self.constants.push(Object::String(value));
        constant_index
    }



    pub fn compile(&mut self, program: &Program) -> Result<Assembly, String> {
        self.scopes.clear();
        self.constants.clear();
        self.assembly = Some(Assembly::new());

        let scope = self.compile_scope(&program.codes, &[], ScopeType::Function)?;

        self.apply_scope_to_assembly(&scope);

        self.assembly.as_mut().unwrap().constants = self.constants.clone();

        Ok(self.assembly.take().unwrap())
    }
}