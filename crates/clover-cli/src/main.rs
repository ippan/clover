use clover::parser::Parser;

fn main() {
    let mut parser = Parser::new();

    let program = parser.parse("local a = -1 + 2 * 3".to_string(), "main".to_string());

    println!("{:?}", program);
}
