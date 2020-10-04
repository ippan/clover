
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Invalid(String),
    Eof,
    None,

    Identifier(String),

    String(String),
    Integer(i64),
    Float(f64),

    True,
    False,
    Null,

    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,

    BitAnd,
    BitOr,

    Not,
    And,
    Or,

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

    End,
    Local,
    Function,
    Return,
    Class,
    Extends,
    New,
    Base,
    This,
    At,

    If,
    Else,
    While,

    Load
}

pub struct TokenData {
    pub token: Token,
    pub line: u16
}

impl TokenData {
    pub fn new(token: Token, line: u16) -> TokenData {
        TokenData { token, line }
    }
}