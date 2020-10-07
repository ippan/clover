use crate::runtime::object::{Object, Slot};
use crate::runtime::state::State;

pub mod opcode;
pub mod object;
pub mod meta_table;
pub mod state;
pub mod assembly;

pub type NativeFunction = fn(&mut State, &mut Object, Vec<Slot>) -> Object;