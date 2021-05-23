use crate::runtime::opcode::{Instruction, OpCode};
use crate::runtime::object::Object;
use std::collections::HashMap;
use crate::runtime::assembly_information::{DebugInfo, FileInfo};

#[derive(Debug, Clone)]
pub struct Model {
    pub property_names: Vec<String>,
    pub functions: HashMap<String, usize>
}

impl Model {
    pub fn new() -> Model {
        Model {
            property_names: Vec::new(),
            functions: HashMap::new()
        }
    }

    pub fn add_property(&mut self, property_name: &str) {
        self.property_names.push(property_name.to_string());
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameter_count: u16,
    pub local_count: u16,
    pub is_instance: bool,

    pub instructions: Vec<Instruction>
}

impl Function {
    pub fn new() -> Function {
        Function {
            parameter_count: 0,
            local_count: 0,
            is_instance: false,

            instructions: Vec::new()
        }
    }

    fn emit(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    fn emit_opcode(&mut self, opcode: OpCode) {
        self.emit(opcode.to_instruction(0));
    }

}

#[derive(Debug)]
pub struct Program {
    pub models: Vec<Model>,
    pub functions: Vec<Function>,
    pub constants: Vec<Object>,

    pub local_count: usize,

    // use to init local variable, key is local index, value is constant index
    pub local_values: HashMap<usize, usize>,

    // entry_point - 1 is the function index
    pub entry_point: usize,

    pub file_info: Option<FileInfo>,
    pub debug_info: Option<DebugInfo>
}