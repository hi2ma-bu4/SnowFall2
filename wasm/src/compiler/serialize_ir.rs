use crate::common::constants::{BINARY_VERSION, MAGIC};
use crate::common::opcode::{Constant, IrModule};

/// Helper to append u32 in Little Endian
fn write_u32_le(vec: &mut Vec<u8>, val: u32) {
    vec.extend_from_slice(&val.to_le_bytes());
}

/// Helper to append f64 in Little Endian
fn write_f64_le(vec: &mut Vec<u8>, val: f64) {
    vec.extend_from_slice(&val.to_le_bytes());
}

/// Helper to append i64 in Little Endian
fn write_i64_le(vec: &mut Vec<u8>, val: i64) {
    vec.extend_from_slice(&val.to_le_bytes());
}

/// irをバイナリデータに変換する
pub fn serialize_ir(ir: IrModule) -> Vec<u8> {
    let mut buf = Vec::new();

    // Header
    buf.extend_from_slice(MAGIC);
    write_u32_le(&mut buf, BINARY_VERSION);

    // Code Section
    write_u32_le(&mut buf, ir.code_section.len() as u32);
    buf.extend_from_slice(&ir.code_section);

    // Constant Pool
    write_u32_le(&mut buf, ir.constant_pool.entries.len() as u32);
    for entry in ir.constant_pool.entries {
        match entry {
            Constant::Int(v) => {
                buf.push(0); // Tag: Int
                write_i64_le(&mut buf, v);
            }
            Constant::Float(v) => {
                buf.push(1); // Tag: Float
                write_f64_le(&mut buf, v);
            }
            Constant::StringPtr(ptr) => {
                buf.push(2); // Tag: StringPtr
                write_u32_le(&mut buf, ptr);
            }
            Constant::Aggregate(ids) => {
                buf.push(3); // Tag: Aggregate
                write_u32_le(&mut buf, ids.len() as u32);
                for id in ids {
                    write_u32_le(&mut buf, id);
                }
            }
        }
    }

    // Naming Table
    write_u32_le(&mut buf, ir.naming_table.data.len() as u32);
    buf.extend_from_slice(&ir.naming_table.data);
    write_u32_le(&mut buf, ir.naming_table.symbols.len() as u32);
    for sym in ir.naming_table.symbols {
        write_u32_le(&mut buf, sym.offset);
        write_u32_le(&mut buf, sym.length);
        buf.push(sym.flags);
    }

    // Debug Section
    write_u32_le(&mut buf, ir.debug_section.delta_encoded_spans.len() as u32);
    buf.extend_from_slice(&ir.debug_section.delta_encoded_spans);

    buf
}
