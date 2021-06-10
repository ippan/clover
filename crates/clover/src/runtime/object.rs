use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use crate::runtime::state::State;
use crate::runtime::program::RuntimeError;


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
    // register all native function in this function
    fn register(&mut self, state: &mut State);
    // direct call to model
    fn model_get(&self, index: Object) -> Result<Object, RuntimeError>;
}

pub trait NativeModelInstance {
    fn instance_get(&self, index: Object) -> Result<Object, RuntimeError>;
    fn instance_set(&mut self, index: Object, value: Object) -> Result<(), RuntimeError>;
}

pub enum Object {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,

    Function(usize),
    InstanceFunction(Box<Object>, usize),
    NativeFunction(NativeFunction),
    InstanceNativeFunction(Box<Object>, NativeFunction),

    Model(usize),
    NativeModel(usize),

    // reference types
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
            Object::InstanceNativeFunction(this, function) => Object::InstanceNativeFunction(this.clone(), *function),
            Object::Model(index) => Object::Model(*index),
            Object::NativeModel(index) => Object::Model(*index),
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
}

impl ToString for Object {
    fn to_string(&self) -> String {
        match self {
            Object::Integer(value) => value.to_string(),
            Object::Float(value) => value.to_string(),
            Object::String(value) => value.clone(),
            Object::Boolean(value) => value.to_string(),
            Object::Null => "null".to_string(),

            _ => "Unknown".to_string()
        }
    }
}