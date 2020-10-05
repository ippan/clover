use crate::ast::token::TokenData;

pub mod token;

// expressions

pub type Codes = Vec<Statement>;

#[derive(Debug)]
pub enum Expression {
    Identifier(Box<IdentifierExpressionData>),
    BaseLiteral(Box<BaseLiteralExpressionData>),
    ThisLiteral(Box<ThisLiteralExpressionData>),
    NullLiteral(Box<NullLiteralExpressionData>),
    IntegerLiteral(Box<IntegerLiteralExpressionData>),
    FloatLiteral(Box<FloatLiteralExpressionData>),
    StringLiteral(Box<StringLiteralExpressionData>),
    BooleanLiteral(Box<BooleanLiteralExpressionData>),
    Prefix(Box<PrefixExpressionData>),
    Infix(Box<InfixExpressionData>),
    Function(Box<FunctionExpressionData>),
    If(Box<IfExpressionData>),
    Call(Box<CallExpressionData>),
    InstanceGet(Box<InstanceGetExpressionData>),
    Class(Box<ClassExpressionData>)
}

#[derive(Debug)]
pub struct IdentifierExpressionData {
    pub data: TokenData
}

#[derive(Debug)]
pub struct BaseLiteralExpressionData {
    pub data: TokenData
}

#[derive(Debug)]
pub struct ThisLiteralExpressionData {
    pub data: TokenData
}

#[derive(Debug)]
pub struct NullLiteralExpressionData {
    pub data: TokenData
}

#[derive(Debug)]
pub struct IntegerLiteralExpressionData {
    pub data: TokenData
}

#[derive(Debug)]
pub struct FloatLiteralExpressionData {
    pub data: TokenData
}

#[derive(Debug)]
pub struct StringLiteralExpressionData {
    pub data: TokenData
}

#[derive(Debug)]
pub struct BooleanLiteralExpressionData {
    pub data: TokenData
}

#[derive(Debug)]
pub struct PrefixExpressionData {
    pub prefix: TokenData,
    pub right: Expression
}

#[derive(Debug)]
pub struct InfixExpressionData {
    pub left: Expression,
    pub infix: TokenData,
    pub right: Expression
}

#[derive(Debug)]
pub struct FunctionExpressionData {
    pub parameters: Vec<LocalStatementData>,
    pub body: Codes
}

#[derive(Debug)]
pub struct IfExpressionData {
    pub condition: Expression,
    pub true_part: Codes,
    pub false_part: Option<Codes>
}

#[derive(Debug)]
pub struct CallExpressionData {
    pub function: Expression,
    pub parameters: Vec<Expression>
}

#[derive(Debug)]
pub struct InstanceGetExpressionData {
    pub instance: Expression,
    pub index: Expression
}

#[derive(Debug)]
pub struct ClassExpressionData {
    pub super_class: Option<Expression>,
    pub members: Vec<LocalStatementData>
}

// statements
#[derive(Debug)]
pub enum Statement {
    Local(Box<LocalStatementData>),
    Return(Box<ReturnStatementData>),
    Expression(Box<ExpressionStatementData>)
}

#[derive(Debug)]
pub struct LocalStatementData {
    pub identifier: TokenData,
    pub expression: Option<Expression>
}

#[derive(Debug)]
pub struct ReturnStatementData {
    pub expression: Expression
}

#[derive(Debug)]
pub struct ExpressionStatementData {
    pub expression: Expression
}

// main program
#[derive(Debug)]
pub struct Program {
    pub codes: Codes,
    pub filename: String
}

impl Program {
    pub fn new(filename: String) -> Program {
        Program {
            codes: Vec::new(),
            filename
        }
    }
}