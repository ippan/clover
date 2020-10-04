use clover::parser::Parser;

fn main() {
    let mut parser = Parser::new();

    let program = parser.parse("local a".to_string(), "main".to_string());

    println!("{:?}", program);
}
