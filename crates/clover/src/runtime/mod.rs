use crate::runtime::object::Object;
use crate::runtime::program::{Program, RuntimeError};
use crate::runtime::state::State;

pub mod object;
pub mod program;
pub mod opcode;
pub mod assembly_information;
pub mod state;


pub fn run(program: Program) -> Result<Object, RuntimeError> {
    let mut state = State::new(program);

    state.execute()
}