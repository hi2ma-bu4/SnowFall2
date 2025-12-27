// このライブラリのルートモジュールです。
// Wasmに公開する関数などを定義します。

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use std::mem;
// `std::sync::Mutex` の代わりに `parking_lot::Mutex` を使用します。
use parking_lot::Mutex;

// `lazy_static` を使用して、一度だけ初期化される静的なグローバル変数を定義します。
#[macro_use]
extern crate lazy_static;

// `console.error` を介してパニック情報を表示するためのフック
extern crate console_error_panic_hook;

// モジュールを宣言
pub mod common;
pub mod compiler;

use crate::common::error::SnowFallError;
use crate::common::object::{SnowObject, SnowValue, TypeId};
use crate::common::operator::implicit_comparison_equal;
use crate::compiler::ast;
use crate::compiler::validator::{self, TypeChecker};
use crate::compiler::{CodeGenerator, Lexer, Parser, Token};
use serde::Serialize;

// グローバルなライブオブジェクトテーブル。
lazy_static! {
    static ref LIVE_OBJECTS: Mutex<HashMap<u32, SnowObject>> = Mutex::new(HashMap::new());
}

// 新しいオブジェクトハンドルを生成するためのカウンター
static mut HANDLE_COUNTER: u32 = 0;

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

/// TypeScript側から参照されるオブジェクトの要素を取得します。
#[wasm_bindgen]
pub fn get_element_by_handle(handle_id: u32, key: JsValue) -> JsValue {
    let objects = LIVE_OBJECTS.lock();
    if let Some(obj) = objects.get(&handle_id) {
        let element = match &obj.data {
            SnowValue::Array(arr) => {
                if let Some(index) = key.as_f64() {
                    arr.get(index as usize)
                } else {
                    None
                }
            }
            SnowValue::Dictionary(dict) => {
                if let Some(key_str) = key.as_string() {
                    dict.get(&key_str)
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(el) = element {
            match serde_wasm_bindgen::to_value(el) {
                Ok(js_val) => return js_val,
                Err(e) => {
                    return JsValue::from_str(&format!("Serialization error: {}", e));
                }
            }
        }
    }
    JsValue::NULL
}

/// TypeScript側から参照されるオブジェクトの要素を設定します。
#[wasm_bindgen]
pub fn set_element_by_handle(handle_id: u32, key: JsValue, value: JsValue) {
    let mut objects = LIVE_OBJECTS.lock();
    if let Some(obj) = objects.get_mut(&handle_id) {
        let snow_object: SnowObject = match serde_wasm_bindgen::from_value(value) {
            Ok(val) => val,
            Err(_) => return,
        };

        match &mut obj.data {
            SnowValue::Array(arr) => {
                if let Some(index) = key.as_f64() {
                    let idx = index as usize;
                    if idx < arr.len() {
                        arr[idx] = snow_object;
                    } else if idx == arr.len() {
                        arr.push(snow_object);
                    }
                }
            }
            SnowValue::Dictionary(dict) => {
                if let Some(key_str) = key.as_string() {
                    dict.insert(key_str, snow_object);
                }
            }
            _ => (),
        };
    }
}

/// TypeScript側がオブジェクトの参照を終えたことをWasmに通知します。
#[wasm_bindgen]
pub fn release_handle(handle_id: u32) {
    let mut objects = LIVE_OBJECTS.lock();
    objects.remove(&handle_id);
}

/// `SnowFallError` をシリアライズして `JsValue` として返す内部関数。
fn propagate_error(error: SnowFallError) -> JsValue {
    match serde_wasm_bindgen::to_value(&error) {
        Ok(js_error) => js_error,
        Err(_) => JsValue::from_str("Failed to serialize error"),
    }
}

/// オブジェクトのプロトタイプチェーンを再帰的に検索してプロパティを探します。
fn find_property_recursive(obj: &SnowObject, key: &str) -> Option<SnowObject> {
    if let Some(prop) = obj.properties.get(key) {
        return Some(prop.clone());
    }
    if let Some(proto) = &obj.__proto__ {
        return find_property_recursive(proto, key);
    }
    None
}

/// `find_property_recursive` をWasmに公開するためのラッパー関数。
#[wasm_bindgen]
pub fn find_property_on_prototype(obj: JsValue, key: String) -> JsValue {
    let snow_obj: SnowObject = match serde_wasm_bindgen::from_value(obj) {
        Ok(val) => val,
        Err(_) => return JsValue::NULL,
    };

    if let Some(found_prop) = find_property_recursive(&snow_obj, &key) {
        serde_wasm_bindgen::to_value(&found_prop).unwrap_or(JsValue::NULL)
    } else {
        JsValue::NULL
    }
}

/// Compiles SnowFall source code into SIR.
#[wasm_bindgen]
pub fn compile(input: &str) -> JsValue {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program();

    // TODO: Add verification step

    let mut codegen = CodeGenerator::new();
    let sir = codegen.generate(&ast);

    serde_wasm_bindgen::to_value(&sir).unwrap()
}

// --- Test Functions ---

/// コード生成器の出力をテストするための関数。
#[wasm_bindgen]
pub fn _test_codegen(input: &str) -> JsValue {
    compile(input)
}

#[derive(Serialize)]
struct VerifierTestResult {
    errors: Vec<common::error::SnowFallError>,
}

/// 静的型検証器の出力をテストするための関数。
#[wasm_bindgen]
pub fn _test_verifier(input: &str) -> JsValue {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program();

    // For now, we ignore parser errors and proceed
    let mut type_checker = TypeChecker::new();
    let _ = type_checker.check(&ast);

    let result = VerifierTestResult {
        errors: type_checker.errors,
    };

    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[derive(Serialize)]
struct ParserTestResult {
    ast: ast::AstNode,
    errors: Vec<String>,
}

/// 構文解析器の出力をテストするための関数。
#[wasm_bindgen]
pub fn _test_parser(input: &str) -> JsValue {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program();
    let result = ParserTestResult {
        ast,
        errors: parser.errors,
    };
    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// 字句解析器の出力をテストするための関数。
#[wasm_bindgen]
pub fn _test_lexer(input: &str) -> JsValue {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    serde_wasm_bindgen::to_value(&tokens).unwrap()
}

/// エラー伝播をテストするための一時的な関数。
#[wasm_bindgen]
pub fn _test_error_propagation() -> JsValue {
    let mut error = SnowFallError::new(
        "RuntimeError".to_string(),
        "This is a test error from Wasm.".to_string(),
        "SF999".to_string(),
        42,
        8,
    );
    error.trace = vec![
        "function_a at line 20".to_string(),
        "main at line 5".to_string(),
    ];
    propagate_error(error)
}

/// プロトタイプチェーン検索をテストするための一時的な関数。
#[wasm_bindgen]
pub fn _test_prototype_lookup(key: String) -> JsValue {
    let mut grandparent = SnowObject::new(1, SnowValue::Void);
    grandparent.properties.insert(
        "grandparent_prop".to_string(),
        SnowObject::new(2, SnowValue::String("from_grandparent".to_string())),
    );
    let mut parent = SnowObject::new(1, SnowValue::Void);
    parent.__proto__ = Some(Box::new(grandparent));
    parent.properties.insert(
        "parent_prop".to_string(),
        SnowObject::new(2, SnowValue::String("from_parent".to_string())),
    );
    let mut child = SnowObject::new(1, SnowValue::Void);
    child.__proto__ = Some(Box::new(parent));
    child.properties.insert(
        "child_prop".to_string(),
        SnowObject::new(2, SnowValue::String("from_child".to_string())),
    );

    if let Some(found_prop) = find_property_recursive(&child, &key) {
        serde_wasm_bindgen::to_value(&found_prop).unwrap_or(JsValue::NULL)
    } else {
        JsValue::NULL
    }
}

/// テスト用の配列ハンドルを作成して返す。
#[wasm_bindgen]
pub fn _test_create_array_handle() -> JsValue {
    let mut objects = LIVE_OBJECTS.lock();
    let handle_id = unsafe {
        HANDLE_COUNTER += 1;
        HANDLE_COUNTER
    };
    let arr = vec![
        SnowObject::new(10, SnowValue::Int(100)),
        SnowObject::new(20, SnowValue::String("hello".to_string())),
    ];
    let snow_obj = SnowObject::new(30, SnowValue::Array(arr));
    objects.insert(handle_id, snow_obj);

    let handle = js_sys::Object::new();
    js_sys::Reflect::set(&handle, &"__type".into(), &"SnowFallHandle".into()).unwrap();
    js_sys::Reflect::set(&handle, &"id".into(), &handle_id.into()).unwrap();
    js_sys::Reflect::set(&handle, &"dataType".into(), &"Array".into()).unwrap();
    js_sys::Reflect::set(&handle, &"size".into(), &2.into()).unwrap();

    handle.into()
}

/// 暗黙の型変換を伴う比較ロジックをテストするための関数。
#[wasm_bindgen]
pub fn _test_implicit_comparison(left: JsValue, right: JsValue) -> Result<bool, JsValue> {
    let left_val: SnowValue =
        serde_wasm_bindgen::from_value(left).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let right_val: SnowValue =
        serde_wasm_bindgen::from_value(right).map_err(|e| JsValue::from_str(&e.to_string()))?;

    implicit_comparison_equal(&left_val, &right_val)
        .map_err(|e| serde_wasm_bindgen::to_value(&e).unwrap_or(JsValue::NULL))
}

/// テスト用のディクショナリハンドルを作成して返す。
#[wasm_bindgen]
pub fn _test_create_dictionary_handle() -> JsValue {
    let mut objects = LIVE_OBJECTS.lock();
    let handle_id = unsafe {
        HANDLE_COUNTER += 1;
        HANDLE_COUNTER
    };
    let mut dict = HashMap::new();
    dict.insert("a".to_string(), SnowObject::new(10, SnowValue::Int(100)));
    dict.insert(
        "b".to_string(),
        SnowObject::new(20, SnowValue::String("world".to_string())),
    );
    let snow_obj = SnowObject::new(40, SnowValue::Dictionary(dict));
    objects.insert(handle_id, snow_obj);

    let handle = js_sys::Object::new();
    js_sys::Reflect::set(&handle, &"__type".into(), &"SnowFallHandle".into()).unwrap();
    js_sys::Reflect::set(&handle, &"id".into(), &handle_id.into()).unwrap();
    js_sys::Reflect::set(&handle, &"dataType".into(), &"Dictionary".into()).unwrap();
    js_sys::Reflect::set(&handle, &"size".into(), &2.into()).unwrap();

    handle.into()
}
