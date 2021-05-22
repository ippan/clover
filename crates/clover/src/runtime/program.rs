use std::collections::HashMap;
use crate::runtime::assembly::Assembly;
use std::fs::OpenOptions;
use std::slice::Iter;

#[derive(Debug)]
pub struct Assemblies {
    indices: HashMap<String, usize>,
    assemblies: Vec<Assembly>
}

impl Assemblies {
    pub fn new() -> Assemblies {
        Assemblies {
            indices: HashMap::new(),
            assemblies: Vec::new()
        }
    }

    pub fn find(&self, name: &str) -> Option<usize> {
        if let Some(&index) = self.indices.get(name) {
            Some(index)
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&Assembly> {
        self.assemblies.get(index)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.find(name).is_some()
    }

    pub fn iter(&self) -> Iter<Assembly> {
        self.assemblies.iter()
    }
}

#[derive(Debug)]
pub struct Program {
    pub assemblies: Assemblies
}