use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

use crate::backend::dependency_solver::DependencySolver;
use crate::backend::function_state::Scope;
use crate::frontend::parser::parse;
use crate::intermediate::{CompileError, CompileErrorList, Position, Token, TokenValue};
use crate::intermediate::ast::{Definition, Document, IncludeDefinition};
use crate::runtime::object::Object;
use crate::runtime::opcode::Instruction;
use crate::runtime::program::Program;
use crate::backend::assembly_state::AssemblyState;

const MAX_LOCALS: usize = 65536;

#[derive(Debug)]
pub struct Context {
    pub constants: Vec<Object>,
    pub local_count: usize,
    pub assemblies: HashMap<String, AssemblyState>
}

impl Context {
    pub fn new() -> Context {
        Context {
            constants: Vec::new(),
            local_count: 0,
            assemblies: HashMap::new()
        }
    }

    pub fn find_assembly(&self, name: &str) -> Option<&AssemblyState> {
        if let Some(assembly_state) = self.assemblies.get(name) {
            Some(assembly_state)
        } else {
            None
        }
    }

    pub fn assembly_exists(&self, name: &str) -> bool {
        self.find_assembly(name).is_some()
    }

    pub fn add_assembly(&mut self, assembly: AssemblyState) {
        self.assemblies.insert(assembly.filename.clone(), assembly);
    }

    pub fn get_loaded_assemblies(&self) -> HashSet<String> {
        let mut loaded_assemblies = HashSet::new();

        for (filename, _) in self.assemblies.iter() {
            loaded_assemblies.insert(filename.clone());
        };

        loaded_assemblies
    }

    pub fn to_program(&self) -> Program {
        Program {

        }
    }
}

#[derive(Debug, Clone)]
pub struct CompilerState {
    pub assembly_state: AssemblyState,
    pub locals: Scope,
    pub errors: CompileErrorList
}

impl CompilerState {
    fn define_local(&mut self, name: &str) {
        let index = self.locals.len();
        self.locals.insert(name.to_string(), index);
    }

    fn compile_include_definition(&mut self, include_definition: IncludeDefinition) {

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

    fn compile(&mut self, document: &Document) {
        for definition in document.definitions.iter() {
            self.compile_definition(definition);
        }
    }
}

pub fn compile_document(document: &Document, context: &mut Context) -> Result<(), CompileErrorList> {

    let mut state = CompilerState {
        assembly_state: AssemblyState::new(&document.filename),
        locals: Scope::new(),
        errors: CompileErrorList::new(&document.filename)
    };

    state.compile(document);


    Ok(())
}

pub fn compile_to(context: &mut Context, source: &str, filename: &str) -> Result<(), CompileErrorList> {
    let mut documents: HashMap<String, Document> = HashMap::new();

    let mut dependency_solver = DependencySolver::new();

    let document = parse(&source, filename)?;

    let loaded_assemblies = context.get_loaded_assemblies();

    dependency_solver.solve(&document, &loaded_assemblies);

    documents.insert(document.filename.clone(), document);

    while let Some(dependency_filename) = dependency_solver.get_unsolved_filename() {
        let dependency_source = load_file(&dependency_filename)?;
        let dependency_document = parse(&dependency_source, &dependency_filename)?;

        dependency_solver.solve(&dependency_document, &loaded_assemblies);
        documents.insert(dependency_filename, dependency_document);
    }

    while let Some(filename_to_compile) = dependency_solver.get_next_no_dependency_filename() {
        let document_to_compile = documents.get(&filename_to_compile).unwrap();

        compile_document(document_to_compile, context)?;

        dependency_solver.set_loaded(&filename_to_compile);
    }

    if !dependency_solver.is_empty() {
        let mut errors = CompileErrorList::new(filename);
        let cycle_filenames = dependency_solver.get_cycle_reference_list().join(", ");

        errors.push_error(Token::new(TokenValue::None, Position::none()), &format!("there may have cycle reference in this files [{}]", cycle_filenames));
        return Err(errors);
    };

    Ok(())
}

fn load_file(filename: &str) -> Result<String, CompileErrorList> {
    if let Ok(source) = read_to_string(filename) {
        Ok(source)
    } else {
        let mut errors = CompileErrorList::new(filename);
        errors.push_error(Token::new(TokenValue::None, Position::none()), "can not open source file");
        Err(errors)
    }
}

pub fn compile_file(filename: &str) -> Result<Program, CompileErrorList> {
    let source = load_file(filename)?;

    compile(&source, filename)
}

pub fn compile(source: &str, filename: &str) -> Result<Program, CompileErrorList> {
    let mut context = Context::new();

    compile_to(&mut context, &source, filename)?;

    Ok(context.to_program())
}