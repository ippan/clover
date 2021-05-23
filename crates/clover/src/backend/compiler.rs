use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

use crate::backend::dependency_solver::DependencySolver;
use crate::backend::function_state::Scope;
use crate::frontend::parser::parse;
use crate::intermediate::{CompileError, CompileErrorList, Position, Token, TokenValue, Positions};
use crate::intermediate::ast::{Definition, Document, IncludeDefinition, ModelDefinition, FunctionDefinition, ImplementDefinition, ApplyDefinition, Statement};
use crate::runtime::object::Object;
use crate::runtime::opcode::Instruction;
use crate::runtime::program::{Program, Model, Function};
use crate::backend::assembly_state::AssemblyState;
use std::os::raw::c_long;
use crate::runtime::assembly_information::{FileInfo, DebugInfo};

const MAX_LOCALS: usize = 65536;

#[derive(Debug)]
pub struct CompilerContext {
    pub models: Vec<Model>,
    pub functions: Vec<Function>,
    pub constants: Vec<Object>,

    pub local_count: usize,
    pub assemblies: HashMap<String, AssemblyState>,
    pub local_values: HashMap<usize, usize>,

    pub file_info: FileInfo,
    pub debug_info: DebugInfo
}

impl CompilerContext {
    pub fn new() -> CompilerContext {
        CompilerContext {
            models: Vec::new(),
            functions: Vec::new(),
            constants: Vec::new(),
            local_count: 0,
            assemblies: HashMap::new(),
            local_values: HashMap::new(),

            file_info: FileInfo::new(),
            debug_info: DebugInfo::new()
        }
    }

    pub fn get_local_value(&self, local_index: usize) -> Option<Object> {
        if !self.local_values.contains_key(&local_index) {
            return None;
        };

        let &constant_index = self.local_values.get(&local_index).unwrap();

        if let Some(object) = self.constants.get(constant_index) {
            Some(object.clone())
        } else {
            None
        }
    }

    pub fn add_model(&mut self, model: Model) -> usize {
        let index = self.models.len();
        self.models.push(model);
        index
    }

    pub fn add_function(&mut self, function: Function, positions: Positions, name: &str, assembly_index: usize) -> usize {
        let index = self.functions.len();
        self.functions.push(function);
        self.debug_info.functions.push(positions);
        self.file_info.function_names.push(name.to_string());
        self.file_info.function_files.push(assembly_index);
        index
    }

    pub fn add_constant(&mut self, object: Object) -> usize {
        let index = self.constants.len();
        self.constants.push(object);
        index
    }

