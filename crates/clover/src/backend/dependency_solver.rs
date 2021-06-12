use crate::intermediate::ast::Document;
use std::collections::{HashMap, HashSet};

pub struct DependencySolver {
    dependencies: HashMap<String, u32>,
    references: HashMap<String, Vec<String>>,
    unsolved: HashSet<String>
}

impl DependencySolver {
    pub fn new() -> DependencySolver {
        DependencySolver {
            dependencies: HashMap::new(),
            references: HashMap::new(),
            unsolved: HashSet::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.dependencies.is_empty()
    }

    pub fn get_cycle_reference_list(&self) -> Vec<String> {
        let mut list = Vec::new();

        for (filename, _) in self.dependencies.iter() {
            list.push(filename.clone());
        };

        list
    }

    pub fn get_unsolved_filename(&self) -> Option<String> {
        if let Some(filename) = self.unsolved.iter().next() {
            Some(filename.clone())
        } else {
            None
        }
    }

    pub fn get_next_no_dependency_filename(&self) -> Option<String> {
        for (filename, &dependency) in self.dependencies.iter() {
            if dependency == 0 {
                return Some(filename.clone());
            };
        };

        None
    }

    pub fn set_loaded(&mut self, filename: &str) {
        self.dependencies.remove(filename);

        if let Some(source_list) = self.references.remove(filename) {
            for source in source_list.iter() {
                self.decrease_dependency(source);
            }
        }
    }

    pub fn solve(&mut self, document: &Document, loaded_assemblies: &HashSet<String>) {
        if loaded_assemblies.contains(&document.filename) || self.dependencies.contains_key(&document.filename) {
            return;
        }

        self.add_dependencies(document, loaded_assemblies);

        if self.unsolved.contains(&document.filename) {
            self.unsolved.remove(&document.filename);
        };
    }

    fn add_reference(&mut self, source: &str, target: &str) {
        if !self.references.contains_key(target) {
            self.references.insert(target.to_string(), Vec::new());
        };

        let list = self.references.get_mut(target).unwrap();
        list.push(source.to_string());
    }

    fn increase_dependency(&mut self, source: &str) {
        if let Some(count) = self.dependencies.get_mut(source) {
            *count += 1;
        } else {
            self.dependencies.insert(source.to_string(), 1);
        };
    }

    fn decrease_dependency(&mut self, source: &str) {
        if let Some(count) = self.dependencies.get_mut(source) {
            *count -= 1;
        }
    }

    fn add_dependencies(&mut self, document: &Document, loaded_assemblies: &HashSet<String>) {
        self.dependencies.insert(document.filename.clone(), 0);

        for dependency_filename in document.get_dependencies().iter() {
            if !loaded_assemblies.contains(dependency_filename) {
                self.increase_dependency(&document.filename);
                self.add_reference(&document.filename, dependency_filename);

                if !self.dependencies.contains_key(dependency_filename) {
                    self.unsolved.insert(dependency_filename.to_string());
                }
            };
        };
    }
}