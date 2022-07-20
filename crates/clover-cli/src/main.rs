use std::error::Error;
use std::fs::File;
use std::process::exit;
use clover::{Clover, Program, State};
use clover_std::clover_std_inject_to;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// compile input file
    #[clap(short, long, action)]
    compile: bool,

    /// specify the output filename when compile
    #[clap(short, long = "output", value_parser)]
    output_filename: Option<String>,

    /// source filename to run/compile
    #[clap(value_parser)]
    pub filename: String,
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let clover = Clover::new();

    let filename: String = args.filename.clone();

    let program = if filename.ends_with(".lucky") {
        if args.compile {
            // can not compile a lucky file
            println!("can not compile lucky file");
            exit(-1);
        }

        let mut file = File::open(filename)?;
        Program::deserialize(&mut file)?
    } else {
        clover.compile_file(filename.as_str())?
    };

    if args.compile {
        let output_filename = args.output_filename.unwrap_or(if args.filename.ends_with("luck") { args.filename + "y" } else { args.filename + ".lucky" });

        let mut file = File::create(output_filename)?;

        program.serialize(&mut file)?;

    } else {
        let mut state: State = program.into();

        clover_std_inject_to(&mut state);

        state.execute()?;
    }

    Ok(())
}
