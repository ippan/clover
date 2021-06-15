use crate::intermediate::{Token, CompileErrorList, TokenValue, CompileError};
use crate::intermediate::ast::{Document, Definition, ModelDefinition, FunctionDefinition, Statement, ImplementDefinition, ApplyDefinition, LocalDefinition, IncludeDefinition, ReturnStatement, Expression, IdentifierExpression, IntegerExpression, FloatExpression, BooleanExpression, ThisExpression, NullExpression, PrefixExpression, IfExpression, InfixExpression, CallExpression, StringExpression, InstanceGetExpression, LocalStatement, ArrayExpression, IndexGetExpression, ForStatement, BreakStatement, RescueStatement};
use crate::frontend::lexer::lex;
use std::slice::Iter;
use std::mem::discriminant;
use crate::intermediate::TokenValue::Identifier;

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

struct ParserState<'a> {
    tokens: Iter<'a, Token>,
    last_token: Token,
    current_token: Token,
    peek_token: Token,
    errors: CompileErrorList
}

impl<'a> ParserState<'a> {
    fn next_token(&mut self) {
        self.last_token = self.current_token.clone();
        self.current_token = self.peek_token.clone();

        if let Some(token) = self.tokens.next() {
            self.peek_token = token.clone();
        } else {
            self.peek_token = Token::none();
        };
    }

    fn push_error(&mut self, token: &Token, message: String) {
        let error = CompileError {
            token: token.clone(),
            message
        };

        self.errors.push(error);
    }

    fn get_current_precedence(&self) -> SymbolPriority {
        match self.current_token.value {
            TokenValue::Assign | TokenValue::PlusAssign | TokenValue::MinusAssign | TokenValue::StarAssign | TokenValue::SlashAssign | TokenValue::PercentAssign => SymbolPriority::Assign,
            TokenValue::And | TokenValue::Or => SymbolPriority::Boolean,
            TokenValue::Equal | TokenValue::NotEqual => SymbolPriority::Equals,
            TokenValue::Less | TokenValue::Greater | TokenValue::LessEqual | TokenValue::GreaterEqual => SymbolPriority::LessGreater,
            TokenValue::Plus | TokenValue::Minus => SymbolPriority::Sum,
            TokenValue::Star | TokenValue::Slash | TokenValue::Percent | TokenValue::BitAnd | TokenValue::BitOr => SymbolPriority::Product,
            TokenValue::Dot | TokenValue::LeftBracket => SymbolPriority::InstanceGet,
            TokenValue::LeftParentheses => SymbolPriority::Call,
            _ => SymbolPriority::Lowest
        }
    }

    fn current_token_is_any_of(&self, token_values: &[TokenValue]) -> bool {
        for token_value in token_values {
            if discriminant(&self.current_token.value) == discriminant(token_value) {
                return true;
            }
        };

        false
    }

    fn skip_until(&mut self, token_values: &[TokenValue]) {
        while !self.current_token_is_any_of(token_values) {
            self.next_token();
        };
    }

    fn expect_token(&mut self, token_value: TokenValue) -> bool {
        if discriminant(&self.current_token.value) == discriminant(&token_value) {
            return true;
        };

        self.push_error(&self.current_token.clone(), format!("Unexpect token [{:?}] (expert [{:?}])", self.current_token.value, token_value));

        self.skip_until(&[ token_value.clone(), TokenValue::Eof, TokenValue::None ]);

        discriminant(&self.current_token.value) == discriminant(&token_value)
    }

