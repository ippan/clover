pub mod ast;

#[derive(Clone, PartialEq, Debug)]
pub enum TokenValue {
    Invalid(String),
    Eof,

    Identifier(String),

    String(String),
    Integer(i64),
    Float(f64),

    True,
    False,
    Null,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,

    BitAnd,
    BitOr,

    And,
    Or,
    Not,

    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    LeftParentheses,
    RightParentheses,
    LeftBracket,
    RightBracket,

    Comma,
    Colon,
    Dot,

    Include,
    From,
    Model,
    Function,
    End,
    Implement,
    Local,
    Apply,
    To,
    Return,
    Public,
    As,
    This,
    If,
    Else,
    While,
    For,
    In,

    None
}

impl TokenValue {
    pub fn to_string(&self) -> String {
        match self {
            TokenValue::Identifier(value) => value.clone(),
            TokenValue::String(value) => value.clone(),
            TokenValue::Integer(value) => value.to_string(),
            TokenValue::Float(value) => value.to_string(),
            TokenValue::Null => "Null".to_string(),
            TokenValue::True => "true".to_string(),
            TokenValue::False => "false".to_string(),
            TokenValue::This => "this".to_string(),

            _ => format!("{:?}", self)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub line: u16,
    pub column: u16
}

impl Position {
    pub fn new(line: u16, column: u16) -> Position {
        Position { line, column }
    }

    pub fn none() -> Position {
        Position { line: 0, column: 0 }
    }
}

pub type Positions = Vec<Position>;

#[derive(Clone, Debug)]
pub struct Token {
    pub value: TokenValue,
    pub position: Position
}

impl Token {
    pub fn new(value: TokenValue, position: Position) -> Token {
        Token { value, position }
    }

    pub fn none() -> Token {
        Token { value: TokenValue::None, position: Position::none() }
    }
}

pub type TokenList = Vec<Token>;

#[derive(Clone, Debug)]
pub struct CompileError {
    pub token: Token,
    pub message: String
}

#[derive(Clone, Debug)]
pub struct CompileErrorList {
    pub filename: String,
    pub errors: Vec<CompileError>
}

impl CompileErrorList {
    pub fn new(filename: &str) -> CompileErrorList {
        CompileErrorList {
            filename: filename.to_string(),
            errors: Vec::new()
        }
    }

    pub fn push(&mut self, error: CompileError) {
        self.errors.push(error);
    }

    pub fn push_error(&mut self, token: &Token, message: &str) {
        self.push(CompileError {
            token: token.clone(),
            message: message.to_string()
        });
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}