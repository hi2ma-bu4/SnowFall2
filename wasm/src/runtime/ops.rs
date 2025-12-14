use crate::types::{SFValue, SFValueType};

/// 足し算 (Int+Int, String+String, オーバーライド想定)
pub fn op_add(a: SFValue, b: SFValue) -> Result<SFValue, String> {
    match (a.dtype, b.dtype) {
        (SFValueType::Int, SFValueType::Int) => Ok(SFValue::new_int(a.raw_int + b.raw_int)),
        (SFValueType::Float, SFValueType::Float) => {
            Ok(SFValue::new_float(a.raw_float + b.raw_float))
        }
        (SFValueType::String, _) | (_, SFValueType::String) => Ok(SFValue::new_string(format!(
            "{}{}",
            a.raw_string, b.raw_string
        ))),
        _ => Err("Unsupported types for ADD".to_string()),
    }
}

/// 引き算
pub fn op_sub(a: SFValue, b: SFValue) -> Result<SFValue, String> {
    match (a.dtype, b.dtype) {
        (SFValueType::Int, SFValueType::Int) => Ok(SFValue::new_int(a.raw_int - b.raw_int)),
        (SFValueType::Float, SFValueType::Float) => {
            Ok(SFValue::new_float(a.raw_float - b.raw_float))
        }
        _ => Err("Unsupported types for SUB".to_string()),
    }
}

/// 掛け算 (String * Int 対応)
pub fn op_mul(a: SFValue, b: SFValue) -> Result<SFValue, String> {
    match (a.dtype.clone(), b.dtype.clone()) {
        (SFValueType::Int, SFValueType::Int) => Ok(SFValue::new_int(a.raw_int * b.raw_int)),
        (SFValueType::Float, SFValueType::Float) => {
            Ok(SFValue::new_float(a.raw_float * b.raw_float))
        }
        // "abc" * 3
        (SFValueType::String, SFValueType::Int) => {
            let count = if b.raw_int < 0 { 0 } else { b.raw_int as usize };
            Ok(SFValue::new_string(a.raw_string.repeat(count)))
        }
        // 3 * "abc"
        (SFValueType::Int, SFValueType::String) => {
            let count = if a.raw_int < 0 { 0 } else { a.raw_int as usize };
            Ok(SFValue::new_string(b.raw_string.repeat(count)))
        }
        _ => Err("Unsupported types for MUL".to_string()),
    }
}

/// 厳密な等価 (===)
pub fn op_eq_strict(a: SFValue, b: SFValue) -> SFValue {
    if a.dtype != b.dtype {
        return SFValue::new_bool(false);
    }
    match a.dtype {
        SFValueType::Int => SFValue::new_bool(a.raw_int == b.raw_int),
        SFValueType::Float => SFValue::new_bool(a.raw_float == b.raw_float),
        SFValueType::String => SFValue::new_bool(a.raw_string == b.raw_string),
        SFValueType::Boolean => SFValue::new_bool(a.raw_bool == b.raw_bool),
        SFValueType::Null => SFValue::new_bool(true),
        _ => SFValue::new_bool(false),
    }
}

/// 緩い等価 (==) : 暗黙の型変換を含む
pub fn op_eq_loose(a: SFValue, b: SFValue) -> SFValue {
    if a.dtype == b.dtype {
        return op_eq_strict(a, b);
    }
    // 簡易実装: 文字列化して比較
    SFValue::new_bool(a.raw_string == b.raw_string)
}
