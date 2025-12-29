use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Token {
    // 特殊トークン (Special Tokens)
    Eof,
    Illegal(String),

    // 識別子とリテラル (Identifiers & Literals)
    Identifiers(String),
    Int(i64),
    Float(f64),
    String(String),

    // 演算子 (Operators)
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

    // 境界記号 (Delimiters)
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

    // キーワード (Keywords)
    Function, // function
    Sub,      // sub
    Class,    // class
    Extends,  // extends
    If,       // if
    Else,     // else
    For,      // for
    While,    // while
    In,       // in
    Of,       // of
    Switch,   // switch
    Case,     // case
    Default,  // default
    Break,    // break
    Continue, // continue
    Return,   // return
    True,     // true
    False,    // false
    Null,     // null
    And,      // and
    Or,       // or

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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
