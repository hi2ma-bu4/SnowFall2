use crate::common::constant_fn::parse_version_u32;

/// 文字列のバージョン情報
pub const VERSION: &str = env!("PKG_VERSION");

/// バイナリのバージョン情報
pub const BINARY_VERSION: u32 = parse_version_u32(VERSION);

/// IRバイナリのヘッダ
pub const MAGIC: &[u8; 4] = b"SNFL";

pub const MAX_CALL_STACK_DEPTH: usize = 1024;
pub const INITIAL_STACK_SIZE: usize = 2048;
pub const GC_THRESHOLD: usize = 1024 * 1024; // 1MBごとにGC起動など
