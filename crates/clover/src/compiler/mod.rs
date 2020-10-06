use crate::runtime::assembly::{Assembly, Function};
use crate::ast::{Program, Statement, Expression, LocalStatementData, ReturnStatementData, ExpressionStatementData, IntegerLiteralExpressionData, FunctionExpressionData};
use std::ops::Deref;
use crate::runtime::object::Object;
use crate::ast::token::Token;
use crate::runtime::opcode::OpCode;
use std::collections::HashMap;

pub struct FreeVariableIndex {
    pub upper_index: u16,
    pub local_index: u16
}

pub struct Scope {
    pub instructions: Vec<u64>,
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
        Err("not implemented".to_string())
    }

    fn compile_return_statement(&mut self, data: &ReturnStatementData) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn compile_expression_statement(&mut self, data: &ExpressionStatementData) -> Result<(), String> {
        match &data.expression {
            Expression::IntegerLiteral(data) => self.compile_integer_literal_expression(data.deref()),
            _ => Err("not implemented".to_string())
        }
    }

    fn compile_statement(&mut self, statement: &Statement, assembly: &mut Assembly) -> Result<(), String> {

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
            self.emit(OpCode::Constant.to_instruction(constant_index))
        })
    }

    fn current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    fn emit(&mut self, instruction: u64) {
        self.current_scope().instructions.push(instruction);
    }

    fn compile_function(&mut self, data: &FunctionExpressionData) {


    }

    fn apply_scope_to_assembly(&mut self, scope: &Scope) -> usize {
        let index = self.assembly.as_ref().unwrap().functions.len();
        self.assembly.as_mut().unwrap().functions.push(scope.to_function());
        index
    }

    fn compile_scope(&mut self, parameters: &[String]) -> Scope {
        self.push_scope();

        for parameter_name in parameters {
            self.current_scope().add_local(parameter_name.clone());
        }

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





        Ok(self.assembly.take().unwrap())
    }
}