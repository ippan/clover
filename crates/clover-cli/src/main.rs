use clover::backend::compiler::compile_file;
use clover::runtime::run;

fn main() {
    let result = compile_file("examples/test.luck");

    match result {
        Ok(program) => {
            println!("{:?}", program);
            let result = run(program);

            println!("{:?}", result);
        },
        Err(compile_error_list) => println!("{:?}", compile_error_list)
    }
}