    // find constant index by include definition
    pub fn find_constant_index_by_include(&self, assembly_name: &str, public_name: &str) -> Option<usize> {
        if let Some(assembly_state) = self.assemblies.get(assembly_name) {
            if let Some(&index) = assembly_state.public_indices.get(public_name) {
                return Some(index);
            };
        };

        None
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

    pub fn add_assembly(&mut self, assembly: AssemblyState) -> usize {
        let index = self.assemblies.len();
        self.assemblies.insert(assembly.filename.clone(), assembly);
        index
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
            models: Vec::new(),
            functions: Vec::new(),
            constants: Vec::new(),

            local_count: 0,
            local_values: HashMap::new(),

            entry_point: 0,

            file_info: None,
            debug_info: None
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
    fn define_local(&mut self, context: &mut CompilerContext, name: &str) -> Option<usize> {
        if self.locals.contains_key(name) {
            None
        } else {
            let index = context.local_count;
            self.locals.insert(name.to_string(), index);
            context.local_count += 1;
            Some(index)
        }
    }

    fn define_local_by_identifier(&mut self, context: &mut CompilerContext, token: &Token) -> Option<usize> {
        if let TokenValue::Identifier(identifier) = &token.value {
            self.define_local(context, identifier)
        } else {
            self.errors.push_error(token, "variable already exists");
            None
        }
    }

    fn compile_statement(&mut self, context: &mut CompilerContext, function: &mut Function, positions: &mut Positions, statement: &Statement) {
        match statement {
            Statement::Return(return_statement) => {},
            _ => {}
        }
    }

    fn compile_include_definition(&mut self, context: &mut CompilerContext, include_definition: &IncludeDefinition) {
        for (i, alias) in include_definition.aliases.iter().enumerate() {
            if let Some(index) = self.define_local_by_identifier(context, alias) {
                let public_name = include_definition.public_names.get(i).unwrap();

                if let Some(constant_index) = context.find_constant_index_by_include(&include_definition.filename.value.to_string(), &public_name.value.to_string()) {
                    context.local_values.insert(index, constant_index);
                }
            }
        }
    }

    // return model constant index
    fn compile_model_definition(&mut self, context: &mut CompilerContext, model_definition: &ModelDefinition) -> usize {
        let mut model = Model::new();

        for token in model_definition.properties.iter() {
            model.add_property(&token.value.to_string());
        }

        let model_index = context.add_model(model);
        let constant_index = context.add_constant(Object::Model(model_index));

        if let Some(local_index) = self.define_local_by_identifier(context, &model_definition.name) {
            context.local_values.insert(local_index, constant_index);
        };

        context.file_info.model_files.push(self.assembly_state.index);
        context.file_info.model_names.push(model_definition.name.value.to_string());

        constant_index
    }

    fn compile_public_model_definition(&mut self, context: &mut CompilerContext, model_definition: &ModelDefinition) {
        let constant_index = self.compile_model_definition(context, model_definition);

        self.assembly_state.public_indices.insert(model_definition.name.value.to_string(), constant_index);
    }

    fn compile_function_definition_base(&mut self, context: &mut CompilerContext, function_definition: &FunctionDefinition) -> (Function, Positions) {
        let mut function = Function::new();
        let mut positions: Positions = Positions::new();

        for statement in function_definition.body.iter() {
            self.compile_statement(context, &mut function, &mut positions, statement);
        }

        (function, positions)
    }

    // return constant index
    fn compile_function_definition(&mut self, context: &mut CompilerContext, function_definition: &FunctionDefinition) -> usize {
        let (function, positions) = self.compile_function_definition_base(context, function_definition);

        // can not have instance function here
        if function.is_instance {
            self.errors.push_error(&function_definition.name, "instance function can inside implement block only");
            0
        } else {
            let function_index = context.add_function(function, positions, &function_definition.name.value.to_string(), self.assembly_state.index);
            let constant_index = context.add_constant(Object::Function(function_index));

            if let Some(local_index) = self.define_local_by_identifier(context, &function_definition.name) {
                context.local_values.insert(local_index, constant_index);
            };

            constant_index
        }
    }

    fn compile_public_function_definition(&mut self, context: &mut CompilerContext, function_definition: &FunctionDefinition) {
        let constant_index = self.compile_function_definition(context, function_definition);

        self.assembly_state.public_indices.insert(function_definition.name.value.to_string(), constant_index);
    }

    fn find_model_index_by_local_name(&mut self, context: &mut CompilerContext, token: &Token) -> Option<usize> {
        if let Some(&model_local_index) = self.locals.get(&token.value.to_string()) {
            if let Some(Object::Model(model_index)) = context.get_local_value(model_local_index) {
                return Some(model_index);
            } else {
                self.errors.push_error(token, "is not a model");
            }
        } else {
            self.errors.push_error(token, "can not found model");
        }

        None
    }

    fn compile_implement_definition(&mut self, context: &mut CompilerContext, implement_definition: &ImplementDefinition) {
        let mut functions: HashMap<String, usize> = HashMap::new();

        for function_definition in implement_definition.functions.iter() {
            let (function, positions) = self.compile_function_definition_base(context, function_definition);
            let index = context.add_function(function, positions, &function_definition.name.value.to_string(), self.assembly_state.index);

            functions.insert(function_definition.name.value.to_string(), index);
        }

        if let Some(model_index) = self.find_model_index_by_local_name(context, &implement_definition.model_name) {
            let model = context.models.get_mut(model_index).unwrap();

            for (name, index) in functions {
                model.functions.insert(name, index);
            };
        }
    }

    fn compile_apply_definition(&mut self, context: &mut CompilerContext, apply_definition: &ApplyDefinition) {
        let mut functions = HashMap::new();

        if let Some(model_index) = self.find_model_index_by_local_name(context, &apply_definition.source_model) {
            let model = context.models.get(model_index).unwrap();

            for (name, &index) in model.functions.iter() {
                functions.insert(name.clone(), index);
            };
        };

        if let Some(model_index) = self.find_model_index_by_local_name(context, &apply_definition.target_model) {
            let model = context.models.get_mut(model_index).unwrap();

            for (name, index) in functions{
                model.functions.insert(name, index);
            };
        };
    }

    fn compile_definition(&mut self, context: &mut CompilerContext, definition: &Definition) {
        match definition {
            Definition::Local(local_definition) => {
                for token in local_definition.variables.iter() {
                    self.define_local_by_identifier(context, token);
                };
            },
            Definition::Include(include_definition) => self.compile_include_definition(context, include_definition),
            Definition::Model(model_definition) => { self.compile_model_definition(context, model_definition); },
            Definition::PublicModel(model_definition) => self.compile_public_model_definition(context, model_definition),
            Definition::Function(function_definition) => { self.compile_function_definition(context, function_definition); },
            Definition::PublicFunction(function_definition) => self.compile_public_function_definition(context, function_definition),
            Definition::Implement(implement_definition) => self.compile_implement_definition(context, implement_definition),
            Definition::Apply(apply_definition) => self.compile_apply_definition(context, apply_definition)
        }
    }

    fn compile(&mut self, context: &mut CompilerContext, document: &Document) {
        for definition in document.definitions.iter() {
            self.compile_definition(context, definition);
        }
    }
}

pub fn compile_document(document: &Document, context: &mut CompilerContext) -> Result<(), CompileErrorList> {

    let mut state = CompilerState {
        assembly_state: AssemblyState::new(&document.filename),
        locals: Scope::new(),
        errors: CompileErrorList::new(&document.filename)
    };

    state.assembly_state.index = context.assemblies.len();

    state.compile(context, document);

    context.add_assembly(state.assembly_state);
    context.file_info.filenames.push(document.filename.clone());

    if state.errors.is_empty() {
        Ok(())
    } else {
        Err(state.errors)
    }
}

pub fn compile_to(context: &mut CompilerContext, source: &str, filename: &str) -> Result<(), CompileErrorList> {
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

        errors.push_error(&Token::new(TokenValue::None, Position::none()), &format!("there may have cycle reference in this files [{}]", cycle_filenames));
        return Err(errors);
    };

    println!("{:?}", context);

    Ok(())
}

fn load_file(filename: &str) -> Result<String, CompileErrorList> {
    if let Ok(source) = read_to_string(filename) {
        Ok(source)
    } else {
        let mut errors = CompileErrorList::new(filename);
        errors.push_error(&Token::new(TokenValue::None, Position::none()), "can not open source file");
        Err(errors)
    }
}

pub fn compile_file(filename: &str) -> Result<Program, CompileErrorList> {
    let source = load_file(filename)?;

    compile(&source, filename)
}

pub fn compile(source: &str, filename: &str) -> Result<Program, CompileErrorList> {
    let mut context = CompilerContext::new();

    compile_to(&mut context, &source, filename)?;

    Ok(context.to_program())
}