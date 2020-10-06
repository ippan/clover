use clover::parser::Parser;
use std::collections::HashMap;
use clover::compiler::Compiler;
use clover::runtime::assembly::Assembly;
use clover::runtime::state::State;
use std::fs::read_to_string;
use std::env;

fn main() {
    let arg: Vec<String> = env::args().collect();

    if arg.len() < 2 {
        println!("usage:\n\t./clover-cli [file]");
        return;
    };

    let filename = arg.get(1).unwrap();

    let source = read_to_string(filename).unwrap();

    let mut parser = Parser::new();

    let program_result = parser.parse(source, "main".to_string());

    match program_result {
        Ok(program) => {
            println!("{:?}", program);

            let mut compiler = Compiler::new();

            let assembly_result = compiler.compile(&program);

            match assembly_result {
                Ok(assembly) => {
                    println!("{:?}", assembly);

                    let mut state = State::new();

                    state.add_assembly(assembly);

                    let object = state.execute(0);

                    println!("{:?}", object);
                },
                Err(error) => {
                    println!("{:?}", error);
                }
            }
        },
        Err(error) => {
            println!("{:?}", error);
        }
    }


}