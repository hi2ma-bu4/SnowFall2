use crate::utils::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TokenType {
    // Identifiers & Literals
    Identifier(String),
    Int(i64),
    Float(f64),
    StringLiteral(String),

    // Keywords
    Let,
    Const,
    Fn,
    Return,
    If,
    Else,
    True,
    False,

    // Operators
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    Bang,     // !
    Eq,       // ==
    NotEq,    // !=
    Lt,       // <
    Gt,       // >

    // Delimiters
    Comma,     // ,
    Colon,     // :
    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }

    // End of File
    EOF,

    // Illegal
    Illegal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String, // デバッグやエラー表示用に元の文字列も保持
    pub span: Span,
}
