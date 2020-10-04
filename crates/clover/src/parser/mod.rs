use crate::parser::lexer::Lexer;

pub mod lexer;

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
    lexer: Option<Lexer>



}