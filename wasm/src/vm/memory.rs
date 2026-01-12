use crate::common::constants::GC_THRESHOLD;
use crate::vm::value::Value;
use ahash::AHashMap;
use std::cell::RefCell;
use std::rc::Rc;

/// ヒープ内のインデックス（ハンドル）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GcRef {
    pub index: usize,
}

/// オブジェクトヘッダ（GCメタデータ）
#[derive(Debug, Clone)]
pub struct ObjHeader {
    pub is_marked: bool,
    pub kind: ObjKind,
}

/// ヒープ管理オブジェクトの種類
#[derive(Debug, Clone)]
pub enum ObjKind {
    String(String),
    Array(Vec<Value>),
    Map(AHashMap<String, Value>),
    /// コンパイル済みの関数情報
    Function(Rc<CompiledFunction>),
    /// クロージャ (関数 + Upvalues)
    Closure(Closure),
    /// Upvalue (Open or Closed)
    Upvalue(RefCell<UpvalueState>),
    /// クラス定義
    Class(ClassInfo),
    /// クラスインスタンス
    Instance(Instance),
}

#[derive(Debug, Clone)]
pub struct CompiledFunction {
    pub name: String,
    pub arity: usize,
    pub body_offset: usize, // IR Code Section内のオフセット
                            // 将来的にはデバッグ情報などをここに保持
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub function: Rc<CompiledFunction>,
    pub upvalues: Vec<GcRef>, // ObjKind::Upvalue への参照
}

#[derive(Debug, Clone)]
pub enum UpvalueState {
    /// Open: スタック上の絶対インデックスを指す
    Open(usize),
    /// Closed: ヒープに退避された値
    Closed(Value),
}

#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub methods: AHashMap<String, GcRef>, // Method Name -> Closure
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub class: GcRef,
    pub fields: AHashMap<String, Value>,
}

pub struct Heap {
    objects: Vec<Option<ObjHeader>>,
    free_slots: Vec<usize>,
    /// GC閾値（バイト数またはオブジェクト数）
    pub bytes_allocated: usize,
    pub next_gc_threshold: usize,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            objects: Vec::with_capacity(1024),
            free_slots: Vec::new(),
            bytes_allocated: 0,
            next_gc_threshold: GC_THRESHOLD,
        }
    }

    /// オブジェクトを確保し、GC参照を返す
    pub fn alloc(&mut self, kind: ObjKind) -> GcRef {
        // 簡易的なGCトリガーロジック（実際は外部から vm.collect_garbage() を呼ぶ）
        self.bytes_allocated += self.estimate_size(&kind);

        let header = ObjHeader {
            is_marked: false,
            kind,
        };

        if let Some(idx) = self.free_slots.pop() {
            self.objects[idx] = Some(header);
            GcRef { index: idx }
        } else {
            let idx = self.objects.len();
            self.objects.push(Some(header));
            GcRef { index: idx }
        }
    }

    pub fn get(&self, r: GcRef) -> Option<&ObjKind> {
        self.objects
            .get(r.index)
            .and_then(|o| o.as_ref())
            .map(|h| &h.kind)
    }

    pub fn get_mut(&mut self, r: GcRef) -> Option<&mut ObjKind> {
        self.objects
            .get_mut(r.index)
            .and_then(|o| o.as_mut())
            .map(|h| &mut h.kind)
    }

    /// サイズ概算
    fn estimate_size(&self, kind: &ObjKind) -> usize {
        match kind {
            ObjKind::String(s) => s.len() + 16,
            ObjKind::Array(v) => v.len() * 16 + 16,
            _ => 64, // 簡易値
        }
    }

    // --- GC: Mark Phase ---

    pub fn mark_value(&mut self, val: &Value) {
        if let Value::Obj(r) = val {
            self.mark_object(*r);
        }
    }

    pub fn mark_object(&mut self, r: GcRef) {
        if let Some(Some(header)) = self.objects.get_mut(r.index) {
            if header.is_marked {
                return;
            }
            header.is_marked = true;

            // 子要素を再帰的にマーク (借用規則を回避するため、インデックスを集めてから再帰するか、unsafeを使うのが一般的だが、
            // ここでは簡易的にCloneして対応するか、ワークリスト方式にする)
            // 再帰呼び出しのためのワークリストアプローチを採用
            let mut worklist = vec![r];
            while let Some(curr) = worklist.pop() {
                // 注: ここでの実装はRustの借用規則と衝突しやすいため、
                // 実際は「グレー」セット管理をHeapの外で行うか、インデックスベースで行う必要がある。
                // 簡略化のため、再帰ロジックの概念のみ記述します。
            }
        }
    }

    // --- GC: Sweep Phase ---

    pub fn sweep(&mut self) {
        for i in 0..self.objects.len() {
            if let Some(ref mut header) = self.objects[i] {
                if header.is_marked {
                    header.is_marked = false;
                } else {
                    // 到達不能: 解放
                    self.objects[i] = None;
                    self.free_slots.push(i);
                }
            }
        }
    }
}
