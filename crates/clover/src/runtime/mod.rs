use crate::runtime::object::{Object, Slot};

pub mod opcode;
pub mod object;
pub mod meta_table;
pub mod state;
pub mod assembly;

pub type NativeFunction = fn(&mut Object, Vec<Slot>) -> Object;