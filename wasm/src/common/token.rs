use serde::{Deserialize, Serialize};
use std::fmt;

use crate::common::Span;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum TokenKind {
    // 特殊トークン (Special Tokens)
    Eof,
    Illegal(String),
    /// 識別子 (Identifiers)
    Identifier(String),
    Literal(LiteralToken),
    Operator(OperatorToken),
    Delimiter(DelimiterToken),
    Keyword(KeywordToken),
}

/// リテラル (Literals)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum LiteralToken {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}
/// 演算子 (Operators)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OperatorToken {
    // 基本的な演算子 (Basic Operators)
    Assign,             // =
    Equal,              // ==
    StrictEqual,        // ===
    Plus,               // +
    Minus,              // -
    Asterisk,           // *
    Power,              // **
    Slash,              // /
    Percent,            // %
    Bang,               // !
    NotEqual,           // !=
    StrictNotEqual,     // !==
    LessThan,           // <
    LessThanOrEqual,    // <=
    GreaterThan,        // >
    GreaterThanOrEqual, // >=

    // 論理演算子 (Logical Operators)
    LogicalAnd, // &&
    LogicalOr,  // ||

    // ビット演算子 (Bitwise Operators)
    BitwiseAnd,                // &
    BitwiseOr,                 // |
    BitwiseXor,                // ^
    BitwiseNot,                // ~
    BitwiseLeftShift,          // <<
    BitwiseUnsignedLeftShift,  // <<<
    BitwiseRightShift,         // >>
    BitwiseUnsignedRightShift, // >>>
}

/// 境界記号 (Delimiters)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DelimiterToken {
    Dot,       // .
    Comma,     // ,
    Colon,     // :
    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
}

/// キーワード (Keywords)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum KeywordToken {
    Function,    // function
    Sub,         // sub
    Class,       // class
    Extends,     // extends
    Constructor, // constructor
    New,         // new
    If,          // if
    Else,        // else
    For,         // for
    While,       // while
    In,          // in
    Of,          // of
    Switch,      // switch
    Case,        // case
    Default,     // default
    Break,       // break
    Continue,    // continue
    Return,      // return
    True,        // true
    False,       // false
    Null,        // null
    And,         // and
    Or,          // or
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
