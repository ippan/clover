use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

use crate::backend::dependency_solver::DependencySolver;
use crate::backend::function_state::{Scope, FunctionState};
use crate::frontend::parser::parse;
use crate::intermediate::{CompileErrorList, Position, Token, TokenValue};
use crate::intermediate::ast::{Definition, Document, IncludeDefinition, ModelDefinition, FunctionDefinition, ImplementDefinition, ApplyDefinition, Statement, Expression, IntegerExpression, FloatExpression, StringExpression, BooleanExpression, IdentifierExpression, InfixExpression, CallExpression, InstanceGetExpression, ThisExpression};
use crate::runtime::object::Object;
use crate::runtime::opcode::{OpCode, Instruction};
use crate::runtime::program::{Program, Model, Function};
use crate::backend::assembly_state::AssemblyState;
use crate::runtime::assembly_information::{FileInfo, DebugInfo};
use std::ops::Deref;

#[derive(Debug)]
pub struct CompilerContext {
    pub models: Vec<Model>,
    pub functions: Vec<Function>,
    pub constants: Vec<Object>,
    pub global_dependencies: HashSet<usize>,

    pub local_count: usize,
    pub assemblies: HashMap<String, AssemblyState>,
    pub local_values: HashMap<usize, usize>,

    pub entry_point: usize,

    pub file_info: FileInfo,
    pub debug_info: DebugInfo
}

impl CompilerContext {
    pub fn new() -> CompilerContext {
        CompilerContext {
            models: Vec::new(),
            functions: Vec::new(),
            constants: Vec::new(),
            global_dependencies: HashSet::new(),

            local_count: 0,
            assemblies: HashMap::new(),
            local_values: HashMap::new(),

            entry_point: 0,

            file_info: FileInfo::new(),
            debug_info: DebugInfo::new()
        }
    }

