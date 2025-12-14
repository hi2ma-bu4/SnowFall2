use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// SnowFallの内部データ型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SFValueType {
    Int,
    Float,
    String,
    Boolean,
    Null,
    // 将来的には Function, Object, Array などがここに入る
    Object,
}

/// プロトタイプ（メソッド辞書）
pub type Prototype = HashMap<String, String>; // メソッド名 -> 内部識別子(簡易化)

/// 値の実体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFValue {
    pub dtype: SFValueType,
    pub raw_int: i64,
    pub raw_float: f64,
    pub raw_string: String,
    pub raw_bool: bool,
    // プロトタイプチェーン用（今回は簡易実装として名前で管理）
    pub type_name: String,
}

impl SFValue {
    pub fn new_int(v: i64) -> Self {
        SFValue {
            dtype: SFValueType::Int,
            raw_int: v,
            raw_float: 0.0,
            raw_string: v.to_string(),
            raw_bool: v != 0,
            type_name: "Int".to_string(),
        }
    }

    pub fn new_float(v: f64) -> Self {
        SFValue {
            dtype: SFValueType::Float,
            raw_int: v as i64,
            raw_float: v,
            raw_string: v.to_string(),
            raw_bool: v != 0.0,
            type_name: "Float".to_string(),
        }
    }

    pub fn new_string(v: String) -> Self {
        SFValue {
            dtype: SFValueType::String,
            raw_int: v.len() as i64,
            raw_float: v.len() as f64,
            raw_string: v,
            raw_bool: true,
            type_name: "String".to_string(),
        }
    }

    pub fn new_bool(v: bool) -> Self {
        SFValue {
            dtype: SFValueType::Boolean,
            raw_int: if v { 1 } else { 0 },
            raw_float: if v { 1.0 } else { 0.0 },
            raw_string: v.to_string(),
            raw_bool: v,
            type_name: "Boolean".to_string(),
        }
    }

    pub fn null() -> Self {
        SFValue {
            dtype: SFValueType::Null,
            raw_int: 0,
            raw_float: 0.0,
            raw_string: "null".to_string(),
            raw_bool: false,
            type_name: "Null".to_string(),
        }
    }
}

/// コンパイル結果
#[derive(Serialize)]
pub struct CompileResult {
    pub success: bool,
    pub bytecode: String,
    pub error_msg: Option<String>,
    pub debug_info: Option<String>,
}

/// コンパイラ設定
#[derive(Deserialize)]
pub struct CompilerConfig {
    pub debug_mode: bool,
}

/// ランタイム設定
#[derive(Deserialize)]
pub struct RuntimeConfig {
    pub max_recursion: usize,
    pub debug_mode: bool,
}
