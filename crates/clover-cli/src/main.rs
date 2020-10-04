use clover::parser::lexer::Lexer;
use clover::ast::token::Token;

fn main() {
    let mut lexer = Lexer::new("local a = 1\nlocal b = a + 1");

    let mut token_data = lexer.lex();

    while token_data.token != Token::Eof {
        println!("{:?}", token_data.token);
        token_data = lexer.lex();
    }
}
