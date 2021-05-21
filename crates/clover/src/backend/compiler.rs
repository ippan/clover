use crate::runtime::assembly::Assembly;
use crate::intermediate::{CompileErrorList, TokenValue};
use crate::runtime::object::Object;
use std::collections::HashMap;
use crate::runtime::opcode::Instruction;
use crate::intermediate::ast::{Document, Definition};
use crate::runtime::program::Assemblies;

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
    pub assembly_index: usize,
    pub constants: Vec<Object>,
    pub locals: Scope,
    pub errors: CompileErrorList
}

impl CompilerState {
    fn define_local(&mut self, name: &str) {
        let index = self.locals.len();
        self.locals.insert(name.to_string(), index);
    }

    fn compile_definition(&mut self, definition: &Definition) {
        match definition {
            Definition::Local(local_definition) => {
                for token in local_definition.variables.iter() {
                    if let TokenValue::Identifier(name) = &token.value {
                        self.define_local(name);
                    }
                };
            },
            _ => {}
        }
    }

    fn include_dependencies(&mut self, assemblies: &Assemblies) {

    }

    fn compile(&mut self, document: &Document) {
        for definition in document.definitions.iter() {
            self.compile_definition(definition);
        }
    }

    fn to_assembly(&self, filename: &str) -> Result<Assembly, CompileErrorList> {
        if self.errors.is_empty() {
            let mut assembly = Assembly {
                filename: filename.to_string(),
                local_count: self.locals.len(),
                constants: self.constants.clone(),
                index: self.assembly_index
            };
            Ok(assembly)
        } else {
            Err(self.errors.clone())
        }
    }
}

pub fn compile_document(document: &Document, assembly_index: usize, assemblies: &Assemblies) -> Result<Assembly, CompileErrorList> {

    let mut state = CompilerState {
        assembly_index,
        constants: Vec::new(),
        locals: Scope::new(),
        errors: CompileErrorList::new(&document.filename)
    };

    state.include_dependencies(assemblies);
    state.compile(document);

    state.to_assembly(document.filename.as_str())
}

pub fn compile() {

}