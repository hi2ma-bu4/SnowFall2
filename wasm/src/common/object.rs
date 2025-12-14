// wasm/src/common/object.rs

use std::collections::HashMap;

// serdeのSerializeとDeserializeをインポートします。
// これらは、Rustの構造体をJSONなどの形式に変換したり、
// その逆を行ったりするために必要です。
use serde::{Deserialize, Serialize};

/// SnowFallのすべての型（Int, String, Arrayなど）を一意に識別するためのID。
/// A unique ID to identify every SnowFall type (Int, String, Array, etc.).
pub type TypeId = u32;

/// SnowFallオブジェクトが持つことができる実際の値を表現する列挙型。
/// Enum representing the actual values a SnowObject can hold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnowValue {
    Int(i32),
    Long(i64),
    Float(f64),
    Char(char),
    String(String),
    Boolean(bool),
    Array(Vec<SnowObject>),
    Dictionary(HashMap<String, SnowObject>),
    /// 値が存在しないことを表します。voidやunitに似ています。
    /// Represents the absence of a value, similar to `unit` or `void`.
    Void,
}

/// SnowFall言語におけるすべてのオブジェクト、型、関数を表現する共通構造体。
/// The common structure representing all objects, types, and functions in the SnowFall language.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnowObject {
    /// オブジェクトの具体的な型を識別するID。
    /// The ID that identifies the concrete type of the object.
    pub type_id: TypeId,

    /// オブジェクトが持つ実際の値。
    /// The actual value the object holds.
    #[serde(default)]
    pub data: SnowValue,

    /// 継承元のプロトタイプオブジェクトへの参照。
    /// シリアライズ時の再帰的な問題を避けるため、`__proto__`はスキップします。
    /// A reference to the parent prototype object from which it inherits.
    /// We skip `__proto__` during serialization to avoid recursion issues.
    #[serde(skip, default)]
    pub __proto__: Option<Box<SnowObject>>,

    /// インスタンス固有のプロパティ（フィールドやメソッド）。
    /// Instance-specific properties (fields and methods).
    #[serde(default)]
    pub properties: HashMap<String, SnowObject>,

    /// プロトタイプが変更された回数を追跡するバージョン番号。
    /// ランタイムのメソッドキャッシュの検証に使用されます。
    /// A version number to track how many times the prototype has been modified.
    /// Used for validating the runtime method cache.
    #[serde(default)]
    pub version: u64,
}

impl SnowObject {
    /// 新しいSnowObjectインスタンスを作成します。
    /// Creates a new SnowObject instance.
    pub fn new(type_id: TypeId, data: SnowValue) -> Self {
        Self {
            type_id,
            data,
            __proto__: None,
            properties: HashMap::new(),
            version: 0,
        }
    }
}

// `Default` for `SnowValue`
impl Default for SnowValue {
    fn default() -> Self {
        SnowValue::Void
    }
}
