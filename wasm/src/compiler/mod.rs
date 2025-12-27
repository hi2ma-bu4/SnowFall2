/**
 * @fileoverview
 * このモジュールは、SnowFall言語のコンパイル時ロジックを実装します。
 * これには、静的検証、ASTの解析、中間コードの生成などが含まれます。
 *
 * This module implements the compile-time logic for the SnowFall language.
 * This includes static validation, AST parsing, and intermediate code generation.
 */
pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod validator;

pub use codegen::CodeGenerator;
pub use lexer::{Lexer, Token};
pub use parser::Parser;
