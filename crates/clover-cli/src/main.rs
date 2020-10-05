use clover::parser::Parser;

fn main() {
    let mut parser = Parser::new();

    let program = parser.parse("local a = -1 + 2 * (3 + 1)\n if (a == 1)\n a += 1\nend\nlocal hello = function(a, b, c = 1) end".to_string(), "main".to_string());

    println!("{:?}", program);
}
