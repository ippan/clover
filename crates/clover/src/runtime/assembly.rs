use crate::runtime::object::Object;
use crate::runtime::opcode::Instruction;

#[derive(Debug, Copy, Clone)]
pub struct FreeVariableIndex {
    pub upper_index: u16,
    pub local_index: u16
}

#[derive(Debug)]
pub struct Function {
    pub parameter_count: u16,
    pub local_variable_count: u16,

    pub free_variables: Vec<FreeVariableIndex>,

    pub instructions: Vec<Instruction>
}

#[derive(Debug)]
pub struct Assembly {
    pub functions: Vec<Function>,
    pub constants: Vec<Object>
}

impl Assembly {
    pub fn new() -> Assembly {
        Assembly {
            functions: Vec::new(),
            constants: Vec::new()
        }
    }

    pub fn form_binary() {

    }

    pub fn serialize(&self) {

    }
}