    fn expect_and_pop_token(&mut self, token_value: TokenValue) -> bool {
        if self.expect_token(token_value) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn parse_identifier_expression(&mut self) -> Option<Expression> {
        if !self.expect_token(TokenValue::Identifier("".to_string())) {
            return None;
        };

        let token = self.current_token.clone();
        self.next_token();

        Some(Expression::Identifier(IdentifierExpression { token }))
    }

    fn parse_integer_expression(&mut self) -> Option<Expression> {
        if !self.expect_token(TokenValue::Integer(0)) {
            return None;
        };

        let token = self.current_token.clone();
        self.next_token();

        Some(Expression::Integer(IntegerExpression { token }))
    }

    fn parse_float_expression(&mut self) -> Option<Expression> {
        if !self.expect_token(TokenValue::Float(0.0)) {
            return None;
        };

        let token = self.current_token.clone();
        self.next_token();

        Some(Expression::Float(FloatExpression { token }))
    }

    fn parse_string_expression(&mut self) -> Option<Expression> {
        if !self.expect_token(TokenValue::String("".to_string())) {
            return None;
        };

        let token = self.current_token.clone();
        self.next_token();

        Some(Expression::String(StringExpression { token }))
    }

    fn parse_boolean_expression(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();
        self.next_token();
        match token.value {
            TokenValue::True | TokenValue::False => Some(Expression::Boolean(BooleanExpression { token })),
            _ => None
        }
    }

    fn parse_keyword_expression(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();
        self.next_token();
        match token.value {
            TokenValue::This => Some(Expression::This(ThisExpression { token })),
            TokenValue::Null => Some(Expression::Null(NullExpression { token })),
            _ => None
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();
        self.next_token();

        match token.value {
            TokenValue::Minus | TokenValue::Not => {
                if let Some(expression) = self.parse_expression(SymbolPriority::Prefix) {
                    Some(Expression::Prefix(PrefixExpression {
                        prefix: token,
                        right: Box::new(expression)
                    }))
                } else {
                    None
                }
            },
            _ => None
        }

    }

    fn parse_group_expression(&mut self) -> Option<Expression> {
        if !self.expect_and_pop_token(TokenValue::LeftParentheses) {
            return None;
        };

        if let Some(expression) = self.parse_expression(SymbolPriority::Lowest) {
            if !self.expect_and_pop_token(TokenValue::RightParentheses) {
                return None;
            }

            Some(expression)
        } else {
            None
        }
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        if !self.current_token_is_any_of(&[ TokenValue::If, TokenValue::ElseIf ]) {
            return None;
        }
        self.next_token();

        let mut expect_end = true;

        if let Some(condition) = self.parse_expression(SymbolPriority::Lowest) {
            let true_part = self.parse_body(&[ TokenValue::End, TokenValue::Else, TokenValue::ElseIf, TokenValue::Eof ]);

            let mut false_part = None;

            if self.current_token.value == TokenValue::Else {
                self.next_token();
                false_part = Some(self.parse_body(&[ TokenValue::End, TokenValue::Eof ]))
            } else if self.current_token.value == TokenValue::ElseIf {
                expect_end = false;

                if let Some(expression) = self.parse_if_expression() {
                    let statements = vec![ Statement::Expression(expression) ];
                    false_part = Some(statements);
                } else {
                    return None;
                };
            }

            if expect_end && !self.expect_and_pop_token(TokenValue::End) {
                return None;
            };

            Some(Expression::If(IfExpression {
                condition: Box::new(condition),
                true_part,
                false_part
            }))
        } else {
            None
        }
    }

    fn parse_comma_expressions(&mut self, end_tokens: &[ TokenValue ]) -> Option<Vec<Expression>> {
        let mut values = Vec::new();

        let mut last_comma = None;

        while !self.current_token_is_any_of(end_tokens) {
            last_comma = None;

            if let Some(parameter) = self.parse_expression(SymbolPriority::Lowest) {
                values.push(parameter);

                if self.current_token.value == TokenValue::Comma {
                    last_comma = Some(self.current_token.clone());
                    self.next_token();
                }

            } else {
                return None;
            }
        }

        if let Some(token) = last_comma {
            self.push_error(&token, "Unexpect token".to_string());
        };

        Some(values)
    }

    fn parse_array_expression(&mut self) -> Option<Expression> {
        self.expect_token(TokenValue::LeftBracket);

        let token = self.current_token.clone();
        self.next_token();

        if let Some(values) = self.parse_comma_expressions(&[ TokenValue::RightBracket, TokenValue::Eof ]) {
            self.expect_and_pop_token(TokenValue::RightBracket);

            Some(Expression::Array(ArrayExpression {
                token,
                values
            }))
        } else {
            None
        }
    }

    fn parse_start_expression(&mut self) -> Option<Expression> {
        match self.current_token.value {
            TokenValue::Identifier(_) => self.parse_identifier_expression(),
            TokenValue::Integer(_) => self.parse_integer_expression(),
            TokenValue::Float(_) => self.parse_float_expression(),
            TokenValue::String(_) => self.parse_string_expression(),
            TokenValue::True | TokenValue::False => self.parse_boolean_expression(),
            TokenValue::This | TokenValue::Null => self.parse_keyword_expression(),
            TokenValue::Minus | TokenValue::Not => self.parse_prefix_expression(),
            TokenValue::LeftParentheses => self.parse_group_expression(),
            TokenValue::LeftBracket => self.parse_array_expression(),
            TokenValue::If => self.parse_if_expression(),
            _ => {
                self.push_error(&self.current_token.clone(), "Unexpect token when parse expression".to_string());
                None
            }
        }
    }

    fn parse_call_expression(&mut self, expression: Expression) -> Option<Expression> {
        self.expect_token(TokenValue::LeftParentheses);

        let token = self.current_token.clone();
        self.next_token();

        if let Some(parameters) = self.parse_comma_expressions(&[ TokenValue::RightParentheses, TokenValue::Eof ]) {
            self.expect_and_pop_token(TokenValue::RightParentheses);

            Some(Expression::Call(CallExpression {
                token,
                function: Box::new(expression),
                parameters
            }))
        } else {
            None
        }
    }

    fn parse_instance_get_expression(&mut self, expression: Expression) -> Option<Expression> {
        let token = self.current_token.clone();
        self.next_token();

        match token.value.clone() {
            TokenValue::Dot => {
                let identifier_token = self.current_token.clone();
                if let TokenValue::Identifier(identifier) = identifier_token.value.clone() {
                    self.next_token();
                    let index = Expression::String(StringExpression { token: Token::new(TokenValue::String(identifier), identifier_token.position) });

                    Some(Expression::InstanceGet(InstanceGetExpression {
                        token,
                        instance: Box::new(expression),
                        index: Box::new(index)
                    }))

                } else {
                    self.push_error(&identifier_token, "Unexpect Token".to_string());
                    None
                }
            },
            TokenValue::LeftBracket => {
                if let Some(index) = self.parse_expression(SymbolPriority::Lowest) {
                    if !self.expect_and_pop_token(TokenValue::RightBracket) {
                        return None;
                    };

                    Some(Expression::IndexGet(IndexGetExpression {
                        token,
                        instance: Box::new(expression),
                        index: Box::new(index)
                    }))
                } else {
                    None
                }
            }
            _ => {
                self.push_error(&token, "Unexpect Token".to_string());
                None
            }
        }
    }

    fn parse_infix_expression(&mut self, expression: Expression) -> Option<Expression> {
        // if '-' or '(' or '[' is the first token at line, it's not a infix expression
        match self.current_token.value {
            TokenValue::Minus | TokenValue::LeftParentheses | TokenValue::LeftBracket => {
                if self.current_token.position.line > self.last_token.position.line {
                    return None;
                };
            },
            _ => {}
        }

        match self.current_token.value {
            TokenValue::Assign | TokenValue::PlusAssign | TokenValue::MinusAssign | TokenValue::StarAssign | TokenValue::SlashAssign | TokenValue::PercentAssign |
            TokenValue::And | TokenValue::Or | TokenValue::Equal | TokenValue::NotEqual | TokenValue::Less | TokenValue::Greater | TokenValue::LessEqual | TokenValue::GreaterEqual |
            TokenValue::BitAnd | TokenValue::BitOr | TokenValue::Plus | TokenValue::Minus | TokenValue::Star | TokenValue::Slash | TokenValue::Percent
            => {
                let token = self.current_token.clone();
                let precedence = self.get_current_precedence();

                self.next_token();

                if let Some(right) = self.parse_expression(precedence) {
                    Some(Expression::Infix(InfixExpression {
                        left: Box::new(expression),
                        infix: token,
                        right: Box::new(right)
                    }))
                } else {
                    None
                }
            }
            TokenValue::LeftParentheses => self.parse_call_expression(expression),
            TokenValue::Dot | TokenValue::LeftBracket => self.parse_instance_get_expression(expression),
            _ => None
        }
    }

    fn parse_expression(&mut self, precedence: SymbolPriority) -> Option<Expression> {
        if let Some(start_expression) = self.parse_start_expression() {
            let mut left_expression = start_expression;

            while self.current_token.value != TokenValue::Eof && precedence < self.get_current_precedence() {
                let expression = left_expression.clone();

                if let Some(new_expression) = self.parse_infix_expression(expression) {
                    left_expression = new_expression;
                } else {
                    return Some(left_expression);
                }
            }

            Some(left_expression)
        } else {
            None
        }
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let return_statement = ReturnStatement {
            token: self.current_token.clone()
        };
        self.next_token();

        Some(Statement::Return(return_statement))
    }

    fn parse_break_statement(&mut self) -> Option<Statement> {
        let break_statement = BreakStatement {
            token: self.current_token.clone()
        };
        self.next_token();

        Some(Statement::Break(break_statement))
    }

    fn parse_rescue_statement(&mut self) -> Option<Statement> {
        let rescue_statement = RescueStatement {
            token: self.current_token.clone()
        };
        self.next_token();

        Some(Statement::Rescue(rescue_statement))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        if let Some(expression) = self.parse_expression(SymbolPriority::Lowest) {
            Some(Statement::Expression(expression))
        } else {
            None
        }
    }

    fn parse_local_statement(&mut self) -> Option<Statement> {
        self.expect_and_pop_token(TokenValue::Local);

        let mut variables = Vec::new();
        let mut values = Vec::new();

        let mut last_is_comma = true;

        while last_is_comma {
            if !self.expect_token(TokenValue::Identifier("".to_string())) {
                return None;
            };
            variables.push(self.current_token.clone());
            self.next_token();

            if self.current_token.value == TokenValue::Assign {
                self.next_token();
                values.push(self.parse_expression(SymbolPriority::Lowest));
            } else {
                values.push(None)
            }

            last_is_comma = self.current_token.value == TokenValue::Comma;

            if last_is_comma {
                self.next_token();
            };
        }

        Some(Statement::Local(LocalStatement {
            variables,
            values
        }))
    }

    fn parse_for_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();

        if !self.expect_and_pop_token(TokenValue::For) {
            return None;
        };

        if !self.expect_token(TokenValue::Identifier("".to_string())) {
            return None;
        };

        let identifier = self.current_token.clone();
        self.next_token();

        if !self.expect_and_pop_token(TokenValue::In) {
            return None;
        };

        let expression = self.parse_expression(SymbolPriority::Lowest);

        if expression.is_none() {
            return None;
        };

        let statements = self.parse_body(&[ TokenValue::Eof, TokenValue::End ]);

        if !self.expect_and_pop_token(TokenValue::End) {
            return None;
        };

        Some(Statement::For(ForStatement{
            token,
            identifier,
            enumerable: expression.unwrap(),
            statements
        }))
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.value {
            TokenValue::Local => self.parse_local_statement(),
            TokenValue::Return => self.parse_return_statement(),
            TokenValue::Break => self.parse_break_statement(),
            TokenValue::Rescue => self.parse_rescue_statement(),
            TokenValue::For => self.parse_for_statement(),
            _ => self.parse_expression_statement()
        }
    }

    fn parse_model_definition(&mut self) -> Option<Definition> {
        // skip model token
        self.next_token();

        // model name
        if !self.expect_token(TokenValue::Identifier("".to_string())) {
            return None;
        }
        let name = self.current_token.clone();
        self.next_token();

        let mut properties = Vec::new();

        // TODO : add annotation

        while let TokenValue::Identifier(_) = self.current_token.value {
            properties.push(self.current_token.clone());
            self.next_token();
        };

        if !self.expect_and_pop_token(TokenValue::End) {
            return None;
        }

        Some(Definition::Model(ModelDefinition {
            name,
            properties
        }))
    }

    fn parse_body(&mut self, terminators: &[TokenValue]) -> Vec<Statement> {
        let mut statements = Vec::new();

        while !self.current_token_is_any_of(terminators) {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            };
        };

        statements
    }

