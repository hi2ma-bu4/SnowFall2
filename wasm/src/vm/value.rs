use crate::vm::memory::GcRef;
use std::fmt;

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
