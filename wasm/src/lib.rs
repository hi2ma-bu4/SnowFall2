extern crate wasm_bindgen;
use serde::Serialize;
use wasm_bindgen::prelude::*;

// `console.error` を介してパニック情報を表示するためのフック
extern crate console_error_panic_hook;

pub mod common;
pub mod compiler;

use crate::common::constants;
use crate::common::error::SnowFallError;
use crate::common::{Token, TokenKind};
use crate::compiler::ast::ProgramAst;
use crate::compiler::serialize_ir::serialize_ir;
use crate::compiler::{IrCompiler, normalizer};
use crate::compiler::{Lexer, Parser};

/// ライブラリの初期化時に一度だけ呼び出されるべき関数
#[wasm_bindgen(start)]
pub fn main_init() {
    console_error_panic_hook::set_once();
}

/// Rust の`Vec::into_raw_parts`によって取得したポインタを解放
///
/// この関数は、`Vec::into_raw_parts`で分解された
/// `(ptr, length, capacity)`の完全な対となる解放関数です。
///
/// 上記条件を満たさない場合、未定義動作(UB)になります。
#[wasm_bindgen]
pub fn free_memory_with_len(ptr: *mut u8, length: usize, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, length, capacity);
    }
}

/// Rust 側で確保されたヒープメモリを解放（解放専用）
///
/// この関数はメモリ解放のみを目的としたAPIです。
/// 長さ情報を必要とせず、`capacity`分のメモリを解放します。
///
/// この関数は`length`を0として扱うため、
/// データの内容には一切アクセスしません。
#[wasm_bindgen]
pub fn free_memory(ptr: *mut u8, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, capacity);
    }
}

/// バージョン情報
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

    serde_wasm_bindgen::to_value(&tokens).map_err(|e| e.into())
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

    serde_wasm_bindgen::to_value(&compile_result).map_err(|e| e.into())
}

/// ソースコードを受け取り、正規化したASTを返す
#[wasm_bindgen]
#[allow(deprecated, reason = "Dev関数では許容")]
#[deprecated(since = "1.0.0", note = "本番環境での使用は非推奨")]
pub fn normalize(source: &str) -> Result<JsValue, JsValue> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let result = parser.parse_program();

    let compile_result = match result {
        Ok(program) => {
            let normalized_program = normalizer::normalize(program);
            ParserResult {
                ast: Some(normalized_program),
                errors: None,
            }
        }
        Err(errors) => ParserResult {
            ast: None,
            errors: Some(errors),
        },
    };

    serde_wasm_bindgen::to_value(&compile_result).map_err(|e| e.into())
}

#[wasm_bindgen]
pub struct WasmCompileResult {
    binary: Option<Vec<u8>>,
    errors: JsValue, // Option<Vec<SnowFallError>> をシリアライズしたもの
}

#[wasm_bindgen]
impl WasmCompileResult {
    /// バイナリデータ (Uint8Array | undefined)
    #[wasm_bindgen(getter)]
    pub fn binary(&self) -> Option<Vec<u8>> {
        self.binary.clone()
    }

    /// エラーリスト (ISnowFallError[] | undefined)
    #[wasm_bindgen(getter)]
    pub fn errors(&self) -> JsValue {
        self.errors.clone()
    }
}

/// ソースコードをコンパイルし、バイナリへのポインタとサイズを返す
/// debug: true の場合、ソースマップ（Debug Section）を含めます
#[wasm_bindgen]
pub fn compile(source: &str, debug: bool) -> WasmCompileResult {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            let errors_val = serde_wasm_bindgen::to_value(&e).unwrap_or(JsValue::NULL);
            return WasmCompileResult {
                binary: None,
                errors: errors_val,
            };
        }
    };

    // Normalize
    let normalized = normalizer::normalize(program);

    // Compile IR
    let compiler = IrCompiler::new(source, debug);
    let ir_module = compiler.compile(normalized);

    // Serialize
    let bytes = serialize_ir(ir_module);

    // Success Result
    WasmCompileResult {
        binary: Some(bytes), // Vec<u8> -> Uint8Array への変換はwasm-bindgenが行う
        errors: JsValue::UNDEFINED,
    }
}