    fn parse_function_definition(&mut self) -> Option<Definition> {
        // skip function token
        self.next_token();

        // function name
        if !self.expect_token(TokenValue::Identifier("".to_string())) {
            return None;
        }
        let name = self.current_token.clone();
        self.next_token();

        if !self.expect_and_pop_token(TokenValue::LeftParentheses) {
            return None;
        };

        // parse parameters
        let mut parameters = Vec::new();
        let mut last_comma = None;

        while self.current_token.value != TokenValue::RightParentheses {
            last_comma = None;

            if parameters.len() == 0 {
                if !self.current_token_is_any_of(&[ TokenValue::Identifier("".to_string()), TokenValue::This ]) {
                    self.errors.push_error(&self.current_token.clone(), "Unexpect token");
                    self.skip_until(&[ TokenValue::Identifier("".to_string()), TokenValue::This, TokenValue::Eof ]);
                };
            } else if !self.expect_token(TokenValue::Identifier("".to_string())) {
                return None;
            };

            parameters.push(self.current_token.clone());
            self.next_token();

            if self.current_token.value == TokenValue::Comma {
                last_comma = Some(self.current_token.clone());
                self.next_token();
            }
        }

        if let Some(comma) = last_comma {
            self.push_error(&comma.clone(), format!("Unexpect token [{:?}]", comma.value));
        };

        if !self.expect_and_pop_token(TokenValue::RightParentheses) {
            return None;
        };

        // parse function body
        let body = self.parse_body(&[ TokenValue::End, TokenValue::Eof ]);

        if !self.expect_and_pop_token(TokenValue::End) {
            return None;
        };

        Some(Definition::Function(FunctionDefinition {
            name,
            parameters,
            body
        }))
    }

