use clover::parser::Parser;
use std::mem::size_of;
use std::collections::HashMap;

fn main() {
    let mut parser = Parser::new();

    let program = parser.parse("1 + 2 * 3\n if (true) a += 1 end".to_string(), "main".to_string());

    println!("{:?}", program);
}
