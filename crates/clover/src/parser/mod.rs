mod lexer;

use crate::ast::token::{TokenData, Token};
use crate::ast::{Program, Statement, LocalStatementData, Expression, ExpressionStatementData, IdentifierExpressionData, InfixExpressionData, IntegerLiteralExpressionData, BaseLiteralExpressionData, NullLiteralExpressionData, ThisLiteralExpressionData, PrefixExpressionData, FloatLiteralExpressionData, BooleanLiteralExpressionData, Codes, IfExpressionData, FunctionExpressionData, ClassExpressionData};
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

    fn current_token_is_any_of(&self, tokens: &[Token]) -> bool {
        for token in tokens {
            if token.clone() == self.current_token_data.token {
                return true;
            }
        };

        false
    }

    fn expect_token(&self, token: &Token) -> Result<(), ParseError> {
        if std::mem::discriminant(&self.current_token_data.token) == std::mem::discriminant(token) {
            Ok(())
        } else {
            parse_error!(self.current_token_data, format!("Unexpect token [{:?}] (expect [{:?}])", self.current_token_data.token, token))
        }
    }

    fn parse_codes(&mut self, stop_tokens: &[Token]) -> Result<Codes, ParseError> {
        let mut codes = Codes::new();

        while self.current_token_data.token != Token::Eof && !self.current_token_is_any_of(stop_tokens) {
            codes.push(self.parse_statement()?);
        }

        Ok(codes)
    }

    fn parse_program(&mut self, filename: String) -> Result<Program, ParseError> {
        let mut program = Program::new(filename);

        program.codes = self.parse_codes(&[])?;

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

        self.expect_token(&Token::Identifier("".to_string()))?;

        self.next_token();

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
        self.expect_token(&Token::Identifier("".to_string()))?;

        let identifier = self.current_token_data.clone();
        self.next_token();
        Ok(Expression::Identifier(Box::new(IdentifierExpressionData { data: identifier })))
    }

    fn parse_integer_expression(&mut self) -> Result<Expression, ParseError> {
        self.expect_token(&Token::Integer(0))?;
        let integer = self.current_token_data.clone();
        self.next_token();
        Ok(Expression::IntegerLiteral(Box::new(IntegerLiteralExpressionData { data: integer })))
    }

    fn parse_float_expression(&mut self) -> Result<Expression, ParseError> {
        self.expect_token(&Token::Float(0.0))?;
        let float = self.current_token_data.clone();
        self.next_token();
        Ok(Expression::FloatLiteral(Box::new(FloatLiteralExpressionData { data: float })))
    }

    fn parse_boolean_expression(&mut self) -> Result<Expression, ParseError> {
        let boolean = self.current_token_data.clone();

        match boolean.token {
            Token::True | Token::False => Ok(Expression::BooleanLiteral(Box::new(BooleanLiteralExpressionData { data: boolean }))),
            _ => parse_error!(self.current_token_data, "Unexpect token (expect boolean)")
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

    fn parse_group_expression(&mut self) -> Result<Expression, ParseError> {
        self.expect_token(&Token::LeftParentheses)?;
        self.next_token();

        let expression = self.parse_expression(SymbolPriority::Lowest)?;

        self.expect_token(&Token::RightParentheses)?;
        self.next_token();

        Ok(expression)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
        self.next_token();

        let condition = self.parse_group_expression()?;
        let true_part = self.parse_codes(&[ Token::Else, Token::End ])?;
        let mut false_part = None;

        if self.current_token_data.token == Token::Else {
            false_part = Some(self.parse_codes(&[ Token::End ])?);
        };

        self.expect_token(&Token::End)?;
        self.next_token();

        Ok(Expression::If(Box::new(IfExpressionData { condition, true_part, false_part })))
    }

    fn parse_key_values(&mut self, terminators: &[Token], assign: Token, separator: Token, must_have_value: bool) -> Result<Vec<LocalStatementData>, ParseError> {
        let mut key_values = Vec::new();
        let mut last_is_seperator = false;

        while !self.current_token_is_any_of(terminators) && self.current_token_data.token != Token::Eof {

            // last is separator?
            if key_values.len() > 0 && !last_is_seperator && separator.clone() != Token::None {
                self.expect_token(&separator)?;
            }

            self.expect_token(&Token::Identifier("".to_string()))?;

            let identifier = self.current_token_data.clone();
            self.next_token();

            if must_have_value {
                self.expect_token(&assign)?;
            }

            let mut expression = None;

            if self.current_token_data.token == assign {
                self.next_token();

                expression = Some(self.parse_expression(SymbolPriority::Lowest)?);
            }

            key_values.push(LocalStatementData { identifier, expression });

            if separator == Token::None {
                continue;
            };

            last_is_seperator = if self.current_token_data.token == separator {
                self.next_token();
                true
            } else {
                false
            };
        };

        if last_is_seperator {
            parse_error!(self.current_token_data, format!("Unexpect token [{:?}]", separator))
        } else {
            Ok(key_values)
        }
    }

    fn parse_function_expression(&mut self) -> Result<Expression, ParseError> {
        // skip function token
        self.next_token();

        // check and skip ( token
        self.expect_token(&Token::LeftParentheses)?;
        self.next_token();

        let parameters = self.parse_key_values(&[ Token::RightParentheses ], Token::Assign, Token::Comma, false)?;

        // check and skip ) token
        self.expect_token(&Token::RightParentheses)?;
        self.next_token();

        let body = self.parse_codes(&[ Token::End ])?;

        // check and skip end token
        self.expect_token(&Token::End)?;
        self.next_token();

        Ok(Expression::Function(Box::new(FunctionExpressionData { parameters, body })))
    }

    fn parse_class_expression(&mut self) -> Result<Expression, ParseError> {
        // skip class token
        self.next_token();

        let mut super_class = None;

        if self.current_token_data.token == Token::Extends {
            // skip extends token
            self.next_token();

            super_class = Some(self.parse_expression(SymbolPriority::Lowest)?);
        }

        let members = self.parse_key_values(&[ Token::End ], Token::Assign, Token::None, true)?;

        Ok(Expression::Class(Box::new(ClassExpressionData { super_class, members })))
    }

    fn get_prefix_expression(&self) -> Result<ParsePrefixExpression, ParseError> {
        match self.current_token_data.token {
            Token::Identifier(_) => Ok(Self::parse_identifier_expression),
            Token::Integer(_) => Ok(Self::parse_integer_expression),
            Token::Float(_) => Ok(Self::parse_float_expression),
            Token::True | Token::False => Ok(Self::parse_boolean_expression),
            Token::Base | Token::This | Token::Null => Ok(Self::parse_keyword_expression),
            Token::Minus | Token::Not => Ok(Self::parse_prefix_expression),
            Token::LeftParentheses => Ok(Self::parse_group_expression),
            Token::If => Ok(Self::parse_if_expression),
            Token::Function => Ok(Self::parse_function_expression),
            Token::Class => Ok(Self::parse_class_expression),
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