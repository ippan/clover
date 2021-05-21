use clover::frontend::parser::parse;
use std::fs::read_to_string;

fn main() {
    let filename = "examples/test.luck";

    let source = read_to_string(filename).unwrap();

    let result = parse(source.as_str(), filename);

    match result {
        Ok(program) => println!("{:?}", program),
        Err(compile_error_list) => println!("{:?}", compile_error_list)
    }
}
