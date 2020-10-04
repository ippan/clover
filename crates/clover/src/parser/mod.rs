mod lexer;

use crate::ast::token::{TokenData, Token};
use crate::ast::{Program, Statement, LocalStatementData, Expression};
use crate::parser::lexer::Lexer;

#[derive(Debug, PartialEq, PartialOrd)]
enum SymbolPriority {
    Lowest      = 0,
    Assign      = 1,
    Boolean     = 2,
    Equals      = 3,
    LessGreater = 4,
    Sum         = 5,
    Product     = 6,
    Prefix      = 7,
    Call        = 8,
    InstanceGet = 9
}


pub struct Parser {
    lexer: Option<Lexer>,

    current_token_data: TokenData,
    peek_token_data: TokenData
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            lexer: None,
            current_token_data: TokenData::new(Token::None, 0),
            peek_token_data: TokenData::new(Token::None, 0)
        }
    }

    pub fn parse(&mut self, source: String, filename: String) -> Result<Program, String> {
        self.lexer = Some(Lexer::new(source.as_str()));

        // fill token data
        self.next_token();
        self.next_token();

        self.parse_program(filename)
    }

    fn next_token(&mut self) {
        self.current_token_data = self.peek_token_data.clone();

        if let Some(lexer) = self.lexer.as_mut() {
            self.peek_token_data = lexer.lex();
        }
    }

    fn parse_program(&mut self, filename: String) -> Result<Program, String> {
        let mut program = Program::new(filename);

        while self.current_token_data.token != Token::Eof {
            program.codes.push(self.parse_statement()?);
            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token_data.token {
            Token::Local => self.parse_local_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement()
        }
    }

    fn parse_local_statement(&mut self) -> Result<Statement, String> {
        self.next_token();

        let identifier = self.current_token_data.clone();

        if let Token::Identifier(_) = identifier.token {
            self.next_token();
        } else {
            return Err("Not implement yet".to_string());
        };

        let mut expression = None;

        if self.current_token_data.token == Token::Assign {
            self.next_token();
            expression = Some(self.parse_expression()?);
        }

        Ok(Statement::Local(Box::new(LocalStatementData { identifier, expression })))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        Err("Not implement yet".to_string())
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        Err("Not implement yet".to_string())
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        Err("Not implement yet".to_string())
    }

}