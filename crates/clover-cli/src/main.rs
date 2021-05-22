use clover::frontend::parser::parse;
use std::fs::read_to_string;
use clover::backend::compiler::{compile_document, compile_file};
use std::collections::HashMap;
use clover::runtime::program::Assemblies;

fn main() {
    let result = compile_file("examples/test.luck");

    match result {
        Ok(program) => {
            println!("{:?}", program);
        },
        Err(compile_error_list) => println!("{:?}", compile_error_list)
    }
}