    fn parse_implement_definition(&mut self) -> Option<Definition> {
        self.next_token();

        if !self.expect_token(TokenValue::Identifier("".to_string())) {
            return None;
        }
        let model_name = self.current_token.clone();
        self.next_token();

        let mut functions = Vec::new();

        while self.current_token.value == TokenValue::Function {
            if let Some(definition) = self.parse_function_definition() {
                if let Definition::Function(function_definition) = definition {
                    functions.push(function_definition);
                };
            };
        };

        if !self.expect_and_pop_token(TokenValue::End) {
            return None;
        };

        Some(Definition::Implement(ImplementDefinition {
            model_name,
            functions
        }))
    }

    fn parse_apply_definition(&mut self) -> Option<Definition> {
        self.next_token();

        if !self.expect_token(TokenValue::Identifier("".to_string())) {
            return None;
        }
        let source_model = self.current_token.clone();
        self.next_token();

        if !self.expect_and_pop_token(TokenValue::To) {
            return None;
        };

        if !self.expect_token(TokenValue::Identifier("".to_string())) {
            return None;
        }
        let target_model = self.current_token.clone();
        self.next_token();

        Some(Definition::Apply(ApplyDefinition {
            source_model,
            target_model
        }))
    }

    fn parse_local_definition(&mut self) -> Option<Definition> {
        self.expect_and_pop_token(TokenValue::Local);

        let mut variables = Vec::new();
        let mut values = Vec::new();

        let mut last_is_comma = true;

        while last_is_comma {
            if !self.expect_token(TokenValue::Identifier("".to_string())) {
                return None;
            };
            variables.push(self.current_token.clone());
            self.next_token();

            if self.current_token.value == TokenValue::Assign {
                self.next_token();

                if self.current_token_is_any_of(&[ TokenValue::Null, TokenValue::True, TokenValue::False, TokenValue::Integer(0), TokenValue::Float(0.0), TokenValue::String("".to_string()) ]) {
                    values.push(Some(self.current_token.clone()));
                } else {
                    self.push_error(&self.current_token.clone(), "can use constant value only".to_string());
                    values.push(None);
                }

                self.next_token();
            } else {
                values.push(None);
            };

            last_is_comma = self.current_token.value == TokenValue::Comma;

            if last_is_comma {
                self.next_token();
            };
        }

        Some(Definition::Local(LocalDefinition {
            variables,
            values
        }))
    }

