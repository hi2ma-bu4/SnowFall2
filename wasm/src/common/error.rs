use ahash::AHashMap;
use serde::{Deserialize, Serialize};

use crate::common::ErrorCode;

/// エラーに関連する追加情報（例: 期待された型、見つかった型など）
pub type SnowFallErrorContext = AHashMap<String, String>;

/// SnowFall言語のすべてのエラーを表現する構造体
/// この構造体はシリアライズされ、Wasm境界を越えてTypeScript側に渡される
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnowFallError {
    /// エラーの種類 (例: "CompilationError", "RuntimeError")
    pub r#type: String,

    /// ユーザー向けの簡潔なエラーメッセージ
    pub message: String,

    /// エラーを一意に識別するためのコード (例: "SF001")
    pub code: String,

    /// エラーが発生したソースコードの行番号 (1ベース)
    pub line: u32,

    /// エラーが発生したソースコードの文字数 (1ベース)
    pub column: u32,

    /// スタックトレース (関数呼び出しの履歴)
    pub trace: Vec<String>,

    /// エラーに関連する追加情報
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<SnowFallErrorContext>,
}

impl SnowFallError {
    /// 新しいSnowFallErrorインスタンスを作成
    pub fn new(r#type: String, message: String, code: String, line: u32, column: u32) -> Self {
        Self {
            r#type,
            message,
            code,
            line,
            column,
            trace: Vec::new(),
            context: None,
        }
    }

    /// `CompilationError` 型の `SnowFallError` を生成するためのファクトリ関数
    pub fn new_compiler_error(
        message: Option<String>,
        code: ErrorCode,
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            r#type: "CompilationError".to_string(),
            message: message.unwrap_or_else(|| code.get_default_message().to_string()),
            code: code.to_str().to_string(),
            line,
            column,
            trace: Vec::new(),
            context: None,
        }
    }

    /// `RuntimeError` 型の `SnowFallError` を生成するためのファクトリ関数
    /// この関数は、動的にキャプチャされたスタックトレースを受け取り、
    /// エラーオブジェクトに含めることができます。これにより、WasmからTSへ
    /// 詳細な実行時エラー情報を渡すことが可能になります
    pub fn new_runtime_error(
        message: String,
        code: String,
        line: u32,
        column: u32,
        trace: Vec<String>,
    ) -> Self {
        Self {
            r#type: "RuntimeError".to_string(),
            message,
            code,
            line,
            column,
            trace,
            context: None,
        }
    }
}
