use clover::parser::Parser;
use std::collections::HashMap;
use clover::compiler::Compiler;
use clover::runtime::assembly::Assembly;
use clover::runtime::state::State;

fn main() {
    let mut parser = Parser::new();

    let program_result = parser.parse("local a = 1\nlocal b\nlocal c = true\nreturn 10.2".to_string(), "main".to_string());

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
