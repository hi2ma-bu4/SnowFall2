use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;

// 型を一意に識別するためのID
pub type TypeId = u32;

// SnowObjectへの共有参照
// Arc<RwLock<T>> を使用することで、複数スレッドからの安全な共有と変更が可能になります。
// SnowFallのランタイムが将来的にマルチスレッドに対応する可能性も見据えた設計です。
pub type SnowObjectRef = Arc<RwLock<SnowObject>>;

/// SnowFall言語におけるすべてのオブジェクトの実体。
/// プリミティブ値からクラスインスタンスまで、すべてこの構造体で表現されます。
#[derive(Debug)]
pub struct SnowObject {
    /// オブジェクトの具体的な型(Int, String, Arrayなど)を一意に識別するID。
    pub type_id: TypeId,

    /// オブジェクトが持つ実際の値。
    pub data: SnowValue,

    /// 継承元のプロトタイプオブジェクトへの参照。
    /// これを辿ることでプロトタイプチェーンが実現されます。
    // #[serde(skip)] // __proto__は実行時に解決するため、シリアライズ対象外
    pub __proto__: Option<SnowObjectRef>,

    /// インスタンス固有のプロパティ(フィールド、メソッド)。
    /// Key: プロパティ名, Value: プロパティの実体であるSnowObjectへの参照
    pub properties: HashMap<String, SnowObjectRef>,
}

/// SnowObjectが保持する具体的な値の型。
/// Enumを使用することで、メモリ効率と型安全性を両立します。
#[derive(Debug)]
pub enum SnowValue {
    Int(i32),
    Long(i64),
    Float(f64),
    Char(char),
    String(String),
    Boolean(bool),
    Array(Vec<SnowObjectRef>),
    // Dictionary, Function, Class などの型もここに追加していく
    // ...
    /// 何も値がないことを示す型 (例: subの戻り値)
    Void,
}

/// SnowFallにおけるエラー情報を表現する構造体。
/// `thiserror` を利用して、定型的なエラー処理を簡潔に記述します。
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum SnowFallError {
    #[error("Compilation Error: {message} at line {line}, column {column}")]
    CompilationError {
        message: String,
        code: String,
        line: u32,
        column: u32,
        trace: Vec<String>,
    },

    #[error("Runtime Error: {message} at line {line}, column {column}")]
    RuntimeError {
        message: String,
        code: String,
        line: u32,
        column: u32,
        trace: Vec<String>,
    },

    #[error("Syntax Error: {message} at line {line}, column {column}")]
    SyntaxError {
        message: String,
        code: String,
        line: u32,
        column: u32,
        trace: Vec<String>,
    },
}
