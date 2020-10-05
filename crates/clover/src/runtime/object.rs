use std::rc::Rc;
use std::collections::HashMap;

pub enum Object {
    Integer(i64),
    Float(i64),
    String(String),
    Boolean(bool),
    Null,
    Map(HashMap<String, Object>),
    Closure, // TODO : add closure data
    NativeFunction // TODO : add native function data
}