use clover::Clover;
use clover_std::clover_std_inject_to;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage:\n  clover [filename]");
        return;
    };

    let clover = Clover::new();

    let filename = &args[1];

    let result = clover.create_state_by_filename(filename.as_str());

    match result {
        Ok(mut state) => {
            clover_std_inject_to(&mut state);

            if let Err(error) = state.execute() {
                println!("{:?}", error);
            };
        },
        Err(compile_error_list) => println!("{:?}", compile_error_list)
    }
}
