use crate::runtime::program::{Program, RuntimeError};
use std::collections::{HashMap, LinkedList};
use crate::runtime::object::Object;
use crate::intermediate::Position;
use crate::runtime::opcode::{Instruction, OpCode};

pub struct Frame {
    pub locals: Vec<Object>,
    pub program_counter: usize,
    pub function_index: usize
}

impl Frame {
    pub fn new(local_count: usize, function_index: usize) -> Frame {
        let mut locals = Vec::new();
        for _ in 0..local_count {
            locals.push(Object::Null);
        };

        Frame {
            locals,
            program_counter: 0,
            function_index
        }
    }
}

pub struct State {
    pub globals: HashMap<String, Object>,
    pub locals: Vec<Object>,
    pub stack: LinkedList<Object>,
    pub frames: LinkedList<Frame>,
    pub program: Program
}

impl State {
    pub fn new(program: Program) -> State {
        let mut locals = Vec::new();

        for i in 0..program.local_count {
            locals.push(if let Some(constant_index) = program.local_values.get(&i) {
                program.constants.get(*constant_index).unwrap().clone()
            } else {
                Object::Null
            });
        };

        State {
            globals: HashMap::new(),
            locals,
            stack: LinkedList::new(),
            frames: LinkedList::new(),
            program
        }
    }

    fn call(&mut self, function_index: usize, parameters: &[ Object ]) -> Result<(), RuntimeError> {
        let function = self.program.functions.get(function_index).unwrap();

        // function index and parameters len is checked outside, no need to check here
        if parameters.len() > function.parameter_count {
            return Err(RuntimeError::new("too many parameters", Position::none()));
        }

        let mut frame = Frame::new(function.local_count, function_index);

        for (i, object) in parameters.iter().enumerate() {
            frame.locals[i] = object.clone();
        }

        self.push_frame(frame);

        Ok(())
    }

    fn current_instruction(&self) -> Instruction {
        let (function_index, program_counter) = {
            let frame = self.frames.back().unwrap();
            (frame.function_index, frame.program_counter)
        };

        let function = self.program.functions.get(function_index).unwrap();

        function.instructions[program_counter]
    }

    fn current_frame_as_mut(&mut self) -> &mut Frame {
        self.frames.back_mut().unwrap()
    }

    fn current_frame(&self) -> &Frame {
        self.frames.back().unwrap()
    }

    pub fn pop(&mut self) -> Option<Object> {
        self.stack.pop_back()
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push_back(frame);
    }

    pub fn last_position(&self) -> Position {
        let program_counter = self.current_frame().program_counter;
        if let Some(debug_info) = &self.program.debug_info {
            if program_counter > 0 {
                let function_index = self.current_frame().function_index;

                return debug_info.functions[function_index][program_counter - 1];
            };
        };

        Position::none()
    }

    pub fn step(&mut self) -> Result<(), RuntimeError> {
        let instruction = self.current_instruction();
        let opcode = instruction.opcode();

        self.current_frame_as_mut().program_counter += 1;

        match opcode {
            OpCode::Pop => { self.stack.pop_back(); },
            OpCode::PushConstant => {
                let constant = self.program.constants[instruction.operand() as usize].clone();
                self.stack.push_back(constant);
            },
            OpCode::PushNull => self.stack.push_back(Object::Null),
            OpCode::PushBoolean => self.stack.push_back(Object::Boolean(instruction.operand() == 1)),
            OpCode::Return => { self.frames.pop_back(); },

            OpCode::LocalGet => self.stack.push_back(self.current_frame().locals.get(instruction.operand() as usize).unwrap().clone()),
            OpCode::LocalSet => { self.current_frame_as_mut().locals[instruction.operand() as usize] = self.stack.back().unwrap().clone(); },
            OpCode::LocalInit => { self.current_frame_as_mut().locals[instruction.operand() as usize] = self.pop().unwrap(); },

            OpCode::ContextGet => self.stack.push_back(self.locals.get(instruction.operand() as usize).unwrap().clone()),
            OpCode::ContextSet => { self.locals[instruction.operand() as usize] = self.stack.back().unwrap().clone(); },

            OpCode::GlobalGet => {
                if let Some(Object::String(global_name)) = self.program.constants.get(instruction.operand() as usize) {
                    if let Some(object) = self.globals.get(global_name) {
                        self.stack.push_back(object.clone());
                    } else {
                        return Err(RuntimeError::new("global not found", self.last_position()));
                    }
                }
            },
            OpCode::GlobalSet => {
                if let Some(Object::String(global_name)) = self.program.constants.get(instruction.operand() as usize) {
                    if let Some(object) = self.globals.get_mut(global_name) {
                        *object = self.stack.back().unwrap().clone();
                    } else {
                        return Err(RuntimeError::new("global not found", self.last_position()));
                    }
                }
            },
            _ => {
                // not implemented
            }
        }

        Ok(())
    }

    pub fn execute_by_function_index(&mut self, function_index: usize, parameters: &[ Object ]) -> Result<Object, RuntimeError> {
        if self.program.functions.len() <= function_index {
            return Err(RuntimeError::new("can not found function", Position::none()));
        };

        let function = self.program.functions.get(function_index).unwrap();

        if parameters.len() > function.parameter_count {
            return Err(RuntimeError::new("too many parameters", Position::none()));
        };

        self.call(function_index, parameters)?;

        while !self.frames.is_empty() {
            self.step()?;
        };

        if let Some(object) = self.pop() {
            Ok(object)
        } else {
            Err(RuntimeError::new("there is no result", Position::none()))
        }
    }

    pub fn execute(&mut self) -> Result<Object, RuntimeError> {
        for &global_index in self.program.global_dependencies.iter() {
            if let Some(Object::String(global_name)) = self.program.constants.get(global_index) {
                if !self.globals.contains_key(global_name) {
                    return Err(RuntimeError::new(&format!("this program need a global variable [{}] which is not found in this state", global_name), Position::none()));
                }
            }
        }

        self.execute_by_function_index(self.program.entry_point, &[])
    }
}