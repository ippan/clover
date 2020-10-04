mod lexer;

use crate::ast::token::{TokenData, Token};
use crate::ast::{Program, Statement, LocalStatementData, Expression, ExpressionStatementData, IdentifierExpressionData, InfixExpressionData, IntegerLiteralExpressionData, BaseLiteralExpressionData, NullLiteralExpressionData, ThisLiteralExpressionData, PrefixExpressionData};
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

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub token_data: TokenData
}

macro_rules! parse_error {
    ($token_data: expr, $message: expr) => {
        Err(ParseError { token_data: $token_data.clone(), message: $message.to_string() })
    }
}

macro_rules! match_token {
    ($token: ident, $($key: expr => $value: expr), *) => {
        match $token {
        $(
            $key => Some($value),
        )*
            _ => None
        }
    }
}

pub struct Parser {
    lexer: Option<Lexer>,

    current_token_data: TokenData,
    peek_token_data: TokenData
}

type ParsePrefixExpression = fn(&mut Parser) -> Result<Expression, ParseError>;
type ParseInfixExpression = fn(&mut Parser, Expression) -> Result<Expression, ParseError>;

impl Parser {
    pub fn new() -> Parser {
        Parser {
            lexer: None,
            current_token_data: TokenData::new(Token::None, 0),
            peek_token_data: TokenData::new(Token::None, 0)
        }
    }

    pub fn parse(&mut self, source: String, filename: String) -> Result<Program, ParseError> {
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

    fn get_current_precedence(&self) -> SymbolPriority {
        match self.current_token_data.token {
            Token::Assign | Token::PlusAssign | Token::MinusAssign | Token::StarAssign | Token::SlashAssign => SymbolPriority::Assign,
            Token::And | Token::Or => SymbolPriority::Boolean,
            Token::Equal | Token::NotEqual => SymbolPriority::Equals,
            Token::Less | Token::Greater | Token::LessEqual | Token::GreaterEqual => SymbolPriority::LessGreater,
            Token::Plus | Token::Minus => SymbolPriority::Sum,
            Token::Star | Token::Slash | Token::BitAnd | Token::BitOr => SymbolPriority::Product,
            Token::Dot | Token::LeftBracket => SymbolPriority::InstanceGet,
            Token::LeftParentheses => SymbolPriority::Call,
            _ => SymbolPriority::Lowest
        }
    }

    fn parse_program(&mut self, filename: String) -> Result<Program, ParseError> {
        let mut program = Program::new(filename);

        while self.current_token_data.token != Token::Eof {
            program.codes.push(self.parse_statement()?);
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current_token_data.token {
            Token::Local => self.parse_local_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement()
        }
    }

    fn parse_local_statement(&mut self) -> Result<Statement, ParseError> {
        self.next_token();

        let identifier = self.current_token_data.clone();

        if let Token::Identifier(_) = identifier.token {
            self.next_token();
        } else {
            return parse_error!(identifier, "Not implement yet");
        };

        let mut expression = None;

        if self.current_token_data.token == Token::Assign {
            self.next_token();
            expression = Some(self.parse_expression(SymbolPriority::Lowest)?);
        }

        Ok(Statement::Local(Box::new(LocalStatementData { identifier, expression })))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        parse_error!(self.current_token_data, "Not implement yet")
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.parse_expression(SymbolPriority::Lowest)?;

        Ok(Statement::Expression(Box::new(ExpressionStatementData { expression })))
    }

    fn parse_expression(&mut self, precedence: SymbolPriority) -> Result<Expression, ParseError> {
        let prefix_function = self.get_prefix_expression()?;

        let mut left_expression = prefix_function(self)?;

        let mut current_token_data = self.current_token_data.clone();

        while current_token_data.token != Token::Eof && precedence < self.get_current_precedence() {

            if let Some(infix_function) = self.get_infix_expression()? {

                left_expression = infix_function(self, left_expression)?;

                current_token_data = self.current_token_data.clone();
            } else {
                return Ok(left_expression);
            }
        };

        Ok(left_expression)
    }

    fn parse_identifier_expression(&mut self) -> Result<Expression, ParseError> {
        let identifier = self.current_token_data.clone();
        if let Token::Identifier(_) = identifier.token {
            self.next_token();
            Ok(Expression::Identifier(Box::new(IdentifierExpressionData { data: identifier })))
        } else {
            parse_error!(self.current_token_data, "Unexpect token (expect Identifier)")
        }
    }

    fn parse_integer_expression(&mut self) -> Result<Expression, ParseError> {
        let integer = self.current_token_data.clone();
        if let Token::Integer(_) = integer.token {
            self.next_token();
            Ok(Expression::IntegerLiteral(Box::new(IntegerLiteralExpressionData { data: integer })))
        } else {
            parse_error!(self.current_token_data, "Unexpect token (expect Identifier)")
        }
    }

    fn parse_keyword_expression(&mut self) -> Result<Expression, ParseError> {
        let keyword = self.current_token_data.clone();
        match keyword.token {
            Token::Base => Ok(Expression::BaseLiteral(Box::new(BaseLiteralExpressionData { data: keyword }))),
            Token::This => Ok(Expression::ThisLiteral(Box::new(ThisLiteralExpressionData { data: keyword }))),
            Token::Null => Ok(Expression::NullLiteral(Box::new(NullLiteralExpressionData { data: keyword }))),
            _ => parse_error!(self.current_token_data, "Unexpect token")
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        let prefix = self.current_token_data.clone();
        match prefix.token {
            Token::Minus | Token::Not => {
                self.next_token();
                let expression = self.parse_expression(SymbolPriority::Prefix)?;
                Ok(Expression::Prefix(Box::new(PrefixExpressionData{ prefix, right: expression })))
            },
            _ => parse_error!(self.current_token_data, "Unexpect token (expect Identifier)")
        }
    }

    fn get_prefix_expression(&self) -> Result<ParsePrefixExpression, ParseError> {
        match self.current_token_data.token {
            Token::Identifier(_) => Ok(Self::parse_identifier_expression),
            Token::Integer(_) => Ok(Self::parse_integer_expression),
            Token::Base | Token::This | Token::Null => Ok(Self::parse_keyword_expression),
            Token::Minus | Token::Not => Ok(Self::parse_prefix_expression),
            _ => parse_error!(self.current_token_data, "Unexpect token when parse expression")
        }
    }

    fn parse_infix_expression(&mut self, expression: Expression) -> Result<Expression, ParseError> {
        let token_data = self.current_token_data.clone();
        let precedence = self.get_current_precedence();

        self.next_token();

        let right = self.parse_expression(precedence)?;

        Ok(Expression::Infix(Box::new(InfixExpressionData { left: expression, infix: token_data, right })))
    }

    fn get_infix_expression(&self) -> Result<Option<ParseInfixExpression>, ParseError> {
        match self.current_token_data.token {

            Token::Assign | Token::PlusAssign | Token::MinusAssign | Token::StarAssign | Token::SlashAssign |
            Token::And | Token::Or | Token::Equal | Token::NotEqual | Token::Less | Token::Greater | Token::LessEqual | Token::GreaterEqual |
            Token::BitAnd | Token::BitOr | Token::Plus | Token::Minus | Token::Star | Token::Slash
            => Ok(Some(Self::parse_infix_expression)),
            _ => Ok(None)
        }
    }
}