use crate::token::TokenType;
use crate::utils::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Statement {
    Let {
        name: String,
        type_annotation: Option<String>,
        value: Expression,
        span: Span,
    },
    Return {
        return_value: Expression,
        span: Span,
    },
    ExpressionStmt {
        expression: Expression,
        span: Span,
    },
    Function {
        name: String,
        params: Vec<Parameter>,
        return_type: Option<String>,
        body: BlockStatement,
        span: Span,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: String,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expression {
    Identifier(String, Span),
    Integer(i64, Span),
    Float(f64, Span),
    Boolean(bool, Span),
    StringLit(String, Span),
    Prefix {
        operator: TokenType,
        right: Box<Expression>,
        span: Span,
    },
    Infix {
        left: Box<Expression>,
        operator: TokenType,
        right: Box<Expression>,
        span: Span,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
        span: Span,
    },
    If {
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
        span: Span,
    },
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Identifier(_, s) => *s,
            Expression::Integer(_, s) => *s,
            Expression::Float(_, s) => *s,
            Expression::Boolean(_, s) => *s,
            Expression::StringLit(_, s) => *s,
            Expression::Prefix { span, .. } => *span,
            Expression::Infix { span, .. } => *span,
            Expression::Call { span, .. } => *span,
            Expression::If { span, .. } => *span,
        }
    }
}
