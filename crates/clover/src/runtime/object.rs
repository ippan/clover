use std::rc::Rc;
use std::collections::HashMap;

// stack size of object is 56 (HashMap size)
#[derive(Clone)]
pub enum Object {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Map(HashMap<String, Rc<Object>>),
    Array(Vec<Rc<Object>>),
    Closure(ClosureData),
    NativeFunction(NativeFunctionData)
}

pub type Slot = Rc<Object>;

#[derive(Clone)]
pub struct ClosureData {

}

#[derive(Clone)]
pub struct NativeFunctionData {

}