pub mod ast;
pub mod ir;
pub mod lexer;
pub mod normalizer;
pub mod parser;
pub mod serialize_ir;

pub use ir::IrCompiler;
pub use lexer::Lexer;
pub use parser::Parser;