    fn parse_include_definition(&mut self) -> Option<Definition> {
        self.next_token();

        let mut models = Vec::new();
        let mut aliases = Vec::new();

        let mut last_comma = None;

        while models.len() == 0 || self.current_token_is_any_of(&[ Identifier("".to_string()) ]) {
            last_comma = None;

            if !self.expect_token(TokenValue::Identifier("".to_string())) {
                return None;
            };
            models.push(self.current_token.clone());
            self.next_token();

            if self.current_token.value == TokenValue::As {
                self.next_token();
                if !self.expect_token(TokenValue::Identifier("".to_string())) {
                    return None;
                };
                aliases.push(self.current_token.clone());
                self.next_token();
            } else {
                aliases.push(models.last().unwrap().clone());
            }

            if self.current_token.value == TokenValue::Comma {
                last_comma = Some(self.current_token.clone());
                self.next_token();
            }
        };

        if let Some(comma) = last_comma {
            self.push_error(&comma.clone(), format!("Unexpect token [{:?}]", comma.value));
        };

        if !self.expect_and_pop_token(TokenValue::From) {
            return None;
        };

        if !self.expect_token(TokenValue::String("".to_string())) {
            return None;
        }
        let filename = self.current_token.clone();
        self.next_token();

        Some(Definition::Include(IncludeDefinition {
            public_names: models,
            aliases,
            filename
        }))
    }

