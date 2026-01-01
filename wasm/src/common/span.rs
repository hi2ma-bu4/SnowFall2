use serde::{Deserialize, Serialize};

/// ソースコード上の位置情報
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
