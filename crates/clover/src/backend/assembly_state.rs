use crate::runtime::object::Object;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AssemblyState {
    pub filename: String,

    // key is public name, value is constant index
    pub public_indices: HashMap<String, usize>
}

impl AssemblyState {
    pub fn new(filename: &str) -> AssemblyState {
        AssemblyState {
            filename: filename.to_string(),
            public_indices: HashMap::new()
        }
    }
}