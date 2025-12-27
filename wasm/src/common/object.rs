use std::collections::HashMap;
use parking_lot::Mutex;

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
#[derive(Debug, Serialize, Deserialize, Default)]
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

    /// プロパティ検索結果をキャッシュするための内部キャッシュ。
    /// `Mutex` を使用して内部可変性を実現します。
    /// シリアライズは不要であり、各インスタンスが独自のキャッシュを持つためスキップします。
    /// An internal cache for property lookup results.
    /// Uses `Mutex` for interior mutability.
    #[serde(skip, default)]
    pub properties_cache: Mutex<HashMap<String, SnowObject>>,
}

impl Clone for SnowObject {
    fn clone(&self) -> Self {
        Self {
            type_id: self.type_id,
            data: self.data.clone(),
            __proto__: self.__proto__.clone(),
            properties: self.properties.clone(),
            version: self.version,
            // Mutexはcloneせず、新しいインスタンスを生成します。
            // The Mutex is not cloned; a new instance is created.
            properties_cache: Mutex::new(HashMap::new()),
        }
    }
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
            properties_cache: Mutex::new(HashMap::new()),
        }
    }

    /// プロパティを検索します。まず自身のプロパティを探し、見つからなければキャッシュを確認し、
    /// それでも見つからなければプロトタイプチェーンを再帰的に検索します。
    ///
    /// ## キャッシュ機構について
    /// このメソッドは、プロトタイプチェーンの検索結果をインスタンスごとにキャッシュします。
    /// プロトタイプチェーンが長い場合（例: 10階層以上）、キャッシュは2回目以降の
    /// プロパティアクセスをO(1)に近づけ、大幅な性能向上をもたらします。
    ///
    /// 一方、プロトタイプの動的な変更（`String.__proto__.foo = ...`）があった場合、
    /// このキャッシュは古くなる可能性があります。そのため、プロトタイプが変更された際には
    /// `version`番号をインクリメントし、関連するすべてのインスタンスのキャッシュを
    /// クリアする機構が別途必要になります。
    ///
    /// Searches for a property. It first looks in its own properties, then checks a cache,
    /// and finally, if not found, recursively searches the prototype chain.
    pub fn get_property(&self, name: &str) -> Option<SnowObject> {
        // 1. 自身のインスタンスプロパティを最優先で検索
        if let Some(prop) = self.properties.get(name) {
            return Some(prop.clone());
        }

        // 2. キャッシュを確認
        if let Some(cached_prop) = self.properties_cache.lock().get(name) {
            return Some(cached_prop.clone());
        }

        // 3. プロトタイプチェーンを検索
        if let Some(proto) = &self.__proto__ {
            if let Some(prop_from_proto) = proto.get_property(name) {
                // 発見したプロパティをキャッシュに保存
                self.properties_cache.lock().insert(name.to_string(), prop_from_proto.clone());
                return Some(prop_from_proto);
            }
        }

        // 4. 見つからなかった場合
        None
    }
}

// `Default` for `SnowValue`
impl Default for SnowValue {
    fn default() -> Self {
        SnowValue::Void
    }
}
