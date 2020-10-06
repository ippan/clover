use std::collections::{HashMap, LinkedList};
use crate::runtime::object::{Slot, Object};
use crate::runtime::assembly::Assembly;

pub struct Frame {
    pub locals: Vec<Slot>,
    pub frees: Vec<Slot>
}

pub struct State {
    globals: HashMap<String, Slot>,
    stack: LinkedList<Slot>,
    frames: LinkedList<Frame>,
    assemblies: Vec<Assembly>
}

impl State {
    pub fn add_assembly(&mut self, assembly: Assembly) -> usize {
        let index = self.assemblies.len();
        self.assemblies.push(assembly);
        index
    }

    pub fn execute(&mut self, assembly_index: usize) {

    }

    pub fn execute_closure(&mut self, closure: &Object) {


    }

}