use crate::runtime::object::Object;

#[derive(Debug)]
pub struct Function {
    pub parameter_count: u16,
    pub local_variable_count: u16
}

#[derive(Debug)]
pub struct Assembly {
    pub filename: String,
    pub index: usize,
    pub local_count: usize,
    pub constants: Vec<Object>
}