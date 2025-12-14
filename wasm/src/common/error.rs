// wasm/src/common/error.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// エラーに関連する追加情報（例: 期待された型、見つかった型など）。
/// Additional context related to an error (e.g., expected type, found type).
pub type SnowFallErrorContext = HashMap<String, String>;

/// SnowFall言語のすべてのエラーを表現する構造体。
/// この構造体はシリアライズされ、Wasm境界を越えてTypeScript側に渡されます。
/// The struct representing all errors in the SnowFall language.
/// This struct is serialized and passed across the Wasm boundary to the TypeScript side.
#[derive(Debug, Serialize, Deserialize)]
pub struct SnowFallError {
    /// エラーの種類 (例: "CompilationError", "RuntimeError")。
    /// The type of the error (e.g., "CompilationError", "RuntimeError").
    pub r#type: String,

    /// ユーザー向けの簡潔なエラーメッセージ。
    /// A concise, user-friendly error message.
    pub message: String,

    /// エラーを一意に識別するためのコード (例: "SF001")。
    /// A unique code to identify the error (e.g., "SF001").
    pub code: String,

    /// エラーが発生したソースコードの行番号 (1ベース)。
    /// The line number in the source code where the error occurred (1-based).
    pub line: u32,

    /// エラーが発生したソースコードの文字数 (1ベース)。
    /// The column number in the source code where the error occurred (1-based).
    pub column: u32,

    /// スタックトレース (関数呼び出しの履歴)。
    /// The stack trace (history of function calls).
    pub trace: Vec<String>,

    /// エラーに関連する追加情報。
    /// Optional additional context related to the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<SnowFallErrorContext>,
}

impl SnowFallError {
    /// 新しいSnowFallErrorインスタンスを作成します。
    /// Creates a new SnowFallError instance.
    pub fn new(
        r#type: String,
        message: String,
        code: String,
        line: u32,
        column: u32,
    ) -> Self {
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
}
