use std::rc::Rc;
use std::collections::HashMap;
use crate::runtime::NativeFunction;
use std::fmt;

// stack size of object is 56 (HashMap size)
#[derive(Clone)]
pub enum Object {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,

    // reference types
    Map(Rc<HashMap<String, Slot>>),
    Array(Rc<Vec<Slot>>),
    Closure(Rc<ClosureData>),
    NativeFunction(NativeFunction)
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
            Object::Map(value) => struct_format.field("Map", value),
            Object::Array(value) => struct_format.field("Array", value),
            Object::Closure(value) => struct_format.field("Closure", value),
            Object::NativeFunction(_) => struct_format.field("NativeFunction", &"".to_string())
        }.finish()
    }
}

pub type Slot = Rc<Object>;

#[derive(Debug, Clone)]
pub struct ClosureData {

}