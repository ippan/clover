use crate::runtime::assembly::*;
use crate::runtime::opcode::*;
use crate::runtime::object::Object;
use crate::ast::*;
use crate::ast::token::Token;
use std::ops::Deref;
use std::collections::HashMap;
use crate::runtime::opcode::OpCode::Pop;

pub struct FreeVariableIndex {
    pub upper_index: u16,
    pub local_index: u16
}

pub struct Scope {
    pub instructions: Vec<Instruction>,
    pub parameters: Vec<String>,
    pub locals: HashMap<String, usize>,
    pub frees: HashMap<String, FreeVariableIndex>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            instructions: Vec::new(),
            parameters: Vec::new(),
            locals: HashMap::new(),
            frees: HashMap::new()
        }
    }

    pub fn add_local(&mut self, name: String) -> usize {
        let index = self.locals.len();
        self.locals.insert(name, index);
        index
    }

    pub fn to_function(& self) -> Function {
        Function {
            parameter_count: self.parameters.len() as u16,
            local_variable_count: self.locals.len() as u16,
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
            let index = self.current_scope().add_local(local_name) as u64;
            self.emit(OpCode::SetLocal.to_instruction(index));
        })
    }

    fn compile_return_statement(&mut self, data: &ReturnStatementData) -> Result<(), String> {
        self.compile_expression(&data.expression)?;
        self.emit_opcode(OpCode::Return);
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
        let constant_index = self.constants.len() as u64;

        unwrap_token!(Token::Integer(value), data.data, {
            self.constants.push(Object::Integer(value));
            self.emit(OpCode::PushConstant.to_instruction(constant_index))
        })
    }

    fn compile_float_literal_expression(&mut self, data: &FloatLiteralExpressionData) -> Result<(), String> {
        let constant_index = self.constants.len() as u64;

        unwrap_token!(Token::Float(value), data.data, {
            self.constants.push(Object::Float(value));
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

    fn compile_expression(&mut self, data: &Expression) -> Result<(), String> {

        match data {
            Expression::IntegerLiteral(data) => self.compile_integer_literal_expression(data.deref()),
            Expression::FloatLiteral(data) => self.compile_float_literal_expression(data.deref()),
            Expression::BooleanLiteral(data) => self.compile_boolean_literal_expression(data.deref()),
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

    fn compile_function(&mut self, data: &FunctionExpressionData) {


    }

    fn apply_scope_to_assembly(&mut self, scope: &Scope) -> usize {
        let index = self.assembly.as_ref().unwrap().functions.len();
        self.assembly.as_mut().unwrap().functions.push(scope.to_function());
        index
    }

    fn compile_scope(&mut self, codes: &Codes, parameters: &[String]) -> Scope {
        self.push_scope();

        for parameter_name in parameters {
            self.current_scope().add_local(parameter_name.clone());
        };

        for statement in codes {
            self.compile_statement(statement);
        };

        if let Some(instruction) = self.current_scope().instructions.last() {
            match instruction.opcode() {
                OpCode::Pop => {
                    // last statement is a expression statement, just return that expression
                    let instruction_index = self.current_scope().instructions.len() - 1;
                    self.replace_instruction(instruction_index, OpCode::Return.to_instruction(0));
                },
                OpCode::Return => {
                    // do nothing
                },
                _ => {
                    // last statement is not a expression statement, return null
                    self.emit_opcode(OpCode::PushNull);
                    self.emit_opcode(OpCode::Return);
                }
            };
        } else {
            // do not have any statement, return null
            self.emit_opcode(OpCode::PushNull);
            self.emit_opcode(OpCode::Return);
        };

        self.pop_scope()
    }

    fn push_scope(&mut self) {
        let scope = Scope::new();
        self.scopes.push(scope);
    }

    fn pop_scope(&mut self) -> Scope {
        self.scopes.pop().unwrap()
    }

    pub fn compile(&mut self, program: &Program) -> Result<Assembly, String> {
        self.scopes.clear();
        self.constants.clear();
        self.assembly = Some(Assembly::new());

        let scope = self.compile_scope(&program.codes, &[]);

        self.apply_scope_to_assembly(&scope);

        self.assembly.as_mut().unwrap().constants = self.constants.clone();

        Ok(self.assembly.take().unwrap())
    }
}