use std::collections::{HashMap, LinkedList};

use crate::intermediate::Position;
use crate::runtime::assembly_information::{DebugInfo, FileInfo};
use crate::runtime::object::Object;
use crate::runtime::opcode::Instruction;
use crate::runtime::state::Frame;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub position: Position,
    pub stack: LinkedList<Frame>
}

impl RuntimeError {
    pub fn new(message: &str, position: Position) -> RuntimeError {
        RuntimeError {
            message: message.to_string(),
            position,
            stack: LinkedList::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Model {
    pub property_indices: HashMap<String, usize>,
    pub functions: HashMap<String, usize>,

    pub property_names: Vec<String>
}

impl Model {
    pub fn new() -> Model {
        Model {
            property_indices: HashMap::new(),
            functions: HashMap::new(),
            property_names: Vec::new()
        }
    }

    pub fn add_property(&mut self, property_name: &str) -> bool {
        if self.property_indices.contains_key(property_name) {
            return false;
        };

        self.property_indices.insert(property_name.to_string(), self.property_indices.len());
        self.property_names.push(property_name.to_string());

        true
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameter_count: usize,
    pub local_count: usize,
    pub rescue_position: usize,
    pub is_instance: bool,

    pub instructions: Vec<Instruction>
}

impl Function {
    pub fn new() -> Function {
        Function {
            parameter_count: 0,
            local_count: 0,
            rescue_position: 0,
            is_instance: false,

            instructions: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub models: Vec<Model>,
    pub functions: Vec<Function>,
    pub constants: Vec<Object>,

    // constant indices point to name of global
    pub global_dependencies: Vec<usize>,

    pub local_count: usize,

    // use to init local variable, key is local index, value is constant index
    pub local_values: HashMap<usize, usize>,

    // entry_point - 1 is the function index
    pub entry_point: usize,

    pub file_info: Option<FileInfo>,
    pub debug_info: Option<DebugInfo>
}

impl Program {
    pub const NULL_CONSTANT_INDEX: usize = 0;
    pub const TRUE_CONSTANT_INDEX: usize = 1;
    pub const FALSE_CONSTANT_INDEX: usize = 2;
    pub const DEFAULT_CONSTANTS: [Object; 3] = [ Object::Null, Object::Boolean(true), Object::Boolean(false) ];
}