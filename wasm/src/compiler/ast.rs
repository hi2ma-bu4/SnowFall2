use serde::{Serialize, Deserialize};
use crate::compiler::Token;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum AstNode {
    Program(Vec<Statement>),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Statement {
    Let {
        name: String,
        type_name: String,
        value: Expression,
    },
    Return(Expression),
    Expression(Expression),
    // For now, `function` and `sub` are statements
    Function {
        name: String,
        params: Vec<String>, // For simplicity, just param names
        body: Box<Statement>,
        return_type: String, // Type name as string
    },
    Sub {
        name: String,
        params: Vec<String>,
        body: Box<Statement>,
    },
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    Block(Vec<Statement>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expression {
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    Boolean(bool),
    Prefix {
        operator: Token,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Call {
        function: Box<Expression>, // identifier or another call
        arguments: Vec<Expression>,
    },
    Cast {
        target_type: String,
        expression: Box<Expression>,
    },
}

// The Visitor Trait
pub trait Visitor {
    type Output;

    fn visit_node(&mut self, node: &AstNode) -> Self::Output;
    fn visit_statement(&mut self, stmt: &Statement) -> Self::Output;
    fn visit_expression(&mut self, expr: &Expression) -> Self::Output;
}
