use crate::runtime::program::{Program, RuntimeError};
use std::collections::{HashMap, LinkedList};
use crate::runtime::object::{Object, ModelInstance, Reference, make_reference};
use crate::intermediate::Position;
use crate::runtime::opcode::{Instruction, OpCode};
use std::ops::Deref;

macro_rules! ensure_type {
    ($object: expr, $object_pattern: pat, $message: expr, $position: expr) => {
        if let $object_pattern = $object {
            Ok(())
        } else {
            Err(RuntimeError::new($message, $position))
        }
    }
}

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

pub type NativeFunction = fn(&mut State, &[Object]) -> Result<Object, RuntimeError>;

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

    fn call_function_by_index(&mut self, function_index: usize, parameters: &[ Object ]) -> Result<(), RuntimeError> {
        let function = self.program.functions.get(function_index).unwrap();

        // function index is checked outside, no need to check here
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

    fn call_model_by_index(&mut self, model_index: usize, parameters: &[ Object ]) -> Result<(), RuntimeError> {
        let model = self.program.models.get(model_index).unwrap();
        if parameters.len() > model.property_indices.len() {
            return Err(RuntimeError::new("too many parameters", Position::none()));
        };

        let mut properties = parameters.iter().cloned().collect::<Vec<Object>>();

        for _ in properties.len()..model.property_indices.len() {
            properties.push(Object::Null);
        };

        self.push(Object::Instance(make_reference(ModelInstance {
            model_index,
            properties
        })));

        Ok(())
    }

    fn call_object(&mut self, object: Object, parameters: &[ Object ]) -> Result<(), RuntimeError> {
        match object {
            Object::Function(function_index) => self.call_function_by_index(function_index, parameters),
            Object::InstanceFunction(model, function_index) => self.call_function_by_index(function_index,&make_instance_call_parameters(model.deref().clone(), parameters)),
            Object::NativeFunction(function_index) => Ok(()),
            Object::InstanceNativeFunction(model, function_index) => Ok(()),
            Object::Model(model_index) => self.call_model_by_index(model_index, parameters),
            Object::NativeModel(model_index) => Ok(()),
            _ => Err(RuntimeError::new(&format!("can not call {:?}", object), self.last_position()))
        }
    }

    fn execute_call_opcode(&mut self, parameter_count: usize) -> Result<(), RuntimeError> {
        let mut parameters = vec![Object::Null; parameter_count];

        for i in (0..parameter_count).rev() {
            parameters[i] = self.stack.pop_back().unwrap();
        };

        let function_object = self.stack.pop_back().unwrap();

        self.call_object(function_object, &parameters)
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

    pub fn push(&mut self, object: Object) {
        self.stack.push_back(object)
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push_back(frame);
    }

    pub fn top(&self) -> Object {
        self.stack.back().unwrap().clone()
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

    // instance get for model
    fn instance_get_model(&mut self, model_index: usize, index: Object) -> Result<(), RuntimeError> {
        if let Object::String(key) = &index {
            let model = self.program.models.get(model_index).unwrap();

            if let Some(&function_index) = model.functions.get(key) {
                self.push(Object::Function(function_index));
                return Ok(());
            };
        }

        self.push(Object::Null);
        Ok(())
    }

    fn instance_get_model_instance(&mut self, model_instance: Reference<ModelInstance>, index: Object) -> Result<(), RuntimeError> {
        if let Object::String(key) = &index {
            let model = self.program.models.get(model_instance.borrow().deref().model_index).unwrap();

            // have property?
            if let Some(&property_index) = model.property_indices.get(key) {
                self.push(model_instance.borrow().deref().properties[property_index].clone());
                return Ok(());
            };

            // have function?
            if let Some(&function_index) = model.functions.get(key) {
                if self.program.functions[function_index].is_instance {
                    self.push(Object::InstanceFunction(Box::new(Object::Instance(model_instance.clone())), function_index));
                } else {
                    self.push(Object::Function(function_index));
                };

                return Ok(());
            };

        } else if let Object::Integer(i) = &index {
            if let Some(object) = model_instance.borrow().deref().properties.get(*i as usize) {
                self.push(object.clone());
                return Ok(());
            }
        }
        self.push(Object::Null);

        Ok(())
    }

    fn instance_get(&mut self) -> Result<(), RuntimeError> {
        let index = self.pop().unwrap();
        let instance = self.pop().unwrap();

        match instance {
            Object::Model(model_index) => self.instance_get_model(model_index, index)?,
            Object::Instance(model_instance) => self.instance_get_model_instance(model_instance, index)?,
            _ => {
                return Err(RuntimeError::new("this object's instance get did not implemented yet", self.last_position()));
            }
        };

        Ok(())
    }

    fn instance_set_model_instance_by_index(&mut self, model_instance: Reference<ModelInstance>, index: usize) -> Result<(), RuntimeError> {
        if let Some(object) = model_instance.borrow_mut().properties.get_mut(index as usize) {
            *object = self.top();
            return Ok(());
        } else {
            return Err(RuntimeError::new("index does not exists", self.last_position()));
        }
    }

    fn instance_set_model_instance(&mut self, model_instance: Reference<ModelInstance>, index: Object) -> Result<(), RuntimeError> {
        if let Object::String(key) = &index {
            let model = self.program.models.get(model_instance.borrow().deref().model_index).unwrap();

            // have property?
            if let Some(&property_index) = model.property_indices.get(key) {
                self.instance_set_model_instance_by_index(model_instance, property_index)?;
            };

        } else if let Object::Integer(i) = &index {
            self.instance_set_model_instance_by_index(model_instance, *i as usize)?;
        }

        Ok(())
    }

    fn instance_set(&mut self) -> Result<(), RuntimeError> {
        let index = self.pop().unwrap();
        let instance = self.pop().unwrap();

        match instance {
            Object::Instance(model_instance) => self.instance_set_model_instance(model_instance, index)?,
            _ => {
                return Err(RuntimeError::new("this object's instance set did not implemented yet", self.last_position()));
            }
        };

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), RuntimeError> {
        let instruction = self.current_instruction();
        let opcode = instruction.opcode();

        self.current_frame_as_mut().program_counter += 1;

        match opcode {
            OpCode::Pop => { self.stack.pop_back(); },
            OpCode::PushConstant => {
                let constant = self.program.constants[instruction.operand() as usize].clone();
                self.push(constant);
            },
            OpCode::PushNull => self.push(Object::Null),
            OpCode::PushBoolean => self.push(Object::Boolean(instruction.operand() == 1)),
            OpCode::Return => { self.frames.pop_back(); },

            OpCode::LocalGet => self.push(self.current_frame().locals.get(instruction.operand() as usize).unwrap().clone()),
            OpCode::LocalSet => { self.current_frame_as_mut().locals[instruction.operand() as usize] = self.top(); },
            OpCode::LocalInit => { self.current_frame_as_mut().locals[instruction.operand() as usize] = self.pop().unwrap(); },

            OpCode::ContextGet => self.push(self.locals.get(instruction.operand() as usize).unwrap().clone()),
            OpCode::ContextSet => { self.locals[instruction.operand() as usize] = self.top(); },

            OpCode::GlobalGet => {
                if let Some(Object::String(global_name)) = self.program.constants.get(instruction.operand() as usize) {
                    if let Some(object) = self.globals.get(global_name) {
                        self.push(object.clone());
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
            OpCode::InstanceGet => self.instance_get()?,
            OpCode::InstanceSet => self.instance_set()?,
            OpCode::Call => self.execute_call_opcode(instruction.operand() as usize)?,
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

        self.call_function_by_index(function_index, parameters)?;

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

// helpers
fn make_instance_call_parameters(object: Object, parameters: &[ Object ]) -> Vec<Object> {
    let mut new_parameters = vec![ object ];
    new_parameters.extend_from_slice(parameters);
    new_parameters
}