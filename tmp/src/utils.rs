use serde::{Deserialize, Serialize};

// --- Constants ---
pub const KEYWORD_LET: &str = "let";
pub const KEYWORD_CONST: &str = "const";
pub const KEYWORD_FN: &str = "fn";
pub const KEYWORD_RETURN: &str = "return";
pub const KEYWORD_TRUE: &str = "true";
pub const KEYWORD_FALSE: &str = "false";
pub const KEYWORD_IF: &str = "if";
pub const KEYWORD_ELSE: &str = "else";

// --- Types ---
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub start_index: usize,
    pub end_index: usize,
}

impl Span {
    pub fn new(line: usize, column: usize, start: usize, end: usize) -> Self {
        Self {
            line,
            column,
            start_index: start,
            end_index: end,
        }
    }
}

// --- Errors ---
#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerError {
    pub message: String,
    pub span: Span,
}

impl CompilerError {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}

pub type CompilerResult<T> = Result<T, CompilerError>;
