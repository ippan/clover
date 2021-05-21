use clover::frontend::parser::parse;
use std::fs::read_to_string;
use clover::backend::compiler::compile_document;
use std::collections::HashMap;
use clover::runtime::program::Assemblies;

fn main() {
    let filename = "examples/test.luck";

    let source = read_to_string(filename).unwrap();

    let result = parse(source.as_str(), filename);

    match result {
        Ok(document) => {
            println!("{:?}", document.get_dependencies());
            let result = compile_document(&document, 0, &Assemblies::new());
            println!("{:?}", result);
        },
        Err(compile_error_list) => println!("{:?}", compile_error_list)
    }
}
