extern crate wasm_bindgen;
use serde::{Deserialize, Serialize};
use std::mem;
use wasm_bindgen::prelude::*;

// `console.error` を介してパニック情報を表示するためのフック
extern crate console_error_panic_hook;

pub mod common;
pub mod compiler;

use crate::common::error::SnowFallError;
use crate::common::{Token, TokenKind, constants};
use crate::compiler::ast::ProgramAst;
use crate::compiler::{Lexer, Parser};

/// ライブラリの初期化時に一度だけ呼び出されるべき関数。
#[wasm_bindgen(start)]
pub fn main_init() {
    console_error_panic_hook::set_once();
}

/// Wasmモジュールのメモリを確保し、そのポインタを返す。
#[wasm_bindgen]
pub fn allocate_memory(size: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(size);
    let ptr = buffer.as_mut_ptr();
    mem::forget(buffer);
    ptr
}

/// Wasmモジュール内の確保されたメモリを解放する。
#[wasm_bindgen]
pub fn free_memory(ptr: *mut u8, size: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size);
    }
}

#[wasm_bindgen]
pub fn version() -> String {
    constants::VERSION.to_string()
}

/// ソースコードを受け取り、トークンのリストを返す
#[wasm_bindgen]
#[allow(deprecated, reason = "Dev関数では許容")]
#[deprecated(since = "1.0.0", note = "本番環境での使用は非推奨")]
pub fn lexer(source: &str) -> Result<JsValue, JsValue> {
    let mut lexer = Lexer::new(source);
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        match lexer.next_token() {
            Ok(token) => {
                if token.kind == TokenKind::Eof {
                    break;
                }

                tokens.push(token);
            }
            Err(e) => {
                return Err(JsValue::from_str(&format!("Lexer error: {:?}", e)));
            }
        }
    }

    serde_wasm_bindgen::to_value(&tokens)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[derive(Serialize)]
pub struct ParserResult {
    ast: Option<ProgramAst>,
    errors: Option<Vec<SnowFallError>>,
}

/// ソースコードを受け取り、解析したASTを返す
#[wasm_bindgen]
#[allow(deprecated, reason = "Dev関数では許容")]
#[deprecated(since = "1.0.0", note = "本番環境での使用は非推奨")]
pub fn parser(source: &str) -> Result<JsValue, JsValue> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let result = parser.parse_program();

    let compile_result = match result {
        Ok(program) => ParserResult {
            ast: Some(program),
            errors: None,
        },
        Err(errors) => ParserResult {
            ast: None,
            errors: Some(errors),
        },
    };

    serde_wasm_bindgen::to_value(&compile_result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[derive(Serialize, Deserialize)]
pub struct CompileOptions {
    pub debug_info: bool,
}
