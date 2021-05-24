use clover::runtime::create_state_by_filename;
use clover_std::clover_inject_to;

fn main() {
    let result = create_state_by_filename("examples/test.luck");

    match result {
        Ok(mut state) => {
            println!("{:?}", &state.program);
            clover_inject_to(&mut state);

            let result = state.execute();

            println!("{:?}", result);
        },
        Err(compile_error_list) => println!("{:?}", compile_error_list)
    }
}
