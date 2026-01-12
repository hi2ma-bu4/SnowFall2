use crate::vm::memory::GcRef;

#[derive(Debug, Clone)]
pub struct CallFrame {
    /// 実行中のクロージャ (Heap上の Closure オブジェクトへの参照)
    pub closure: GcRef,
    /// Instruction Pointer (IRコード内の絶対インデックス)
    pub ip: usize,
    /// Base Pointer (スタック上の開始位置)
    pub bp: usize,
}
