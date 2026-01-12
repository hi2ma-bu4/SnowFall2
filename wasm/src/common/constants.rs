use crate::common::constant_fn::parse_version_u32;

/// 文字列のバージョン情報
pub const VERSION: &str = env!("PKG_VERSION");

/// バイナリのバージョン情報
pub const BINARY_VERSION: u32 = parse_version_u32(VERSION);

/// IRバイナリのヘッダ
pub const MAGIC: &[u8; 4] = b"SNFL";
