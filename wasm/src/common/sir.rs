// wasm/src/common/sir.rs

use serde::{Deserialize, Serialize};

use super::object::SnowValue;

/// SIRファイルのヘッダー情報を格納する構造体。
/// Struct to hold the header information of an SIR file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub sir_version: String,
    pub debug_enabled: bool,
    pub source_hash: Option<String>,
}

/// 定数テーブルのエントリを表す。
/// Represents an entry in the constants table.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantEntry {
    pub index: u32,
    pub value: SnowValue,
}

/// 単一のオペコードとオペランドを表す命令。
/// An instruction representing a single opcode and its operands.
#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    pub index: u32,
    pub opcode: String,
    pub operands: Vec<u32>,
}

/// デバッグマップのエントリを表す。
/// Represents an entry in the debug map.
#[derive(Debug, Serialize, Deserialize)]
pub struct DebugMapEntry {
    pub instruction_index_delta: u32,
    pub line_delta: u32,
    pub column_delta: i32,
}

/// SnowFall中間表現（SIR）全体を表現する構造体。
/// The main struct representing the entire SnowFall Intermediate Representation (SIR).
#[derive(Debug, Serialize, Deserialize)]
pub struct Sir {
    pub header: Header,
    pub constants: Vec<ConstantEntry>,
    pub code: Vec<Instruction>,
    pub debug_map: Option<Vec<DebugMapEntry>>,
}
