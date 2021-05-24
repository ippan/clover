use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub type Reference<T> = Rc<RefCell<T>>;

#[derive(Debug, PartialEq)]
pub struct Instance {
    model_index: usize,
    properties: HashMap<String, Object>
}

#[derive(Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,

    Function(usize),
    InstanceFunction(Box<Object>, usize),
    NativeFunction(usize),
    InstanceNativeFunction(Box<Object>, usize),

    Model(usize),
    NativeModel(usize),

    // reference types
    Instance(Reference<Instance>),
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
            _ => struct_format.field("Unknown", &"Unknown".to_string())
        }.finish()
    }
}