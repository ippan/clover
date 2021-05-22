use crate::intermediate::ast::Document;
use std::collections::{HashMap, HashSet};
use crate::runtime::program::Assemblies;

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

    pub fn get_unsolved_filename(&self) -> Option<String> {
        if let Some(filename) = self.unsolved.iter().next() {
            Some(filename.clone())
        } else {
            None
        }
    }

    pub fn solve(&mut self, document: &Document, assemblies: &Assemblies) {
        if assemblies.exists(&document.filename) {
            return;
        }

        self.add_dependencies(document, assemblies);

        if self.unsolved.contains(&document.filename) {
            self.unsolved.remove(&document.filename);
        };

        if self.references.contains_key(&document.filename) {

        }

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

    fn add_dependencies(&mut self, document: &Document, assemblies: &Assemblies) {
        self.dependencies.insert(document.filename.clone(), 0);

        for dependency_filename in document.get_dependencies().iter() {
            if !assemblies.exists(dependency_filename) {
                self.increase_dependency(&document.filename);
                self.add_reference(&document.filename, dependency_filename);

                if !self.dependencies.contains_key(dependency_filename) {
                    self.unsolved.insert(dependency_filename.to_string());
                }
            };
        };
    }
}