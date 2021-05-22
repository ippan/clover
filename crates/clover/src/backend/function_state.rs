use std::collections::HashMap;

use crate::runtime::opcode::Instruction;

pub type Scope = HashMap<String, usize>;

#[derive(Debug, Clone)]
struct FunctionState {
    is_instance: bool,
    parameter_count: u16,
    local_count: usize,
    scopes: Vec<Scope>,
    instructions: Vec<Instruction>
}

impl FunctionState {
    fn find_local(&self, name: &str) -> Option<usize> {
        for scope in self.scopes.iter().rev() {
            if let Some(&index) = scope.get(name) {
                return Some(index);
            };
        }

        None
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn define_local(&mut self, name: &str) -> usize {
        if let Some(scope) = self.scopes.last_mut() {
            let index = self.local_count;
            scope.insert(name.to_string(), index);
            self.local_count += 1;
            index
        } else {
            panic!("must not be reach here");
        }
    }
}
