use crate::vm::memory::GcRef;
use std::fmt;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    /// ヒープオブジェクトへの参照 (Handle)
    Obj(GcRef),
}

impl Value {
    pub fn type_name(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Bool(_) => "Boolean".to_string(),
            Value::Int(_) => "Int".to_string(),
            Value::Float(_) => "Float".to_string(),
            Value::Obj(_) => "Object".to_string(), // 詳細な型はHeap経由で取得
        }
    }

    /// 真偽値判定 (SnowFallの仕様に合わせて拡張可能)
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Obj(_) => true,
        }
    }

    /// JSValueへの変換 (実行結果をJSに返すため)
    pub fn to_js_value(&self) -> JsValue {
        match self {
            Value::Null => JsValue::NULL,
            Value::Bool(b) => JsValue::from_bool(*b),
            // JSのNumberは正確には53bitまでだが、安全のためBigIntにするか、
            // 相互運用性を重視してNumberにするかは要件次第。ここでは安全にNumber(f64)とする
            Value::Int(i) => JsValue::from_f64(*i as f64),
            Value::Float(f) => JsValue::from_f64(*f),
            // オブジェクトの場合は簡易的に文字列表現を返すか、undefinedとする
            // 本格的な連携にはserdeシリアライズが必要
            Value::Obj(_) => JsValue::from_str("<Object>"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(n) => write!(f, "{}", n),
            Value::Obj(r) => write!(f, "<obj:{}>", r.index),
        }
    }
}
