use crate::common::error::SnowFallError;
use crate::common::object::SnowValue;

impl SnowValue {
    /// 内部の値を指定されたRustの型に変換しようと試みます。
    /// Attempts to convert the internal value to a specified Rust type.
    fn to_type(&self, target_type: std::any::TypeId) -> Option<SnowValue> {
        if target_type == std::any::TypeId::of::<f64>() {
            self.to_float()
        } else if target_type == std::any::TypeId::of::<i64>() {
            self.to_long()
        } else if target_type == std::any::TypeId::of::<char>() {
            self.to_char()
        } else if target_type == std::any::TypeId::of::<bool>() {
            self.to_boolean()
        } else if target_type == std::any::TypeId::of::<String>() {
            self.to_string_value()
        } else {
            None
        }
    }

    // 各型への具体的な変換ロジック
    fn to_float(&self) -> Option<SnowValue> {
        match self {
            SnowValue::Float(f) => Some(SnowValue::Float(*f)),
            SnowValue::Int(i) => Some(SnowValue::Float(*i as f64)),
            SnowValue::Long(l) => Some(SnowValue::Float(*l as f64)),
            SnowValue::Char(c) => Some(SnowValue::Float(*c as u32 as f64)),
            SnowValue::Boolean(b) => Some(SnowValue::Float(if *b { 1.0 } else { 0.0 })),
            SnowValue::String(s) => s.parse::<f64>().ok().map(SnowValue::Float),
            _ => None,
        }
    }

    fn to_long(&self) -> Option<SnowValue> {
        match self {
            SnowValue::Long(l) => Some(SnowValue::Long(*l)),
            SnowValue::Int(i) => Some(SnowValue::Long(*i as i64)),
            SnowValue::Char(c) => Some(SnowValue::Long(*c as i64)),
            SnowValue::Boolean(b) => Some(SnowValue::Long(if *b { 1 } else { 0 })),
            _ => None,
        }
    }

    fn to_char(&self) -> Option<SnowValue> {
        match self {
            SnowValue::Char(c) => Some(SnowValue::Char(*c)),
            SnowValue::Int(i) if *i >= 0 && *i <= char::MAX as i32 => {
                std::char::from_u32(*i as u32).map(SnowValue::Char)
            }
            _ => None,
        }
    }

    fn to_boolean(&self) -> Option<SnowValue> {
        match self {
            SnowValue::Boolean(b) => Some(SnowValue::Boolean(*b)),
            SnowValue::Int(i) => Some(SnowValue::Boolean(*i != 0)),
            SnowValue::Long(l) => Some(SnowValue::Boolean(*l != 0)),
            SnowValue::Float(f) => Some(SnowValue::Boolean(*f != 0.0)),
            SnowValue::Char(c) => Some(SnowValue::Boolean(*c != '\0')),
            SnowValue::String(s) => Some(SnowValue::Boolean(!s.is_empty())),
            _ => Some(SnowValue::Boolean(true)), // 配列や辞書などはtrueと評価
        }
    }

    fn to_string_value(&self) -> Option<SnowValue> {
        Some(SnowValue::String(self.to_string_repr()))
    }

    /// `to_string` との衝突を避けるための内部的な文字列表現関数。
    fn to_string_repr(&self) -> String {
        match self {
            SnowValue::String(s) => s.clone(),
            SnowValue::Int(i) => i.to_string(),
            SnowValue::Long(l) => l.to_string(),
            SnowValue::Float(f) => f.to_string(),
            SnowValue::Char(c) => c.to_string(),
            SnowValue::Boolean(b) => b.to_string(),
            SnowValue::Array(_) => "[Array]".to_string(),
            SnowValue::Dictionary(_) => "[Dictionary]".to_string(),
            SnowValue::Void => "void".to_string(),
        }
    }
}

