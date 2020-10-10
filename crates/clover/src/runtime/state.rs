use std::collections::{HashMap, LinkedList};
use crate::runtime::object::{Slot, Object, ClosureData};
use crate::runtime::assembly::Assembly;
use crate::runtime::opcode::{Instruction, OpCode};
use std::rc::Rc;
use std::ops::Deref;

pub struct Frame {
    pub locals: Vec<Slot>,
    pub program_counter: usize,
    pub assembly_index: usize,
    pub function_index: usize
}

impl Frame {
    pub fn new(local_count: u16, assembly_index: usize, function_index: usize) -> Frame {
        let mut locals = Vec::new();
        for _ in 0..local_count {
            locals.push(Slot::new(Object::Null));
        };

        Frame {
            locals,
            program_counter: 0,
            assembly_index,
            function_index
        }
    }
}

pub struct State {
    globals: HashMap<String, Slot>,
    stack: LinkedList<Slot>,
    frames: LinkedList<Frame>,
    assemblies: Vec<Assembly>
}

impl State {

    fn build_meta_tables() -> Vec<Object> {
        let mut meta_tables = Vec::new();





        meta_tables
    }

    pub fn new() -> State {
        State {
            globals: HashMap::new(),
            stack: LinkedList::new(),
            frames: LinkedList::new(),
            assemblies: Vec::new()
        }
    }

    pub fn add_assembly(&mut self, assembly: Assembly) -> usize {
        let index = self.assemblies.len();
        self.assemblies.push(assembly);
        index
    }

    pub fn execute(&mut self, assembly_index: usize) -> Result<Object, String> {
        if let Some(assembly) = self.assemblies.get(assembly_index) {
            if assembly.functions.is_empty() {
                return Ok(Object::Null);
            };

            let function_index = assembly.functions.len() - 1;
            let function = assembly.functions.get(function_index).unwrap();

            self.push_frame(function.local_variable_count, assembly_index, function_index);

            while !self.frames.is_empty() {
                self.step()?;
            };
        }

        Ok(self.stack.pop_back().unwrap().deref().clone())
    }

    pub fn current_frame(&mut self) -> &mut Frame {
        self.frames.back_mut().unwrap()
    }

    pub fn current_instruction(&self) -> Instruction {
        let (assembly_index, function_index, program_counter) = {
            let frame = self.frames.back().unwrap();
            (frame.assembly_index, frame.function_index, frame.program_counter)
        };

        let assembly = &self.assemblies[assembly_index];
        let function = &assembly.functions[function_index];

        function.instructions[program_counter]
    }

    pub fn current_assembly_index(&self) -> usize {
        self.frames.back().unwrap().assembly_index
    }

    pub fn current_assembly(&self) -> &Assembly {
        let frame = self.frames.back().unwrap();
        &self.assemblies[self.current_assembly_index()]
    }

    pub fn execute_operation(&mut self, name: &str) -> Result<(), String> {
        let right = self.stack.pop_back().unwrap();
        let left = self.stack.pop_back().unwrap();

        match name {
            "_add" => {
                // TODO: use meta method

                if let Object::Integer(left_integer) = left.deref() {
                    if let Object::Integer(right_integer) = right.deref() {
                        self.stack.push_back(Slot::new(Object::Integer(left_integer + right_integer)));

                        return Ok(());
                    }
                }

                Err("not implement yet".to_string())
            },

            _ => {
                Err("not implement yet".to_string())
            }
        }

    }

    fn push_closure(&mut self, function_index: usize) -> Result<(), String> {
        let free_variable_indices = self.current_assembly().functions.get(function_index).unwrap().free_variables.clone();
        let assembly_index = self.current_assembly_index();

        let mut free_variables = HashMap::new();

        for free_variable_index in free_variable_indices {
            free_variables.insert(free_variable_index.local_index as usize, self.current_frame().locals[free_variable_index.upper_index as usize].clone());
        };

        self.stack.push_back(Slot::new(Object::Closure(Rc::new(ClosureData {
            assembly_index,
            free_variables,
            function_index
        }))));

        Ok(())
    }

    pub fn instance_get(&mut self, object: &Object, key: &Object) -> Slot {
        Slot::new(Object::Null)
    }

    pub fn step(&mut self) -> Result<(), String> {
        let instruction = self.current_instruction();
        let opcode = instruction.opcode();

        self.current_frame().program_counter += 1;

        match opcode {
            OpCode::Pop => { self.stack.pop_back(); },
            OpCode::PushConstant => {
                let constant = self.current_assembly().constants[instruction.operand() as usize].clone();
                self.stack.push_back(Slot::new(constant));
            },
            OpCode::PushNull => self.stack.push_back(Slot::new(Object::Null)),
            OpCode::PushBoolean => self.stack.push_back(Slot::new(Object::Boolean(instruction.operand() == 1))),
            OpCode::Return => {
                for _ in 0..instruction.operand() {
                    self.frames.pop_back();
                };
            },
            OpCode::SetLocal => {
                let value = self.stack.pop_back().unwrap();
                let slot = self.current_frame().locals.get_mut(instruction.operand() as usize).unwrap();
                *Rc::get_mut(slot).unwrap() = value.deref().clone();
            },
            OpCode::GetLocal => {
                let value = self.current_frame().locals.get(instruction.operand() as usize).unwrap().deref().clone();
                self.stack.push_back(Slot::new(value));
            },
            OpCode::GetEnvironment => {
                if let Object::String(key) = &self.current_assembly().constants[instruction.operand() as usize] {
                    if let Some(global_object) = self.globals.get(key){
                        let value = global_object.deref().clone();
                        self.stack.push_back(Slot::new(value));
                    } else {
                        self.stack.push_back(Slot::new(Object::Null));
                    }

                } else {
                    return Err(format!("get global with a invalid object"));
                }
            }
            OpCode::Add => { self.execute_operation("_add")?; },
            OpCode::Sub => { self.execute_operation("_sub")?; },
            OpCode::Multiply => { self.execute_operation("_multiply")?; },
            OpCode::Divide => { self.execute_operation("_divide")?; },

            OpCode::Closure => { self.push_closure(instruction.operand() as usize)? },

            _ => {}
        };

        Ok(())
    }

    pub fn execute_closure(&mut self, closure: &Object) {


    }

    pub fn push_frame(&mut self, local_count: u16, assembly_index: usize, function_index: usize) {
        self.frames.push_back(Frame::new(local_count, assembly_index, function_index));
    }

    pub fn add_global(&mut self, name: String, object: Object) {
        self.globals.insert(name, Slot::new(object));
    }
}