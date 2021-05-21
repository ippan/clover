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
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    If(IfExpression),
    Call(CallExpression),
    InstanceGet(InstanceGetExpression)
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
    pub function: Box<Expression>,
    pub parameters: Vec<Expression>
}

#[derive(Clone, Debug)]
pub struct InstanceGetExpression {
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
    Include(IncludeDefinition)
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
    pub models: Vec<Token>,
    pub aliases: Vec<Token>,
    pub filename: Token
}

#[derive(Clone, Debug)]
pub enum Statement {
    Local(LocalStatement),
    Return(ReturnStatement),
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
pub struct Document {
    pub definitions: Vec<Definition>,
    pub filename: String
}

impl Document {
    pub fn get_dependencies(&self) -> Vec<String> {
        let mut filenames = Vec::new();

        let path = env::current_dir().unwrap();

        let mut definition_iterator = self.definitions.iter();
        while let Some(Definition::Include(definition)) = definition_iterator.next() {
            let mut current_path = PathBuf::from(&self.filename);
            current_path.pop();

            if let TokenValue::String(filename) = &definition.filename.value {
                current_path.push(filename);
                if let Ok(include_path) = current_path.canonicalize() {
                    filenames.push(include_path.strip_prefix(&path).unwrap().to_str().unwrap().to_string());
                }
            };
        };

        filenames
    }
}