/// `==` 演算子のための暗黙の型変換を伴う比較ロジック。
///
/// ## 優先順位
/// `String` > `Float` > `Long` > `Int` > `Char` > `Boolean` の順で型昇格が行われます。
///
/// ## 数値の精度について
/// - `Int`から`Float`への変換は、`f64`が`i32`の全範囲を正確に表現できるため、
///   精度喪失のリスクはありません。
/// - `Long`(`i64`)から`Float`(`f64`)への変換では、`f64`の仮数部が53ビットであるため、
///   `2^53`を超える大きな整数では精度が失われる可能性があります。
///   しかし、SnowFall言語の仕様として、これは許容される動作と定義します。
///   これは、JavaScriptの`number`型が同様の動作をするため、一貫性を保つためです。
///
/// ## エラーハンドリング
/// 変換不可能な型同士の比較（例: `[Array] == 1`）は、型変換に失敗し、
/// 結果として`false`を返します。`RuntimeError`はスローされません。
///
/// Implements comparison with implicit type conversion for the `==` operator.
pub fn implicit_comparison_equal(
    left: &SnowValue,
    right: &SnowValue,
) -> Result<bool, SnowFallError> {
    // 1. どちらかがStringなら、両方をStringに変換して比較
    if matches!(left, SnowValue::String(_)) || matches!(right, SnowValue::String(_)) {
        return Ok(left.to_string_repr() == right.to_string_repr());
    }
    // 2. どちらかがFloatなら、両方をFloatに変換して比較
    if matches!(left, SnowValue::Float(_)) || matches!(right, SnowValue::Float(_)) {
        let l_float = left.to_float().and_then(|v| match v {
            SnowValue::Float(f) => Some(f),
            _ => None,
        });
        let r_float = right.to_float().and_then(|v| match v {
            SnowValue::Float(f) => Some(f),
            _ => None,
        });
        if let (Some(l), Some(r)) = (l_float, r_float) {
            return Ok(l == r);
        }
    }
    // 3. どちらかがLongなら、両方をLongに変換して比較
    if matches!(left, SnowValue::Long(_)) || matches!(right, SnowValue::Long(_)) {
        let l_long = left.to_long().and_then(|v| match v {
            SnowValue::Long(l) => Some(l),
            _ => None,
        });
        let r_long = right.to_long().and_then(|v| match v {
            SnowValue::Long(l) => Some(l),
            _ => None,
        });
        if let (Some(l), Some(r)) = (l_long, r_long) {
            return Ok(l == r);
        }
    }
    // 4. どちらかがIntなら、両方をIntとして比較（ただしLongへの昇格は考慮済み）
    if matches!(left, SnowValue::Int(_)) || matches!(right, SnowValue::Int(_)) {
        let l_long = left.to_long().and_then(|v| match v {
            SnowValue::Long(l) => Some(l),
            _ => None,
        });
        let r_long = right.to_long().and_then(|v| match v {
            SnowValue::Long(l) => Some(l),
            _ => None,
        });
        if let (Some(l), Some(r)) = (l_long, r_long) {
            return Ok(l == r);
        }
    }
    // 5. どちらかがCharなら、両方をCharに変換して比較
    if matches!(left, SnowValue::Char(_)) || matches!(right, SnowValue::Char(_)) {
        let l_char = left.to_char().and_then(|v| match v {
            SnowValue::Char(c) => Some(c),
            _ => None,
        });
        let r_char = right.to_char().and_then(|v| match v {
            SnowValue::Char(c) => Some(c),
            _ => None,
        });
        if let (Some(l), Some(r)) = (l_char, r_char) {
            return Ok(l == r);
        }
    }
    // 6. どちらかがBooleanなら、両方をBooleanに変換して比較
    if matches!(left, SnowValue::Boolean(_)) || matches!(right, SnowValue::Boolean(_)) {
        let l_bool = left.to_boolean().and_then(|v| match v {
            SnowValue::Boolean(b) => Some(b),
            _ => None,
        });
        let r_bool = right.to_boolean().and_then(|v| match v {
            SnowValue::Boolean(b) => Some(b),
            _ => None,
        });
        if let (Some(l), Some(r)) = (l_bool, r_bool) {
            return Ok(l == r);
        }
    }

    // 7. 上記のいずれにも当てはまらない場合（Array, Dictionary, Voidなど）、変換を試みずにfalseを返す
    Ok(false)
}
