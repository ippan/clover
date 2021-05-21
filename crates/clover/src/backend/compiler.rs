use crate::runtime::assembly::Assembly;
use crate::intermediate::CompileErrorList;
use crate::runtime::object::Object;
use std::collections::HashMap;
use crate::runtime::opcode::Instruction;
use crate::intermediate::ast::Document;

const MAX_LOCALS: usize = 65536;

type Scope = HashMap<String, usize>;

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

pub struct CompilerState {
    pub assembly: Assembly,
    pub constants: Vec<Object>

}




pub fn compile_document(document: &Document, assembly_index: u32) -> Result<Assembly, CompileErrorList> {




    Ok(Assembly {
        filename: document.filename.clone()
    })
}

pub fn compile() {

}