    pub fn add_constant(&mut self, object: Object) -> usize {
        for (i, constant) in self.constants.iter().enumerate() {
            if *constant == object {
                return i;
            };
        };

        let index = self.constants.len();
        self.constants.push(object);
        index
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

    pub fn add_function(&mut self, function_state: FunctionState, name: &str, assembly_index: usize) -> usize {
        let index = self.functions.len();

        let function = Function {
            parameter_count: function_state.parameter_count,
            local_count: function_state.local_count,
            is_instance: function_state.is_instance,

            instructions: function_state.instructions
        };

        self.functions.push(function);
        self.debug_info.functions.push(function_state.positions);
        self.file_info.function_names.push(name.to_string());
        self.file_info.function_files.push(assembly_index);
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
            models: self.models.clone(),
            functions: self.functions.clone(),
            constants: self.constants.clone(),
            global_dependencies: self.global_dependencies.iter().cloned().collect(),

            local_count: self.local_count,
            local_values: self.local_values.clone(),

            entry_point: self.entry_point,

            file_info: Some(self.file_info.clone()),
            debug_info: Some(self.debug_info.clone())
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

    fn compile_integer_expression(&mut self, context: &mut CompilerContext, function_state: &mut FunctionState, integer_expression: &IntegerExpression) {
        if let TokenValue::Integer(value) = integer_expression.token.value {
            let index = context.add_constant(Object::Integer(value));
            function_state.emit(OpCode::PushConstant.to_instruction(index as u64), integer_expression.token.position);
        }
    }

    fn compile_float_expression(&mut self, context: &mut CompilerContext, function_state: &mut FunctionState, float_expression: &FloatExpression) {
        if let TokenValue::Float(value) = float_expression.token.value {
            let index = context.add_constant(Object::Float(value));
            function_state.emit(OpCode::PushConstant.to_instruction(index as u64), float_expression.token.position);
        }
    }

    fn compile_string_expression(&mut self, context: &mut CompilerContext, function_state: &mut FunctionState, string_expression: &StringExpression) {
        if let TokenValue::String(value) = &string_expression.token.value {
            let index = context.add_constant(Object::String(value.clone()));
            function_state.emit(OpCode::PushConstant.to_instruction(index as u64), string_expression.token.position);
        }
    }

    fn compile_boolean_expression(&mut self, _context: &mut CompilerContext, function_state: &mut FunctionState, bool_expression: &BooleanExpression) {
        match bool_expression.token.value {
            TokenValue::True => function_state.emit(OpCode::PushBoolean.to_instruction(1), bool_expression.token.position),
            TokenValue::False => function_state.emit(OpCode::PushBoolean.to_instruction(0), bool_expression.token.position),
            _ => self.errors.push_error(&bool_expression.token, "Unexpect token")
        }
    }

    fn compile_identifier_expression(&mut self, context: &mut CompilerContext, function_state: &mut FunctionState, identifier_expression: &IdentifierExpression) {
        let identifier = identifier_expression.token.value.to_string();

        if let Some(index) = function_state.find_local(&identifier) {
            function_state.emit(OpCode::LocalGet.to_instruction(index as u64), identifier_expression.token.position);
        } else if let Some(&index) = self.locals.get(&identifier) {
            function_state.emit(OpCode::ContextGet.to_instruction(index as u64), identifier_expression.token.position);
        } else {
            let index = context.add_constant(Object::String(identifier));
            context.global_dependencies.insert(index);
            function_state.emit(OpCode::GlobalGet.to_instruction(index as u64), identifier_expression.token.position);
        }
    }

    fn compile_this_expression(&mut self, _context: &mut CompilerContext, function_state: &mut FunctionState, this_expression: &ThisExpression) {
        function_state.emit(OpCode::LocalGet.to_instruction(0 as u64), this_expression.token.position);
    }

    fn compile_assign_expression_left_part(&mut self, context: &mut CompilerContext, function_state: &mut FunctionState, infix_expression: &InfixExpression) {
        let left_expression = infix_expression.left.deref();

        match left_expression {
            Expression::Identifier(identifier_expression) => {
                let identifier = identifier_expression.token.value.to_string();

                if let Some(index) = function_state.find_local(&identifier) {
                    function_state.emit(OpCode::LocalSet.to_instruction(index as u64), infix_expression.infix.position);
                } else if let Some(&index) = self.locals.get(&identifier) {
                    function_state.emit(OpCode::ContextSet.to_instruction(index as u64), infix_expression.infix.position);
                } else {
                    let index = context.add_constant(Object::String(identifier));
                    context.global_dependencies.insert(index);
                    function_state.emit(OpCode::GlobalSet.to_instruction(index as u64), infix_expression.infix.position);
                }
            },
            Expression::InstanceGet(instance_get_expression) => {
                self.compile_expression(context, function_state, instance_get_expression.instance.deref());
                self.compile_expression(context, function_state, instance_get_expression.index.deref());

                function_state.emit_opcode(OpCode::InstanceSet, instance_get_expression.token.position);
            },
            _ => self.errors.push_error(&infix_expression.infix, "can not assign")
        }
    }

    fn compile_assign_expression(&mut self, context: &mut CompilerContext, function_state: &mut FunctionState, infix_expression: &InfixExpression) {
        self.compile_expression(context, function_state, infix_expression.right.deref());
        self.compile_assign_expression_left_part(context, function_state, infix_expression);
    }

    fn compile_infix_expression(&mut self, context: &mut CompilerContext, function_state: &mut FunctionState, infix_expression: &InfixExpression) {
        if infix_expression.infix.value == TokenValue::Assign {
            return self.compile_assign_expression(context, function_state, infix_expression);
        };

        if let Some(opcode) = get_operation_opcode_by_token(&infix_expression.infix) {
            self.compile_expression(context, function_state, infix_expression.left.deref());
            self.compile_expression(context, function_state, infix_expression.right.deref());
            function_state.emit_opcode(opcode, infix_expression.infix.position);
        } else {
            self.errors.push_error(&infix_expression.infix, "unknown operation");
        }

        match infix_expression.infix.value {
            TokenValue::PlusAssign | TokenValue::MinusAssign | TokenValue::StarAssign | TokenValue::SlashAssign => { self.compile_assign_expression_left_part(context, function_state, infix_expression); },
            _ => {
                // do nothing
            }
        };
    }

    fn compile_call_expression(&mut self, context: &mut CompilerContext, function_state: &mut FunctionState, call_expression: &CallExpression) {
        // compile the function, after this the function object will on the top of stack
        self.compile_expression(context, function_state, call_expression.function.deref());
        // compile parameters
        for parameter_expression in call_expression.parameters.iter() {
            self.compile_expression(context, function_state, parameter_expression);
        };

        function_state.emit(OpCode::Call.to_instruction(call_expression.parameters.len() as u64), call_expression.token.position);
    }

    fn compile_instance_get_expression(&mut self, context: &mut CompilerContext, function_state: &mut FunctionState, instance_get_expression: &InstanceGetExpression) {
        self.compile_expression(context, function_state, instance_get_expression.instance.deref());
        self.compile_expression(context, function_state, instance_get_expression.index.deref());

        function_state.emit_opcode(OpCode::InstanceGet, instance_get_expression.token.position);
    }

    fn compile_expression(&mut self, context: &mut CompilerContext, function_state: &mut FunctionState, expression: &Expression) {
        match expression {
            Expression::Integer(integer_expression) => self.compile_integer_expression(context, function_state, integer_expression),
            Expression::Float(float_expression) => self.compile_float_expression(context, function_state, float_expression),
            Expression::String(string_expression) => self.compile_string_expression(context, function_state, string_expression),
            Expression::Boolean(bool_expression) => self.compile_boolean_expression(context, function_state, bool_expression),
            Expression::Null(null_expression) => function_state.emit_opcode(OpCode::PushNull, null_expression.token.position),
            Expression::Identifier(identifier_expresision) => self.compile_identifier_expression(context, function_state, identifier_expresision),
            Expression::Infix(infix_expression) => self.compile_infix_expression(context, function_state, infix_expression),
            Expression::Call(call_expression) => self.compile_call_expression(context, function_state, call_expression),
            Expression::InstanceGet(instance_get_expression) => self.compile_instance_get_expression(context, function_state, instance_get_expression),
            Expression::This(this_expression) => self.compile_this_expression(context, function_state, this_expression),
            _ => {}
        }
    }

    fn compile_statement(&mut self, context: &mut CompilerContext, function_state: &mut FunctionState, statement: &Statement) {
        match statement {
            Statement::Return(return_statement) => function_state.emit_return(return_statement.token.position),
            Statement::Expression(expression) => {
                self.compile_expression(context, function_state, expression);
                function_state.emit_opcode_without_position(OpCode::Pop);
            },
            Statement::Local(local_statement) => {
                for (i, token) in local_statement.variables.iter().enumerate() {
                    if let Some(index) = function_state.define_local(&token.value.to_string()) {
                        if let Some(expression) = local_statement.values.get(i).unwrap() {
                            self.compile_expression(context, function_state, expression);
                            function_state.emit(OpCode::LocalInit.to_instruction(index as u64), token.position);
                        }
                    } else {
                        self.errors.push_error(token, "variable already exists");
                    };
                }
            }
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
            if !model.add_property(&token.value.to_string()) {
                self.errors.push_error(token, "property already exists");
            }
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

    fn compile_function_definition_base(&mut self, context: &mut CompilerContext, function_definition: &FunctionDefinition) -> FunctionState {
        let mut function_state = FunctionState::new();

        for parameter in function_definition.parameters.iter() {
            if TokenValue::This == parameter.value {
                function_state.is_instance = true;
            };

            if function_state.define_local(&parameter.value.to_string()).is_none() {
                self.errors.push_error(parameter, "parameter already exists");
            };
        };

        function_state.parameter_count = function_definition.parameters.len();

        for statement in function_definition.body.iter() {
            self.compile_statement(context, &mut function_state, statement);
        };

        function_state.emit_return(function_state.get_last_position());

        function_state
    }

    // return constant index
    fn compile_function_definition(&mut self, context: &mut CompilerContext, function_definition: &FunctionDefinition) -> usize {
        let function_state = self.compile_function_definition_base(context, function_definition);

        // can not have instance function here
        if function_state.is_instance {
            self.errors.push_error(&function_definition.name, "instance function can inside implement block only");
            0
        } else {
            let function_index = context.add_function(function_state, &function_definition.name.value.to_string(), self.assembly_state.index);
            let constant_index = context.add_constant(Object::Function(function_index));

            if let Some(local_index) = self.define_local_by_identifier(context, &function_definition.name) {
                context.local_values.insert(local_index, constant_index);
            };

            if &function_definition.name.value.to_string() == "main" {
                context.entry_point = function_index;
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
            let function_state = self.compile_function_definition_base(context, function_definition);
            let index = context.add_function(function_state, &function_definition.name.value.to_string(), self.assembly_state.index);

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

// helpers
fn get_operation_opcode_by_token(token: &Token) -> Option<OpCode> {
    match token.value {
        TokenValue::Plus | TokenValue::PlusAssign => Some(OpCode::Add),
        TokenValue::Minus | TokenValue::MinusAssign => Some(OpCode::Sub),
        TokenValue::Star | TokenValue::StarAssign => Some(OpCode::Multiply),
        TokenValue::Slash | TokenValue::SlashAssign => Some(OpCode::Divide),
        _ => None
    }
}