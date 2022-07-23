use crate::runtime::program::{Program, RuntimeError};
use std::collections::{HashMap, LinkedList};
use crate::runtime::object::{Object, ModelInstance, Reference, make_reference, NativeModel, NativeFunction, NativeModelInstance};
use crate::intermediate::Position;
use crate::runtime::opcode::{Instruction, OpCode};
use std::ops::Deref;
use crate::runtime::object_property::{instance_get_array, instance_get_integer, instance_get_float, instance_get_string};

#[derive(Debug, Clone)]
pub struct Frame {
    pub locals: Vec<Object>,
    pub program_counter: usize,
    pub function_index: usize,
    pub stack_size: usize
}

impl Frame {
    pub fn new(local_count: usize, function_index: usize, stack_size: usize) -> Frame {
        let mut locals = Vec::new();
        for _ in 0..local_count {
            locals.push(Object::Null);
        };

        Frame {
            locals,
            program_counter: 0,
            function_index,
            stack_size
        }
    }
}

pub struct State {
    globals: HashMap<String, Object>,
    locals: Vec<Object>,
    native_models: Vec<Reference<dyn NativeModel>>,
    stack: LinkedList<Object>,
    frames: LinkedList<Frame>,
    program: Program
}

impl From<Program> for State {
    fn from(program: Program) -> Self {
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
            native_models: Vec::new(),
            stack: LinkedList::new(),
            frames: LinkedList::new(),
            program
        }
    }
}

impl State {
    pub fn get_program(&self) -> &Program {
        &self.program
    }

    pub fn call_function_by_index(&mut self, function_index: usize, parameters: &[ Object ]) -> Result<(), RuntimeError> {
        let function = self.program.functions.get(function_index).unwrap();

        // function index is checked outside, no need to check here
        if parameters.len() > function.parameter_count {
            return Err(RuntimeError::new("too many parameters", Position::none()));
        }

        let mut frame = Frame::new(function.local_count, function_index, self.stack.len());

        for (i, object) in parameters.iter().enumerate() {
            frame.locals[i] = object.clone();
        }

        self.push_frame(frame);

        Ok(())
    }

    pub fn current_frame_as_mut(&mut self) -> &mut Frame {
        self.frames.back_mut().unwrap()
    }

    pub fn current_frame(&self) -> &Frame {
        self.frames.back().unwrap()
    }

    pub fn pop(&mut self) -> Option<Object> {
        self.stack.pop_back()
    }

