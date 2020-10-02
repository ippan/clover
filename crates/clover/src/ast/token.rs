
pub enum Token {
    Invalid,
    Eof,
    None,

    Identifier,
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

}

pub struct TokenData {
    pub token: Token,
    pub line: u16
}