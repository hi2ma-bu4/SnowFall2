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
#[derive(Debug, Serialize, Deserialize, Clone)]
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

use std::fmt::{self, Display, Formatter};
/// SnowFall中間表現（SIR）全体を表現する構造体。
/// The main struct representing the entire SnowFall Intermediate Representation (SIR).
#[derive(Debug, Serialize, Deserialize)]
pub struct Sir {
    pub header: Header,
    pub constants: Vec<ConstantEntry>,
    pub code: Vec<Instruction>,
    pub debug_map: Option<Vec<DebugMapEntry>>,
}

impl Display for Sir {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // .SNWF section (Header)
        writeln!(
            f,
            ".SNWF {} {} {}",
            self.header.sir_version,
            if self.header.debug_enabled { 1 } else { 0 },
            self.header.source_hash.as_deref().unwrap_or("-")
        )?;

        // .CONST section (Constants)
        if !self.constants.is_empty() {
            writeln!(f, ".CONST")?;
            for c in &self.constants {
                write!(f, "{}:", c.index)?;
                match &c.value {
                    SnowValue::Int(v) => writeln!(f, "I:{}", v)?,
                    SnowValue::Float(v) => writeln!(f, "F:{}", v)?,
                    SnowValue::String(v) => writeln!(f, "S:{}", v)?,
                    SnowValue::Boolean(v) => writeln!(f, "B:{}", if *v { 1 } else { 0 })?,
                    SnowValue::Char(v) => writeln!(f, "C:{}", v)?,
                    // TODO: Handle complex types like Array, Dictionary, etc.
                    _ => writeln!(f, "U:<?>")?, // U for Unimplemented/Unknown
                };
            }
        }

        // .CODE section (Instructions)
        if !self.code.is_empty() {
            writeln!(f, ".CODE")?;
            for i in &self.code {
                write!(f, "{}", i.opcode)?;
                for op in &i.operands {
                    write!(f, " {}", op)?;
                }
                writeln!(f)?;
            }
        }

        // .DEBG section (Debug Map) - Optional
        if let Some(debug_map) = &self.debug_map {
            if !debug_map.is_empty() {
                writeln!(f, ".DEBG")?;
                for entry in debug_map {
                    writeln!(
                        f,
                        "{}:{},{}",
                        entry.instruction_index_delta, entry.line_delta, entry.column_delta
                    )?;
                }
            }
        }

        Ok(())
    }
}
