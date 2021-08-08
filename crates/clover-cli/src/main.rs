use clover::{Clover, State};
use clover_std::clover_std_inject_to;
use std::env;
use clover::debug::CompileErrorList;

fn main() -> Result<(), CompileErrorList> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage:\n  clover [filename]");
        return Ok(());
    };

    let clover = Clover::new();

    let filename = &args[1];

    let program = clover.compile_file(filename.as_str())?;

    let mut state: State = program.into();

    clover_std_inject_to(&mut state);

    if let Err(error) = state.execute() {
        println!("{:?}", error);
    };

    Ok(())
}
