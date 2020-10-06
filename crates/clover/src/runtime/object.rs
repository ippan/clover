use std::rc::Rc;
use std::collections::HashMap;

// stack size of object is 56 (HashMap size)
#[derive(Debug, Clone)]
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
    NativeFunction(Rc<NativeFunctionData>)
}

pub type Slot = Rc<Object>;

#[derive(Debug, Clone)]
pub struct ClosureData {

}

#[derive(Debug, Clone)]
pub struct NativeFunctionData {

}