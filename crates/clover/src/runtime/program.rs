use std::collections::{HashMap, LinkedList};

use crate::intermediate::Position;
use crate::runtime::assembly_information::{DebugInfo, FileInfo};
use crate::runtime::object::{Object, Reference, make_reference};
use crate::runtime::opcode::Instruction;
use crate::runtime::state::Frame;
use std::io::{Write, Read};
use byteorder::{ReadBytesExt, LittleEndian, WriteBytesExt};

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

    pub property_names: Vec<Reference<String>>
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
        self.property_names.push(make_reference(property_name.to_string()));

        true
    }

    fn serialize(&self, writer: &mut dyn Write) -> Result<(), std::io::Error>  {
        writer.write_u32::<LittleEndian>(self.property_names.len() as u32)?;

        for property_name_reference in &self.property_names {
            let property_name = property_name_reference.borrow();
            serialize_string(property_name.as_str(), writer)?;
        }

        writer.write_u32::<LittleEndian>(self.functions.len() as u32)?;
        for (function_name, &function_index) in &self.functions {
            serialize_string(function_name, writer)?;
            writer.write_u32::<LittleEndian>(function_index as u32)?;
        };

        Ok(())
    }

    fn deserialize(reader: &mut dyn Read) -> Result<Model, std::io::Error> {
        let mut model = Model::new();
        let property_count = reader.read_u32::<LittleEndian>()?;

        for _ in 0..property_count {
            let property_name = deserialize_string(reader)?;
            model.add_property(property_name.as_str());
        }

        let function_count = reader.read_u32::<LittleEndian>()?;
        for _ in 0..function_count {
            let function_name = deserialize_string(reader)?;
            let function_index = reader.read_u32::<LittleEndian>()? as usize;

            model.functions.insert(function_name, function_index);
        };

        Ok(model)
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

    fn serialize(&self, writer: &mut dyn Write) -> Result<(), std::io::Error>  {
        writer.write_u32::<LittleEndian>(self.parameter_count as u32)?;
        writer.write_u32::<LittleEndian>(self.local_count as u32)?;
        writer.write_u32::<LittleEndian>(self.rescue_position as u32)?;
        writer.write_u8(if self.is_instance { 1 } else { 0 })?;

        writer.write_u32::<LittleEndian>(self.instructions.len() as u32)?;

        for instruction in &self.instructions {
            writer.write_u64::<LittleEndian>(instruction.into())?;
        };

        Ok(())
    }

    fn deserialize(reader: &mut dyn Read) -> Result<Function, std::io::Error> {
        let parameter_count = reader.read_u32::<LittleEndian>()? as usize;
        let local_count = reader.read_u32::<LittleEndian>()? as usize;
        let rescue_position = reader.read_u32::<LittleEndian>()? as usize;
        let is_instance = reader.read_u8()?;

        let instruction_count = reader.read_u32::<LittleEndian>()?;
        let mut instructions = Vec::new();

        for _ in 0..instruction_count {
            instructions.push(Instruction::from(reader.read_u64::<LittleEndian>()?));
        };

        Ok(Function {
            parameter_count,
            local_count,
            rescue_position,
            is_instance: is_instance == 1,
            instructions
        })
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

fn serialize_string(string: &str, writer: &mut dyn Write) -> Result<(), std::io::Error> {
    let string_binary = string.as_bytes();

    writer.write_u32::<LittleEndian>(string_binary.len() as u32)?;
    writer.write_all(string_binary)?;

    Ok(())
}

fn deserialize_string(reader: &mut dyn Read) -> Result<String, std::io::Error> {
    let string_length = reader.read_u32::<LittleEndian>()? as usize;

    let mut buffer: Vec<u8> = vec![0; string_length];

    reader.read(&mut buffer)?;

    if let Ok(string) = String::from_utf8(buffer) {
        Ok(string)
    } else {
        // TODO : change error type
        Err(std::io::Error::from_raw_os_error(0))
    }
}

impl Program {
    pub const NULL_CONSTANT_INDEX: usize = 0;
    pub const TRUE_CONSTANT_INDEX: usize = 1;
    pub const FALSE_CONSTANT_INDEX: usize = 2;
    pub const DEFAULT_CONSTANTS: [Object; 3] = [ Object::Null, Object::Boolean(true), Object::Boolean(false) ];

    const OBJECT_TYPE_INTEGER: u8 = 0;
    const OBJECT_TYPE_FLOAT: u8 = 1;
    const OBJECT_TYPE_STRING: u8 = 2;
    const OBJECT_TYPE_MODEL: u8 = 3;
    const OBJECT_TYPE_FUNCTION: u8 = 4;

    // luck
    const HEADER: u32 = 0x6b63756c;

    pub fn serialize(&self, writer: &mut dyn Write) -> Result<(), std::io::Error> {
        writer.write_u32::<LittleEndian>(Program::HEADER)?;
        writer.write_u8(crate::version::MAJOR)?;
        writer.write_u8(crate::version::MINOR)?;
        writer.write_u8(crate::version::PATCH)?;
        writer.write_u8(0)?;

        // models
        writer.write_u32::<LittleEndian>(self.models.len() as u32)?;
        for model in &self.models {
            model.serialize(writer)?;
        };

        // functions
        writer.write_u32::<LittleEndian>(self.functions.len() as u32)?;
        for function in &self.functions {
            function.serialize(writer)?;
        };

        // constants
        writer.write_u32::<LittleEndian>(self.constants.len() as u32)?;
        for i in Program::DEFAULT_CONSTANTS.len()..self.constants.len() {
            let object = &self.constants[i];

            match object {
                Object::Integer(value) => {
                    writer.write_u8(Program::OBJECT_TYPE_INTEGER)?;
                    writer.write_i64::<LittleEndian>(*value)?;
                },
                Object::Float(value) => {
                    writer.write_u8(Program::OBJECT_TYPE_FLOAT)?;
                    writer.write_f64::<LittleEndian>(*value)?;
                },
                Object::String(string) => {
                    writer.write_u8(Program::OBJECT_TYPE_STRING)?;
                    serialize_string(string.borrow().as_str(), writer)?;
                },
                Object::Model(model_index) => {
                    writer.write_u8(Program::OBJECT_TYPE_MODEL)?;
                    writer.write_u32::<LittleEndian>(*model_index as u32)?;
                },
                Object::Function(function_index) => {
                    writer.write_u8(Program::OBJECT_TYPE_FUNCTION)?;
                    writer.write_u32::<LittleEndian>(*function_index as u32)?;
                },
                _ => {
                    // can't be here
                    return Err(std::io::Error::from_raw_os_error(0));
                }
            }
        };

        // global dependencies
        writer.write_u32::<LittleEndian>(self.global_dependencies.len() as u32)?;
        for &global_dependency in &self.global_dependencies {
            writer.write_u32::<LittleEndian>(global_dependency as u32)?;
        };

        // local count
        writer.write_u32::<LittleEndian>(self.local_count as u32)?;

        // local values
        writer.write_u32::<LittleEndian>(self.local_values.len() as u32)?;
        for (&index, &value) in &self.local_values {
            writer.write_u32::<LittleEndian>(index as u32)?;
            writer.write_u32::<LittleEndian>(value as u32)?;
        };

        // entry point
        writer.write_u32::<LittleEndian>(self.entry_point as u32)?;

        Ok(())
    }

    pub fn deserialize(reader: &mut dyn Read) -> Result<Program, std::io::Error> {
        if Program::HEADER != reader.read_u32::<LittleEndian>()? {
            println!("warn: header not match");
        };

        if crate::version::MAJOR != reader.read_u8()? {
            println!("warn: major version not match");
        };

        if crate::version::MINOR != reader.read_u8()? {
            println!("warn: minor version not match");
        };

        if crate::version::PATCH != reader.read_u8()? {
            println!("warn: patch version not match");
        };

        if 0 != reader.read_u8()? {
            println!("warn: header end not match");
        };

        // models
        let mut models = Vec::new();
        let model_count = reader.read_u32::<LittleEndian>()?;

        for _ in 0..model_count {
            models.push(Model::deserialize(reader)?);
        };

        // functions
        let mut functions = Vec::new();
        let function_count = reader.read_u32::<LittleEndian>()?;

        for _ in 0..function_count {
            functions.push(Function::deserialize(reader)?);
        };

        // constants
        let mut constants = Program::DEFAULT_CONSTANTS.to_vec();
        let constant_count = reader.read_u32::<LittleEndian>()? as usize;

        for _ in Program::DEFAULT_CONSTANTS.len()..constant_count {
            let object_type = reader.read_u8()?;

            let constant = match object_type {
                Program::OBJECT_TYPE_INTEGER => {
                    Object::Integer(reader.read_i64::<LittleEndian>()?)
                },
                Program::OBJECT_TYPE_FLOAT => {
                    Object::Float(reader.read_f64::<LittleEndian>()?)
                },
                Program::OBJECT_TYPE_STRING => {
                    Object::String(make_reference(deserialize_string(reader)?))
                },
                Program::OBJECT_TYPE_MODEL => {
                    Object::Model(reader.read_u32::<LittleEndian>()? as usize)
                },
                Program::OBJECT_TYPE_FUNCTION => {
                    Object::Function(reader.read_u32::<LittleEndian>()? as usize)
                },
                _ => {
                    // can't be here
                    return Err(std::io::Error::from_raw_os_error(0));
                }
            };

            constants.push(constant);
        };

        // global dependencies
        let mut global_dependencies = Vec::new();
        let global_dependency_count = reader.read_u32::<LittleEndian>()?;
        for _ in 0..global_dependency_count {
            global_dependencies.push(reader.read_u32::<LittleEndian>()? as usize);
        };

        let local_count = reader.read_u32::<LittleEndian>()? as usize;

        let mut local_values = HashMap::new();
        let local_value_count = reader.read_u32::<LittleEndian>()?;
        for _ in 0..local_value_count {
            let index = reader.read_u32::<LittleEndian>()? as usize;
            let value = reader.read_u32::<LittleEndian>()? as usize;
            local_values.insert(index, value);
        };

        let entry_point = reader.read_u32::<LittleEndian>()? as usize;

        Ok(Program {
            models,
            functions,
            constants,
            global_dependencies,

            local_count,
            local_values,

            entry_point,

            file_info: None,
            debug_info: None
        })
    }

}