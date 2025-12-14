use crate::common::error::SnowFallError;
use crate::common::object::TypeId;
use std::collections::HashMap;

/// コンパイル時の型定義を表すための簡易的な構造体。
/// A simplified struct to represent a type definition at compile time.
pub struct TypeDefinition {
    pub properties: HashMap<String, TypeId>,
    pub __proto__: Option<TypeId>,
}

/// コンパイラが持つ型システム全体を表す。
/// Represents the entire type system held by the compiler.
pub type TypeSystem = HashMap<TypeId, TypeDefinition>;

/// プロパティアクセスが静的に有効かどうかを検証します。
/// Verifies if a property access is statically valid.
pub fn validate_property_access(
    type_system: &TypeSystem,
    mut current_type_id: TypeId,
    property_name: &str,
) -> Result<(), SnowFallError> {
    loop {
        if let Some(type_def) = type_system.get(&current_type_id) {
            // 1. 現在の型のプロパティをチェック
            if type_def.properties.contains_key(property_name) {
                return Ok(());
            }
            // 2. プロトタイプをたどる
            if let Some(proto_id) = type_def.__proto__ {
                current_type_id = proto_id;
            } else {
                // プロトタイプチェーンの終端
                break;
            }
        } else {
            // 型定義が見つからない
            break;
        }
    }

    Err(SnowFallError::new(
        "CompilationError".to_string(),
        format!("Property '{}' not found on type.", property_name),
        "SF020".to_string(), // 新しいエラーコード
        0,                   // 行番号と列番号はASTから取得する想定
        0,
    ))
}
