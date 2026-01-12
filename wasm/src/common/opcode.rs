use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Opcode {
    // --- スタック操作 (Stack Management) ---
    /// スタックトップを破棄
    Pop = 0x00,
    /// スタックトップを複製
    Dup = 0x01,
    /// スタックトップの2つを入れ替え
    Swap = 0x02,

    // --- リテラル・定数ロード (Load Operations) ---
    // Constant Pool (DAG化/重複排除済み) へのインデックス参照
    /// [Index: LEB128] 定数プールからロード
    LdConst = 0x10,
    /// nullをロード
    LdNull = 0x11,
    /// trueをロード
    LdTrue = 0x12,
    /// falseをロード
    LdFalse = 0x13,

    // --- 変数・シンボル操作 (Variable Operations) ---
    // Naming Table / Local Slot への参照
    /// [Slot: LEB128] ローカル変数の読み込み
    LdLoc = 0x20,
    /// [Slot: LEB128] ローカル変数への書き込み
    StLoc = 0x21,
    /// [NameIdx: LEB128] グローバル変数の読み込み
    LdGlb = 0x22,
    /// [NameIdx: LEB128] グローバル変数への書き込み
    StGlb = 0x23,

    // --- 算術・論理演算 (Arithmetic & Logical) ---
    // 前置演算子
    /// プラス (`+x`)
    Plus = 0x30,
    /// マイナス (`-x`)
    Minus = 0x31,
    /// 論理否定 (`!x`)
    Bang = 0x32,
    /// ビットNOT (`~x`)
    BitNot = 0x33,

    // 中置演算子
    /// 加算 (`+`)
    Add = 0x35,
    /// 減算 (`-`)
    Sub = 0x36,
    /// 乗算 (`*`)
    Mul = 0x37,
    /// 除算 (`/`)
    Div = 0x38,
    /// 剰余 (`%`)
    Mod = 0x39,
    /// べき乗 (`**`)
    Pow = 0x3A,

    /// 等価比較 (`==`)
    Eq = 0x40,
    /// 非等価比較 (`!=`)
    Neq = 0x41,
    /// 厳密等価比較 (`===`)
    StrictEq = 0x42,
    /// 厳密非等価比較 (`!==`)
    StrictNeq = 0x43,
    /// 小なり (`<`)
    Lt = 0x44,
    /// 大なり (`>`)
    Gt = 0x45,
    /// 以下 (`<=`)
    Le = 0x46,
    /// 以上 (`>=`)
    Ge = 0x47,

    /// 論理AND (`&&`)
    LogAndAlso = 0x50,
    /// 論理OR (`||`)
    LogOrElse = 0x51,
    /// 論理AND (`and`)
    LogAnd = 0x52,
    /// 論理OR (`or`)
    LogOr = 0x53,

    /// ビットAND (`&`)
    BitAnd = 0x55,
    /// ビットOR (`|`)
    BitOr = 0x56,
    /// ビットXOR (`^`)
    BitXor = 0x57,
    /// 左シフト (`<<`)
    BitLeftShift = 0x58,
    /// 右シフト (`>>`)
    BitRightShift = 0x59,
    /// 符号なし左シフト (`<<<`)
    BitULeftShift = 0x5A,
    /// 符号なし右シフト (`>>>`)
    BitURightShift = 0x5B,

    // --- 制御フロー (Control Flow) ---
    // 相対ジャンプ (Relative Branching + LEB128)
    /// [Offset: SLEB128] 無条件相対ジャンプ
    Jmp = 0x60,
    /// [Offset: SLEB128] 真なら相対ジャンプ
    JmpIf = 0x61,
    /// [Offset: SLEB128] 偽なら相対ジャンプ
    JmpIfNot = 0x62,

    /// [ArgCount: u8] 関数呼び出し
    Call = 0x63,
    /// 戻り値ありで復帰
    Ret = 0x64,
    /// 戻り値なしで復帰 (Sub用)
    RetVoid = 0x65,

    /// [CaseCount: LEB128, DefaultOffset: SLEB128, Cases...]
    Switch = 0x66,

    // --- オブジェクト・クラス・配列 (Object/Class/Array) ---
    /// [ClassIdx: LEB128, ArgCount: u8] クラスインスタンス化
    New = 0x70,
    /// [PropIdx: LEB128] プロパティ読み込み (インデックスベース最適化)
    LdProp = 0x71,
    /// [PropIdx: LEB128] プロパティ書き込み
    StProp = 0x72,
    /// 配列/オブジェクトの動的アクセス [obj, key]
    LdElem = 0x73,
    /// 配列/オブジェクトの動的代入 [obj, key, val]
    StElem = 0x74,

    /// [Count: LEB128] 配列リテラル生成
    MakeArray = 0x75,
    /// [Count: LEB128] オブジェクトリテラル生成
    MakeObj = 0x76,

    // --- 関数・クラス定義 (Function/Class Definition) ---
    /// [BodyOffset: SLEB128, ArgCount: u8] 関数オブジェクト生成 (Stack: -> [Func])
    MakeFunc = 0x7A,
    /// [NameIdx: LEB128] クラス定義開始 (Stack: [SuperClass] -> [Class])
    DefClass = 0x7B,
    /// [NameIdx: LEB128] メソッド追加 (Stack: [Class, Func] -> [Class])
    AddMethod = 0x7C,

    // --- スコープ・環境 (Scope Management) ---
    /// ブロックスコープ開始
    EnterBlock = 0x80,
    /// ブロックスコープ終了
    ExitBlock = 0x81,

    // --- デバッグ・特殊 (Special) ---
    /// [LineDelta: SLEB128, ColDelta: SLEB128] Span情報更新
    SetSpan = 0x90,
    /// 何もしない
    Nop = 0xFF,
}

/// IRのセクション構造の定義 (分離ストレージ管理用)
pub struct IrModule {
    /// Opcode + Operands (Linearized)
    pub code_section: Vec<u8>,
    /// DAG化された定数 (Int, Float, String)
    pub constant_pool: ConstantPool,
    /// インターン化された識別子
    pub naming_table: NamingTable,
    /// 差分符号化されたSpanデータ
    pub debug_section: DebugSection,
}

/// 重複するリテラルをIDで集約し、ポインタ（インデックス）参照化
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantPool {
    /// 実際の定数データ。インデックスがIDとなる。
    /// 重複排除（Internalization）が行われた後の状態を保持
    pub entries: Vec<Constant>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Constant {
    Int(i64),
    Float(f64),
    /// NamingTable内のオフセットまたはインデックス
    StringPtr(u32),
    /// 複雑な定数構造（配列リテラル等）をDAGとして表現するための再帰参照
    /// [Id1, Id2, Id3] のようにConstantPool内の他IDを指す
    Aggregate(Vec<u32>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamingTable {
    /// 全ての文字列（識別子、プロパティ名、文字列リテラル）を結合した巨大なバッファ
    pub data: Vec<u8>,
    /// 各シンボルのメタデータ (offset, length)
    /// 0番目のインデックスが SymbolID 0 に対応する
    pub symbols: Vec<SymbolEntry>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SymbolEntry {
    pub offset: u32,
    pub length: u32,
    /// 頻出するシンボルかどうかのフラグや、型ヒント（Grammar-aware用）
    pub flags: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugSection {
    /// 差分符号化（Delta Encoding）された位置情報のリスト
    /// フォーマット: [IP差分, 行差分, 列差分]
    /// すべて LEB128/SLEB128 で符号化して Vec<u8> にパックされる
    pub delta_encoded_spans: Vec<u8>,
}
