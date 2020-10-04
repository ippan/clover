use crate::ast::token::TokenData;

pub mod token;

// expressions

pub type Program = Vec<Statement>;

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

pub struct IdentifierExpressionData {
    pub data: TokenData
}

pub struct BaseLiteralExpressionData {
    pub data: TokenData
}

pub struct ThisLiteralExpressionData {
    pub data: TokenData
}

pub struct NullLiteralExpressionData {
    pub data: TokenData
}

pub struct IntegerLiteralExpressionData {
    pub data: TokenData
}

pub struct FloatLiteralExpressionData {
    pub data: TokenData
}

pub struct StringLiteralExpressionData {
    pub data: TokenData
}

pub struct BooleanLiteralExpressionData {
    pub data: TokenData
}

pub struct PrefixExpressionData {
    pub prefix: TokenData,
    pub right: Expression
}

pub struct InfixExpressionData {
    pub left: Expression,
    pub infix: TokenData,
    pub right: Expression
}

pub struct FunctionExpressionData {
    pub parameters: Vec<LocalStatementData>,
    pub body: Program
}

pub struct IfExpressionData {
    pub condition: Expression,
    pub true_part: Program,
    pub false_part: Program
}

pub struct CallExpressionData {
    pub function: Expression,
    pub parameters: Vec<Expression>
}

pub struct InstanceGetExpressionData {
    pub instance: Expression,
    pub index: Expression
}

pub struct ClassExpressionData {
    pub super_class: Expression,
    pub members: Vec<LocalStatementData>
}

// statements

pub enum Statement {
    Local(Box<LocalStatementData>),
    Return(Box<ReturnStatementData>),
    Expression(Box<ExpressionStatementData>)
}

pub struct LocalStatementData {
    pub identifier: TokenData,
    pub expression: Option<Expression>
}

pub struct ReturnStatementData {
    pub expression: Expression
}

pub struct ExpressionStatementData {
    pub expression: Expression
}