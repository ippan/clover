use crate::runtime::object::Object;
use crate::runtime::program::{Program, RuntimeError};
use crate::runtime::state::State;
use crate::backend::compiler::{compile_file, DefaultFileLoader};
use crate::intermediate::CompileErrorList;

pub mod object;
pub mod program;
pub mod opcode;
pub mod assembly_information;
pub mod state;

mod operation;
mod object_property;
mod iterator;


pub fn run(program: Program) -> Result<Object, RuntimeError> {
    let mut state = State::new(program);

    state.execute()
}

pub fn create_state_by_filename(filename: &str) -> Result<State, CompileErrorList> {
    let file_loader = DefaultFileLoader::new();

    let program = compile_file(filename, &file_loader)?;

    Ok(State::new(program))
}