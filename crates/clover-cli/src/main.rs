use clover::backend::compiler::compile_file;
use std::collections::HashMap;

fn main() {
    let result = compile_file("examples/test.luck");

    match result {
        Ok(program) => {
            println!("{:?}", program);
        },
        Err(compile_error_list) => println!("{:?}", compile_error_list)
    }
}
