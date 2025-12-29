mod compiler;
mod token;
mod utils;

use crate::compiler::{Lexer, Parser};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile_to_ast(source: &str) -> Result<String, JsValue> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    match parser.parse_program() {
        Ok(program) => {
            // ASTをJSON文字列にシリアライズ
            match serde_json::to_string(&program) {
                Ok(json) => Ok(json),
                Err(e) => Err(JsValue::from_str(&format!("Serialization error: {}", e))),
            }
        }
        Err(errors) => {
            // エラーリストもJSONとして返す
            match serde_json::to_string(&errors) {
                Ok(json_errors) => Err(JsValue::from_str(&json_errors)),
                Err(e) => Err(JsValue::from_str(&format!(
                    "Error serialization error: {}",
                    e
                ))),
            }
        }
    }
}