    fn parse_public_definition(&mut self) -> Option<Definition> {
        self.expect_and_pop_token(TokenValue::Public);

        match self.current_token.value {
            TokenValue::Model => {
                if let Some(Definition::Model(model_definition)) = self.parse_model_definition() {
                    Some(Definition::PublicModel(model_definition))
                } else {
                    None
                }
            },
            TokenValue::Function => {
                if let Some(Definition::Function(function_definition)) = self.parse_function_definition() {
                    Some(Definition::PublicFunction(function_definition))
                } else {
                    None
                }
            },
            _ => {
                self.push_error(&self.current_token.clone(), "Unexpect token".to_string());
                None
            }
        }
    }

    fn parse_definition(&mut self) -> Option<Definition> {
        match self.current_token.value {
            TokenValue::Model => self.parse_model_definition(),
            TokenValue::Function => self.parse_function_definition(),
            TokenValue::Implement => self.parse_implement_definition(),
            TokenValue::Apply => self.parse_apply_definition(),
            TokenValue::Include => self.parse_include_definition(),
            TokenValue::Local => self.parse_local_definition(),
            TokenValue::Public => self.parse_public_definition(),
            _ => {
                self.push_error(&self.current_token.clone(), format!("Unexcpet token [{:?}]", self.current_token.clone()));
                self.skip_until(&[ TokenValue::Include, TokenValue::Public, TokenValue::Model, TokenValue::Implement, TokenValue::Apply, TokenValue::Local, TokenValue::Function ]);
                None
            }
        }
    }

    fn parse_definitions(&mut self) -> Vec<Definition> {
        let mut definitions = Vec::new();

        let mut include_definitions_ended = false;

        while self.current_token.value != TokenValue::Eof && self.current_token.value != TokenValue::None {
            let current_token = self.current_token.clone();

            if let Some(definition) = self.parse_definition() {

                if let Definition::Include(_) = definition {
                    if include_definitions_ended {
                        self.push_error(&current_token, "include definition must at the top of files".to_string());
                    };
                } else {
                    include_definitions_ended = true;
                };

                definitions.push(definition);
            };
        };

        definitions
    }

    fn parse_document(&mut self, filename: String) -> Document {
        let mut document = Document {
            definitions: self.parse_definitions(),
            filename
        };

        document.normalize_include_paths();

        document
    }
}

pub fn parse(source: &str, filename: &str) -> Result<Document, CompileErrorList> {
    let token_list = lex(source)?;

    let mut state = ParserState {
        tokens: token_list.iter(),
        last_token: Token::none(),
        current_token: Token::none(),
        peek_token: Token::none(),
        errors: CompileErrorList::new(filename)
    };

    state.next_token();
    state.next_token();

    let document = state.parse_document(filename.to_string());

    if state.errors.is_empty() {
        Ok(document)
    } else {
        Err(state.errors)
    }
}