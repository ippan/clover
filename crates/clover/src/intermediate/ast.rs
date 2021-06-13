use crate::intermediate::{Token, TokenValue};
use std::env;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum Expression {
    Identifier(IdentifierExpression),
    This(ThisExpression),
    Null(NullExpression),
    Integer(IntegerExpression),
    Float(FloatExpression),
    String(StringExpression),
    Boolean(BooleanExpression),
    Array(ArrayExpression),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    If(IfExpression),
    Call(CallExpression),
    InstanceGet(InstanceGetExpression),
    IndexGet(IndexGetExpression)
}

#[derive(Clone, Debug)]
pub struct IdentifierExpression {
    pub token: Token
}

#[derive(Clone, Debug)]
pub struct ThisExpression {
    pub token: Token
}

#[derive(Clone, Debug)]
pub struct NullExpression {
    pub token: Token
}

#[derive(Clone, Debug)]
pub struct IntegerExpression {
    pub token: Token
}

#[derive(Clone, Debug)]
pub struct FloatExpression {
    pub token: Token
}

#[derive(Clone, Debug)]
pub struct StringExpression {
    pub token: Token
}

#[derive(Clone, Debug)]
pub struct BooleanExpression {
    pub token: Token
}

#[derive(Clone, Debug)]
pub struct ArrayExpression {
    pub token: Token,
    pub values: Vec<Expression>
}

#[derive(Clone, Debug)]
pub struct PrefixExpression {
    pub prefix: Token,
    pub right: Box<Expression>
}

#[derive(Clone, Debug)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub infix: Token,
    pub right: Box<Expression>
}

#[derive(Clone, Debug)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub true_part: Vec<Statement>,
    pub false_part: Option<Vec<Statement>>
}

#[derive(Clone, Debug)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<Expression>,
    pub parameters: Vec<Expression>
}

#[derive(Clone, Debug)]
pub struct InstanceGetExpression {
    pub token: Token,
    pub instance: Box<Expression>,
    pub index: Box<Expression>
}

#[derive(Clone, Debug)]
pub struct IndexGetExpression {
    pub token: Token,
    pub instance: Box<Expression>,
    pub index: Box<Expression>
}

#[derive(Clone, Debug)]
pub enum Definition {
    Model(ModelDefinition),
    Implement(ImplementDefinition),
    Apply(ApplyDefinition),
    Function(FunctionDefinition),
    Local(LocalDefinition),
    Include(IncludeDefinition),
    PublicModel(ModelDefinition),
    PublicFunction(FunctionDefinition)
}

#[derive(Clone, Debug)]
pub struct ModelDefinition {
    pub name: Token,
    // TODO : add annotation
    pub properties: Vec<Token>
}

#[derive(Clone, Debug)]
pub struct FunctionDefinition {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Vec<Statement>
}

#[derive(Clone, Debug)]
pub struct ImplementDefinition {
    pub model_name: Token,
    pub functions: Vec<FunctionDefinition>
}

#[derive(Clone, Debug)]
pub struct ApplyDefinition {
    pub source_model: Token,
    pub target_model: Token
}

#[derive(Clone, Debug)]
pub struct LocalDefinition {
    pub variables: Vec<Token>
}

#[derive(Clone, Debug)]
pub struct IncludeDefinition {
    pub public_names: Vec<Token>,
    pub aliases: Vec<Token>,
    pub filename: Token
}

#[derive(Clone, Debug)]
pub enum Statement {
    Local(LocalStatement),
    Return(ReturnStatement),
    For(ForStatement),
    Break(BreakStatement),
    Rescue(RescueStatement),
    Expression(Expression)
}

#[derive(Clone, Debug)]
pub struct LocalStatement {
    pub variables: Vec<Token>,
    pub values: Vec<Option<Expression>>
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
    pub token: Token
}

#[derive(Clone, Debug)]
pub struct BreakStatement {
    pub token: Token
}

#[derive(Clone, Debug)]
pub struct RescueStatement {
    pub token: Token
}

#[derive(Clone, Debug)]
pub struct ForStatement {
    pub token: Token,
    pub identifier: Token,
    pub enumerable: Expression,
    pub statements: Vec<Statement>
}

#[derive(Clone, Debug)]
pub struct Document {
    pub definitions: Vec<Definition>,
    pub filename: String
}

impl Document {
    pub fn normalize_include_paths(&mut self) {
        let path = env::current_dir().unwrap().canonicalize().unwrap();

        let mut definition_iterator = self.definitions.iter_mut();
        while let Some(Definition::Include(definition)) = definition_iterator.next() {
            let mut current_path = PathBuf::from(&self.filename);
            current_path.pop();

            if let TokenValue::String(filename) = &definition.filename.value {
                current_path.push(filename);
                if let Ok(include_path) = current_path.canonicalize() {
                    let stripped_filename = include_path.strip_prefix(&path).unwrap().to_str().unwrap().to_string();
                    definition.filename.value = TokenValue::String(stripped_filename.replace("\\", "/"));
                }
            };
        };
    }

    pub fn get_dependencies(&self) -> Vec<String> {
        let mut filenames = Vec::new();

        let mut definition_iterator = self.definitions.iter();
        while let Some(Definition::Include(definition)) = definition_iterator.next() {
            if let TokenValue::String(filename) = &definition.filename.value {
                    filenames.push(filename.clone());
            };
        };

        filenames
    }
}