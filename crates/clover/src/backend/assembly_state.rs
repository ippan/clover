use crate::runtime::object::Object;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AssemblyState {
    pub filename: String,
    pub local_indices: HashMap<String, usize>
}

impl AssemblyState {
    pub fn new(filename: &str) -> AssemblyState {
        AssemblyState {
            filename: filename.to_string(),
            local_indices: HashMap::new()
        }
    }

    // add a local with index, if local already exists, return false
    pub fn add_local(&mut self, name: &str, index: usize) -> bool {
        if self.local_indices.contains_key(name) {
            false
        } else {
            self.local_indices.insert(name.to_string(), index);
            true
        }
    }
}