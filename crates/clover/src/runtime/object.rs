use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use crate::runtime::state::State;
use crate::runtime::program::RuntimeError;
use std::ops::Deref;
use crate::debug::Position;

pub type Reference<T> = Rc<RefCell<T>>;

pub fn make_reference<T>(object: T) -> Reference<T> {
    Rc::new(RefCell::new(object))
}

pub type NativeFunction = fn(&mut State, &[Object]) -> Result<Object, RuntimeError>;

#[derive(Debug)]
pub struct ModelInstance {
    pub model_index: usize,
    pub properties: Vec<Object>
}

pub trait NativeModel {
    // model constructor
    fn call(&mut self, state: &mut State, _parameters: &[Object]) -> Result<Object, RuntimeError> { Err(RuntimeError::new("this native model do not have constructor", state.last_position())) }

    fn model_get(&self, key: &str) -> Result<Object, RuntimeError> { Err(RuntimeError::new(&format!("this native do not have property [{}]", key), Position::none())) }
}

pub trait NativeModelInstance {
    fn index_get(&self, this: Reference<dyn NativeModelInstance>, index: &Object) -> Result<Object, RuntimeError>;
    fn index_set(&mut self, this: Reference<dyn NativeModelInstance>, index: &Object, value: Object) -> Result<(), RuntimeError>;
    fn instance_get(&self, this: Reference<dyn NativeModelInstance>, key: &str) -> Result<Object, RuntimeError>;
    fn instance_set(&mut self, this: Reference<dyn NativeModelInstance>, key: &str, value: Object) -> Result<(), RuntimeError>;

    fn call(&mut self, this: Reference<dyn NativeModelInstance>, state: &mut State, key: &str, parameters: &[Object]) ->Result<Object, RuntimeError>;

    fn raw_get_integer(&self, _key: &str) -> Option<i64> { None }
    fn raw_get_float(&self, _key: &str) -> Option<f64> { None }
    fn raw_get_boolean(&self, _key: &str) -> Option<bool> { None }
    fn raw_get_byte_array(&self, _key: &str) -> Option<&[u8]> { None }
}

pub fn ensure_parameters_length(parameters: &[Object], length: usize) -> Result<(), RuntimeError> {
    if parameters.len() == length {
        Ok(())
    } else {
        Err(RuntimeError::new(&format!("need {} parameters, got {}", length, parameters.len()), Position::none()))
    }
}

pub enum Object {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,

    Function(usize),
    InstanceFunction(Box<Object>, usize),
    NativeFunction(NativeFunction),
    InstanceNativeFunction(Reference<dyn NativeModelInstance>, String),

    Model(usize),
    NativeModel(usize),

    // reference types
    String(Reference<String>),
    Instance(Reference<ModelInstance>),
    NativeInstance(Reference<dyn NativeModelInstance>),

    Array(Reference<Vec<Object>>),
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut struct_format = f.debug_struct("Object");

        match self {
            Object::Integer(value) => struct_format.field("Integer", value),
            Object::Float(value) => struct_format.field("Float", value),
            Object::String(value) => struct_format.field("String", value),
            Object::Boolean(value) => struct_format.field("Boolean", value),
            Object::Null => struct_format.field("Null", &"Null".to_string()),

            Object::Model(value) => struct_format.field("Model", value),
            Object::Instance(value) => struct_format.field("Instance", value),
            Object::Array(array) => struct_format.field("Array", array.deref()),
            _ => struct_format.field("Unknown", &"Unknown".to_string())
        }.finish()
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        match self {
            Object::Integer(value) => Object::Integer(*value),
            Object::Float(value) => Object::Float(*value),
            Object::String(value) => Object::String(value.clone()),
            Object::Boolean(value) => Object::Boolean(*value),
            Object::Null => Object::Null,
            Object::Function(index) => Object::Function(*index),
            Object::InstanceFunction(this, index) => Object::InstanceFunction(this.clone(), *index),
            Object::NativeFunction(function) => Object::NativeFunction(*function),
            Object::InstanceNativeFunction(this, function_name) => Object::InstanceNativeFunction(this.clone(), function_name.clone()),
            Object::Model(index) => Object::Model(*index),
            Object::NativeModel(index) => Object::NativeModel(*index),
            Object::Instance(instance) => Object::Instance(instance.clone()),
            Object::NativeInstance(instance) => Object::NativeInstance(instance.clone()),
            Object::Array(value) => Object::Array(value.clone())
        }
    }
}

impl Object {
    pub fn is_string(&self) -> bool {
        matches!(self, Object::String(_))
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Object::Boolean(value) => *value,
            Object::Null => false,
            _ => true
        }
    }

    pub fn as_reference_string(&self) -> Reference<String> {
        if let Object::String(value) = self {
            value.clone()
        } else {
            make_reference("".to_string())
        }
    }

    pub fn is_null(&self) -> bool { matches!(self, Object::Null) }

    pub fn integer_value(&self) -> Result<i64, RuntimeError> {
        if let Object::Integer(value) = self {
            Ok(*value)
        } else {
            Err(RuntimeError::new("value is not a integer", Position::none()))
        }
    }

    pub fn float_value(&self) -> Result<f64, RuntimeError> {
        if let Object::Float(value) = self {
            Ok(*value)
        } else if let Object::Integer(value) = self {
            Ok(*value as f64)
        } else {
            Err(RuntimeError::new("value is not a float", Position::none()))
        }
    }

    pub fn string_value(&self) -> Result<String, RuntimeError> {
        if let Object::String(value) = self {
            Ok(value.borrow().deref().clone())
        } else {
            Err(RuntimeError::new("value is not a string", Position::none()))
        }
    }

    pub fn native_instance_value(&self) -> Result<Reference<dyn NativeModelInstance>, RuntimeError> {
        if let Object::NativeInstance(instance) = self {
            Ok(instance.clone())
        } else {
            Err(RuntimeError::new("value is not a native instance", Position::none()))
        }
    }

}

fn objects_to_string(objects: &Vec<Object>) -> String {
    objects.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(", ")
}

impl ToString for Object {
    fn to_string(&self) -> String {
        match self {
            Object::Integer(value) => value.to_string(),
            Object::Float(value) => value.to_string(),
            Object::String(value) => value.borrow().deref().clone(),
            Object::Boolean(value) => value.to_string(),
            Object::Null => "null".to_string(),

            Object::Model(index) => "{ (".to_string() + index.to_string().as_str() + ") }",
            Object::Instance(instance) => "{ (".to_string() + instance.borrow().deref().model_index.to_string().as_str() + ") " + objects_to_string(&instance.borrow().deref().properties).as_str() + " }",
            Object::Array(array) => "[ ".to_string() + objects_to_string(array.borrow().deref()).as_str() + " ]",
            _ => "Unknown".to_string()
        }
    }
}