    pub fn push(&mut self, object: Object) {
        self.stack.push_back(object)
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

    fn execute_until_frame_size_equal(&mut self, frame_size: usize) -> Result<Object, RuntimeError> {
        while self.frames.len() != frame_size {
            self.step()?;
        };

        self.get_top()
    }

    pub fn execute_by_object(&mut self, object: Object, parameters: &[ Object ]) -> Result<Object, RuntimeError> {
        let frame_size = self.frames.len();

        self.call_object(object, parameters)?;

        self.execute_until_frame_size_equal(frame_size)
    }

    pub fn get_object_property_by_name(&mut self, object: Object, name: &str) -> Result<Object, RuntimeError> {
        let name_index = Object::String(make_reference(name.to_string()));

        self.instance_get_with_index(object, &name_index)?;

        self.get_top()
    }

    pub fn get_object_property_by_index(&mut self, object: Object, index: i64) -> Result<Object, RuntimeError> {
        let number_index = Object::Integer(index);

        self.instance_get_with_index(object, &number_index)?;

        self.get_top()
    }

    pub fn execute_by_function_index(&mut self, function_index: usize, parameters: &[ Object ]) -> Result<Object, RuntimeError> {
        if self.program.functions.len() <= function_index {
            return Err(RuntimeError::new("can not found function", Position::none()));
        };

        let function = self.program.functions.get(function_index).unwrap();

        if parameters.len() > function.parameter_count {
            return Err(RuntimeError::new("too many parameters", Position::none()));
        };

        let frame_size = self.frames.len();

        self.call_function_by_index(function_index, parameters)?;

        self.execute_until_frame_size_equal(frame_size)
    }

    pub fn execute(&mut self) -> Result<Object, RuntimeError> {
        for &global_index in self.program.global_dependencies.iter() {
            if let Some(Object::String(global_name)) = self.program.constants.get(global_index) {
                if !self.globals.contains_key(global_name.borrow().deref()) {
                    return Err(RuntimeError::new(&format!("this program need a global variable [{}] which is not found in this state", global_name.borrow().deref()), Position::none()));
                }
            }
        }

        self.execute_by_function_index(self.program.entry_point, &[])
    }

    pub fn add_native_function(&mut self, name: &str, function: NativeFunction)  {
        self.globals.insert(name.to_string(), Object::NativeFunction(function));
    }

    pub fn add_native_model(&mut self, name: &str, native_model: Reference<dyn NativeModel>) -> usize {
        let index = self.native_models.len();
        self.native_models.push(native_model);

        self.globals.insert(name.to_string(), Object::NativeModel(index));

        index
    }

    pub fn step(&mut self) -> Result<(), RuntimeError> {
        if let Err(mut error) = self.internal_step() {

            let mut call_stack = LinkedList::new();

            while self.frames.len() > 0 {
                let rescue_position = self.program.functions.get(self.current_frame().function_index).unwrap().rescue_position;

                if rescue_position > 0 {
                    self.current_frame_as_mut().program_counter = rescue_position;
                    return Ok(());
                } else {
                    let frame = self.frames.pop_back().unwrap();
                    while self.stack.len() > frame.stack_size {
                        self.stack.pop_back();
                    };
                    call_stack.push_front(frame);
                }
            }

            error.stack = call_stack;

            Err(error)
        } else {
            Ok(())
        }
    }
}

impl State {
    fn get_top(&mut self) -> Result<Object, RuntimeError> {
        if let Some(object) = self.pop() {
            Ok(object)
        } else {
            Err(RuntimeError::new("there is no result", Position::none()))
        }
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

    fn call_native_model_by_index(&mut self, model_index: usize, parameters: &[ Object ]) -> Result<(), RuntimeError> {
        let native_model = self.native_models.get(model_index).unwrap().clone();

        let result = native_model.borrow_mut().call(self, parameters)?;

        self.push(result);

        Ok(())
    }

    fn call_native_function(&mut self, function: NativeFunction, parameters: &[ Object ]) -> Result<(), RuntimeError> {
            let result = function(self, parameters)?;
            self.push(result);
            Ok(())
    }

    fn call_instance_native_function(&mut self, instance: Reference<dyn NativeModelInstance>, function_name: &str, parameters: &[ Object ]) -> Result<(), RuntimeError> {
        let instance_copy = instance.clone();
        let result = instance.borrow_mut().call(instance_copy, self, function_name, parameters)?;
        self.push(result);
        Ok(())
    }

    fn call_object(&mut self, object: Object, parameters: &[ Object ]) -> Result<(), RuntimeError> {
        match object {
            Object::Function(function_index) => self.call_function_by_index(function_index, parameters),
            Object::InstanceFunction(model, function_index) => self.call_function_by_index(function_index,&make_instance_call_parameters(model.deref().clone(), parameters)),
            Object::NativeFunction(function) => self.call_native_function(function, parameters),
            Object::InstanceNativeFunction(instance, function_name) => self.call_instance_native_function(instance, &function_name, parameters),
            Object::NativeModel(model_index) => self.call_native_model_by_index(model_index, parameters),
            Object::Model(model_index) => self.call_model_by_index(model_index, parameters),
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

    fn push_frame(&mut self, frame: Frame) {
        self.frames.push_back(frame);
    }

    fn pop_frame(&mut self) {
        let frame = self.frames.pop_back().unwrap();

        if self.stack.len() > frame.stack_size + 1 {
            return;
        };

        let return_value = self.pop().unwrap();

        // clean up stack
        while self.stack.len() > frame.stack_size {
            self.pop();
        };

        self.push(return_value);
    }

    fn instance_get_native_model(&mut self, model_index: usize, key: &str) -> Result<(), RuntimeError> {
        let model = self.native_models.get(model_index).unwrap();
        let result = model.borrow().model_get(key)?;
        self.push(result);

        Ok(())
    }

    fn instance_get_native_instance(&mut self, instance: Reference<dyn NativeModelInstance>, key: &str) -> Result<(), RuntimeError> {
        let instance_copy = instance.clone();
        let result = instance.borrow().instance_get(instance_copy, key)?;
        self.push(result);
        Ok(())
    }

    fn instance_get_with_index(&mut self, instance: Object, index: &Object) -> Result<(), RuntimeError> {
        match instance {
            Object::Model(model_index) => self.index_get_model(model_index, index)?,
            Object::Instance(model_instance) => self.index_get_model_instance(model_instance, index)?,
            Object::NativeModel(model_index) => self.instance_get_native_model(model_index, index.as_reference_string().borrow().deref())?,
            Object::NativeInstance(instance) => self.instance_get_native_instance(instance, index.as_reference_string().borrow().deref())?,

            Object::Integer(value) => instance_get_integer(self, value, index.as_reference_string().borrow().deref())?,
            Object::Float(value) => instance_get_float(self, value, index.as_reference_string().borrow().deref())?,
            Object::String(value) => instance_get_string(self, value, index.as_reference_string().borrow().deref())?,

            Object::Array(array) => instance_get_array(self, array, index.as_reference_string().borrow().deref())?,
            _ => {
                return Err(RuntimeError::new("this object's instance get did not implemented yet", self.last_position()));
            }
        };

        Ok(())
    }

    fn instance_get(&mut self) -> Result<(), RuntimeError> {
        let index = self.pop().unwrap();
        let instance = self.pop().unwrap();

        self.instance_get_with_index(instance, &index)
    }

    // index get for model
    fn index_get_model(&mut self, model_index: usize, index: &Object) -> Result<(), RuntimeError> {
        if let Object::String(key) = &index {
            let model = self.program.models.get(model_index).unwrap();

            if let Some(&function_index) = model.functions.get(key.borrow().deref()) {
                self.push(Object::Function(function_index));
                return Ok(());
            };
        }

        self.push(Object::Null);
        Ok(())
    }

    fn index_get_model_instance(&mut self, model_instance: Reference<ModelInstance>, index: &Object) -> Result<(), RuntimeError> {
        if let Object::String(key) = &index {
            let model = self.program.models.get(model_instance.borrow().deref().model_index).unwrap();

            // have property?
            if let Some(&property_index) = model.property_indices.get(key.borrow().deref()) {
                self.push(model_instance.borrow().deref().properties[property_index].clone());
                return Ok(());
            };

            // have function?
            if let Some(&function_index) = model.functions.get(key.borrow().deref()) {
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

    fn index_get_array(&mut self, array: Reference<Vec<Object>>, index: &Object) -> Result<(), RuntimeError> {
        match index {
            Object::Integer(i) => {
                let array_index = *i;
                if array_index < 0 || array_index >= array.borrow().deref().len() as i64 {
                    return Err(RuntimeError::new("index out of range", self.last_position()));
                };

                self.push(array.borrow().deref().get(array_index as usize).unwrap().clone());
            },
            _ => {
                return Err(RuntimeError::new("can not get array with object index", self.last_position()));
            }
        };

        Ok(())
    }

    fn index_get(&mut self) -> Result<(), RuntimeError> {
        let index = self.pop().unwrap();
        let instance = self.pop().unwrap();

        match instance {
            Object::Model(model_index) => self.index_get_model(model_index, &index)?,
            Object::Instance(model_instance) => self.index_get_model_instance(model_instance, &index)?,
            Object::Array(array) => self.index_get_array(array, &index)?,
            Object::NativeInstance(instance) => {
                let instance_copy = instance.clone();
                self.push(instance.borrow_mut().index_get(instance_copy, &index)?);
            }
            _ => {
                return Err(RuntimeError::new("this object's instance get did not implemented yet", self.last_position()));
            }
        };

        Ok(())
    }

    fn instance_set(&mut self) -> Result<(), RuntimeError> {
        let index = self.pop().unwrap();
        let instance = self.pop().unwrap();

        match instance {
            Object::Instance(model_instance) => self.index_set_model_instance(model_instance, &index)?,
            Object::NativeInstance(instance) => {
                let instance_copy = instance.clone();
                instance.borrow_mut().instance_set(instance_copy, index.as_reference_string().borrow().deref(), self.top())?
            },
            _ => {
                return Err(RuntimeError::new("this object's instance set did not implemented yet", self.last_position()));
            }
        };

        Ok(())
    }

    fn index_set_model_instance_by_index(&mut self, model_instance: Reference<ModelInstance>, index: usize) -> Result<(), RuntimeError> {
        if let Some(object) = model_instance.borrow_mut().properties.get_mut(index as usize) {
            *object = self.top();
            Ok(())
        } else {
            Err(RuntimeError::new("index does not exists", self.last_position()))
        }
    }

    fn index_set_model_instance(&mut self, model_instance: Reference<ModelInstance>, index: &Object) -> Result<(), RuntimeError> {
        if let Object::String(key) = &index {
            let model = self.program.models.get(model_instance.borrow().deref().model_index).unwrap();

            // have property?
            if let Some(&property_index) = model.property_indices.get(key.borrow().deref()) {
                self.index_set_model_instance_by_index(model_instance, property_index)?;
            };

        } else if let Object::Integer(i) = &index {
            self.index_set_model_instance_by_index(model_instance, *i as usize)?;
        }

        Ok(())
    }

    fn index_set_array(&mut self, array: Reference<Vec<Object>>, index: &Object) -> Result<(), RuntimeError> {
        match index {
            Object::Integer(i) => {
                if *i < 0 || *i >= array.borrow().deref().len() as i64 {
                    return Err(RuntimeError::new("index out of range", self.last_position()));
                };

                array.borrow_mut()[*i as usize] = self.top();
            },
            _ => {
                return Err(RuntimeError::new("can not get array with object index", self.last_position()));
            }
        };

        Ok(())
    }

    fn index_set(&mut self) -> Result<(), RuntimeError> {
        let index = self.pop().unwrap();
        let instance = self.pop().unwrap();

        match instance {
            Object::Instance(model_instance) => self.index_set_model_instance(model_instance, &index)?,
            Object::NativeInstance(instance) => {
                let instance_copy = instance.clone();
                instance.borrow_mut().index_set(instance_copy, &index, self.top())?
            },
            Object::Array(array) => self.index_set_array(array, &index)?,
            _ => {
                return Err(RuntimeError::new("this object's instance set did not implemented yet", self.last_position()));
            }
        };

        Ok(())
    }

    fn push_array(&mut self, value_count: usize) -> Result<(), RuntimeError> {
        let mut array = Vec::<Object>::new();

        for _ in 0..value_count {
            array.push(self.pop().unwrap());
        }

        array.reverse();

        self.push(Object::Array(make_reference(array)));

        Ok(())
    }

    fn binary_operation(&mut self, operand: usize) -> Result<(), RuntimeError> {
        let right = self.pop().unwrap();
        let left = self.pop().unwrap();

        self.binary_operation_with_parameters(&left, &right, operand)?;

        Ok(())
    }

    fn internal_step(&mut self) -> Result<(), RuntimeError> {
        let instruction = self.current_instruction();
        let opcode = instruction.opcode();

        self.current_frame_as_mut().program_counter += 1;

        match opcode {
            OpCode::Pop => { self.stack.pop_back(); },
            OpCode::PushConstant => {
                let constant = self.program.constants[instruction.operand() as usize].clone();
                self.push(constant);
            },
            OpCode::Return => { self.pop_frame(); },

            OpCode::LocalGet => self.push(self.current_frame().locals.get(instruction.operand() as usize).unwrap().clone()),
            OpCode::LocalSet => { self.current_frame_as_mut().locals[instruction.operand() as usize] = self.top(); },
            OpCode::LocalInit => { self.current_frame_as_mut().locals[instruction.operand() as usize] = self.pop().unwrap(); },

            OpCode::ContextGet => self.push(self.locals.get(instruction.operand() as usize).unwrap().clone()),
            OpCode::ContextSet => { self.locals[instruction.operand() as usize] = self.top(); },

            OpCode::GlobalGet => {
                let global_object_option = if let Some(Object::String(global_name)) = self.program.constants.get(instruction.operand() as usize) {
                    if let Some(object) = self.globals.get(global_name.borrow().deref()) {
                       Some(object.clone())
                    } else {
                        return Err(RuntimeError::new("global not found", self.last_position()));
                    }
                } else {
                    None
                };

                if let Some(global_object) = global_object_option {
                    self.push(global_object);
                };

            },
            OpCode::GlobalSet => {
                if let Some(Object::String(global_name)) = self.program.constants.get(instruction.operand() as usize) {
                    if let Some(object) = self.globals.get_mut(global_name.borrow().deref()) {
                        *object = self.stack.back().unwrap().clone();
                    } else {
                        return Err(RuntimeError::new("global not found", self.last_position()));
                    }
                }
            },
            OpCode::InstanceGet => self.instance_get()?,
            OpCode::InstanceSet => self.instance_set()?,
            OpCode::IndexGet => self.index_get()?,
            OpCode::IndexSet => self.index_set()?,
            OpCode::Call => self.execute_call_opcode(instruction.operand() as usize)?,
            OpCode::Array => self.push_array(instruction.operand() as usize)?,
            OpCode::Operation => self.binary_operation(instruction.operand() as usize)?,
            OpCode::Not => {
                let value = Object::Boolean(!self.pop().unwrap().to_bool());
                self.push(value);
            },
            OpCode::Negative => {
                let target = self.pop().unwrap();
                self.push(self.negative_operation(&target)?)
            },
            OpCode::Jump => { self.current_frame_as_mut().program_counter = instruction.operand() as usize; },
            OpCode::JumpIf => {
                let object = self.pop().unwrap();
                if object.to_bool() {
                    self.current_frame_as_mut().program_counter = instruction.operand() as usize;
                };
            },
            OpCode::ForNext => { self.for_next(instruction.operand() as usize)?; },
            OpCode::Iterate => { self.iterate(instruction.operand() as usize); },
            _ => {
                // not implemented
            }
        }

        Ok(())
    }
}

// helpers
fn make_instance_call_parameters(object: Object, parameters: &[ Object ]) -> Vec<Object> {
    let mut new_parameters = vec![ object ];
    new_parameters.extend_from_slice(parameters);
    new_parameters
}