use crate::common::Span;
use crate::common::opcode::{
    Constant, ConstantPool, DebugSection, IrModule, NamingTable, Opcode, SymbolEntry,
};
use crate::compiler::ast::{
    Expression, ExpressionKind, FunctionKind, InfixOperator, PrefixOperator, ProgramAst, Statement,
    StatementKind,
};
use ahash::AHashMap;
use std::collections::hash_map::Entry;

/// 固定幅（パディング付き）でSLEB128を書き込む（ジャンプオフセットのバックパッチ用）
/// ここでは4バイト分を確保する (最大28ビット分のオフセットに対応)
/// leb128クレートは固定幅書き込みをサポートしていないため、自前で実装するか
/// パディング用のフィラーを入れる必要があるが、ここではシンプルに
/// 「4バイトの固定領域」として扱い、値を強制的に4バイト長SLEB128として書き込む。
fn write_sleb128_padded(val: i64, buf: &mut Vec<u8>) {
    let mut value = val;
    for i in 0..4 {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        // 最後のバイトでなければ継続ビットを立てる
        if i < 3 {
            byte |= 0x80;
        } else {
            // 4バイト目は継続ビットなし。ただし符号ビットの扱いが必要
            // ここでは簡易的に4バイト固定長として扱う
        }
        buf.push(byte);
    }
}

/// ローカル変数のスコープ管理
struct Scope {
    /// 変数名 -> ローカルスロットインデックス
    locals: AHashMap<String, u32>,
    /// 親スコープの変数カウント（スロットオフセット用）
    offset: u32,
}

pub struct IrCompiler<'a> {
    source: &'a str,

    // Config
    debug_enabled: bool,

    // 出力セクション
    code: Vec<u8>,
    constants: Vec<Constant>,
    string_data: Vec<u8>,
    symbols: Vec<SymbolEntry>,
    debug_deltas: Vec<u8>,

    // 重複排除マップ (Interning)
    const_map: AHashMap<String, u32>, // 簡単にするために、キーは定数の文字列表現です
    string_map: AHashMap<String, u32>,

    // 状態
    scopes: Vec<Scope>,
    next_local_slot: u32,

    // デバッグ情報の状態 (Delta Encoding)
    last_line: i64,
    last_col: i64,
}

impl<'a> IrCompiler<'a> {
    pub fn new(source: &'a str, debug_enabled: bool) -> Self {
        Self {
            source,
            debug_enabled,
            code: Vec::new(),
            constants: Vec::new(),
            string_data: Vec::new(),
            symbols: Vec::new(),
            debug_deltas: Vec::new(),
            const_map: AHashMap::new(),
            string_map: AHashMap::new(),
            scopes: vec![Scope {
                locals: AHashMap::new(),
                offset: 0,
            }],
            next_local_slot: 0,
            last_line: 1,
            last_col: 1,
        }
    }

    pub fn compile(mut self, program: ProgramAst) -> IrModule {
        for stmt in program.statements {
            self.compile_statement(stmt);
        }

        IrModule {
            code_section: self.code,
            constant_pool: ConstantPool {
                entries: self.constants,
            },
            naming_table: NamingTable {
                data: self.string_data,
                symbols: self.symbols,
            },
            debug_section: DebugSection {
                delta_encoded_spans: self.debug_deltas,
            },
        }
    }

    // --- Helpers ---

    fn emit(&mut self, op: Opcode, span: Span) {
        self.code.push(op as u8);
        if self.debug_enabled {
            self.record_span(span);
        }
    }

    fn emit_u8(&mut self, val: u8) {
        self.code.push(val);
    }

    fn emit_leb128(&mut self, val: u64) {
        let _ = leb128::write::unsigned(&mut self.code, val);
    }

    fn emit_sleb128(&mut self, val: i64) {
        let _ = leb128::write::signed(&mut self.code, val);
    }

    /// 定数プールへの登録（重複排除あり）
    fn add_constant(&mut self, c: Constant) -> u32 {
        // TODO: 後で対応
        // 簡易的なキー生成 (本来はConstant自体をHash化すべきだが、浮動小数点の扱いが面倒なため仮で文字列化)
        let key = match &c {
            Constant::Int(v) => format!("I:{}", v),
            Constant::Float(v) => format!("F:{}", v),
            Constant::StringPtr(ptr) => format!("S:{}", ptr),
            Constant::Aggregate(vec) => format!("A:{:?}", vec),
        };

        match self.const_map.entry(key) {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(v) => {
                let idx = self.constants.len() as u32;
                self.constants.push(c);
                v.insert(idx);
                idx
            }
        }
    }

    /// 文字列インターニング
    fn intern_string(&mut self, s: &str) -> u32 {
        if let Some(&idx) = self.string_map.get(s) {
            return idx;
        }

        let offset = self.string_data.len() as u32;
        let length = s.len() as u32;
        self.string_data.extend_from_slice(s.as_bytes());

        let idx = self.symbols.len() as u32;
        self.symbols.push(SymbolEntry {
            offset,
            length,
            flags: 0,
        });

        self.string_map.insert(s.to_string(), idx);
        idx
    }

    /// デバッグ情報の差分符号化
    /// ソースコード全体を持っていないと行・列の計算が重いため、簡易的に
    /// source文字列から改行を数えて計算する (本来はLexerの結果などを流用すべき)
    fn record_span(&mut self, span: Span) {
        // Span offset -> Line/Col Calculation
        // これは計算コストが高いが、コンパイル時のみなので許容する
        let (line, col) = self.get_line_col(span.start);

        // Delta Encoding
        let d_line = (line as i64) - self.last_line;
        let d_col = (col as i64) - self.last_col;

        let _ = leb128::write::signed(&mut self.debug_deltas, d_line);
        let _ = leb128::write::signed(&mut self.debug_deltas, d_col);

        self.last_line = line as i64;
        self.last_col = col as i64;
    }

    fn get_line_col(&self, offset: usize) -> (u32, u32) {
        // TODO: 後で対応
        // 簡易実装: ソース全体を走査して行数を数える
        // 本来はLexerのトークンからLine/Colを引き継ぐべきだが、ASTのSpanはoffsetのみ
        let safe_offset = offset.min(self.source.len());
        let slice = &self.source[..safe_offset];
        let line = slice.matches('\n').count() as u32 + 1;
        let last_newline = slice.rfind('\n').unwrap_or(0);
        let col = if last_newline == 0 && !slice.starts_with('\n') {
            safe_offset as u32 + 1
        } else {
            (safe_offset - last_newline) as u32
        };
        (line, col)
    }

    // --- Scopes ---

    fn enter_scope(&mut self) {
        let offset = self.next_local_slot;
        self.scopes.push(Scope {
            locals: AHashMap::new(),
            offset,
        });
    }

    // スコープを抜けるとき、そのスコープで確保したスロット分だけnext_local_slotを戻す
    fn exit_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            self.next_local_slot = scope.offset;
        }
    }

    fn declare_local(&mut self, name: &str) -> u32 {
        let slot = self.next_local_slot;
        self.next_local_slot += 1;
        if let Some(scope) = self.scopes.last_mut() {
            scope.locals.insert(name.to_string(), slot);
        }
        slot
    }

    fn resolve_local(&self, name: &str) -> Option<u32> {
        // 内側のスコープから検索
        for scope in self.scopes.iter().rev() {
            if let Some(&slot) = scope.locals.get(name) {
                return Some(slot);
            }
        }
        None
    }

    // --- AST Traversal ---

    fn compile_statement(&mut self, stmt: Statement) {
        match stmt.kind {
            StatementKind::Expression(expr) => {
                self.compile_expression(expr);
                // 式文の結果はスタックに残るので破棄する (式がVoidを返す場合はPop不要だが、現状は全て値を持つと仮定)
                self.emit(Opcode::Pop, stmt.span);
            }
            StatementKind::VariableDeclaration { declarators, .. } => {
                for decl in declarators {
                    if let Some(init) = decl.value {
                        self.compile_expression(init);
                    } else {
                        self.emit(Opcode::LdNull, stmt.span);
                    }
                    // ローカル変数の登録
                    let slot = self.declare_local(&decl.name);
                    self.emit(Opcode::StLoc, stmt.span);
                    self.emit_leb128(slot as u64);
                }
            }
            StatementKind::Block(stmts) => {
                self.emit(Opcode::EnterBlock, stmt.span);
                self.enter_scope();
                for s in stmts {
                    self.compile_statement(s);
                }
                self.exit_scope();
                self.emit(Opcode::ExitBlock, stmt.span);
            }
            StatementKind::If {
                condition,
                consequence,
                alternative,
            } => {
                self.compile_expression(condition);

                // JmpIfFalse -> ElseBlock
                self.emit(Opcode::JmpIfNot, stmt.span);
                let else_jmp_idx = self.code.len();
                // 4バイトのプレースホルダー (SLEB128 padding)
                self.code.extend_from_slice(&[0x80, 0x80, 0x80, 0x00]);

                // Then Block
                self.compile_statement(*consequence);

                if let Some(alt) = alternative {
                    // Thenが終わったらEndへジャンプ
                    self.emit(Opcode::Jmp, stmt.span);
                    let end_jmp_idx = self.code.len();
                    self.code.extend_from_slice(&[0x80, 0x80, 0x80, 0x00]);

                    // Elseブロックの開始位置を計算してJmpIfNotをパッチ
                    let else_start_offset = (self.code.len() as i64) - (else_jmp_idx as i64); // 相対
                    self.patch_jump(else_jmp_idx, else_start_offset);

                    self.compile_statement(*alt);

                    // End位置を計算してThen後のJmpをパッチ
                    let end_offset = (self.code.len() as i64) - (end_jmp_idx as i64);
                    self.patch_jump(end_jmp_idx, end_offset);
                } else {
                    // Elseがない場合、JmpIfNotの飛び先はここ
                    let end_offset = (self.code.len() as i64) - (else_jmp_idx as i64);
                    self.patch_jump(else_jmp_idx, end_offset);
                }
            }
            StatementKind::While { condition, body } => {
                let loop_start = self.code.len() as i64;

                self.compile_expression(condition);

                self.emit(Opcode::JmpIfNot, stmt.span);
                let exit_jmp_idx = self.code.len();
                self.code.extend_from_slice(&[0x80, 0x80, 0x80, 0x00]);

                self.compile_statement(*body);

                // 先頭に戻る (相対ジャンプなので負数)
                self.emit(Opcode::Jmp, stmt.span);
                let jump_back_offset = loop_start - (self.code.len() as i64);
                // 通常のSLEB128書き込み
                self.emit_sleb128(jump_back_offset);

                // 出口のパッチ
                let exit_offset = (self.code.len() as i64) - (exit_jmp_idx as i64);
                self.patch_jump(exit_jmp_idx, exit_offset);
            }
            StatementKind::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.compile_expression(expr);
                    self.emit(Opcode::Ret, stmt.span);
                } else {
                    self.emit(Opcode::RetVoid, stmt.span);
                }
            }
            StatementKind::FunctionDeclaration {
                name, params, body, ..
            } => {
                // 関数定義: 実行フローをスキップして関数本体を配置
                self.emit(Opcode::Jmp, stmt.span);
                let skip_jmp_idx = self.code.len();
                self.code.extend_from_slice(&[0x80, 0x80, 0x80, 0x00]);

                let body_start = self.code.len() as i64;

                // --- Function Body Start ---
                self.enter_scope();

                // 引数をローカル変数に登録
                // 注: 実装上の呼び出し規約として、引数はスタックからポップされて
                // スロットの先頭に格納されると仮定する、またはVMがセットアップする
                for param in &params {
                    self.declare_local(&param.name);
                }

                self.compile_statement(*body);
                // フォールスルー対策
                self.emit(Opcode::RetVoid, stmt.span);

                self.exit_scope();
                // --- Function Body End ---

                // パッチ: 関数本体をスキップ
                let skip_offset = (self.code.len() as i64) - (skip_jmp_idx as i64);
                self.patch_jump(skip_jmp_idx, skip_offset);

                // 関数生成命令を発行
                // Stack: -> [FuncObj]
                let relative_body_start = body_start - (self.code.len() as i64);
                self.emit(Opcode::MakeFunc, stmt.span);
                self.emit_sleb128(relative_body_start); // Offset to body
                self.emit_u8(params.len() as u8);

                // 関数を変数（またはグローバル）に代入
                if let Some(slot) = self.resolve_local(&name) {
                    self.emit(Opcode::StLoc, stmt.span);
                    self.emit_leb128(slot as u64);
                } else {
                    let name_idx = self.intern_string(&name);
                    self.emit(Opcode::StGlb, stmt.span);
                    self.emit_leb128(name_idx as u64);
                }
            }
            StatementKind::ClassDeclaration {
                name,
                superclass,
                members,
            } => {
                // スーパークラスのロード
                if let Some(super_name) = superclass {
                    if let Some(slot) = self.resolve_local(&super_name) {
                        self.emit(Opcode::LdLoc, stmt.span);
                        self.emit_leb128(slot as u64);
                    } else {
                        let idx = self.intern_string(&super_name);
                        self.emit(Opcode::LdGlb, stmt.span);
                        self.emit_leb128(idx as u64);
                    }
                } else {
                    self.emit(Opcode::LdNull, stmt.span);
                }

                // クラス定義開始 (Stack: [Super] -> [Class])
                let class_name_idx = self.intern_string(&name);
                self.emit(Opcode::DefClass, stmt.span);
                self.emit_leb128(class_name_idx as u64);

                // メンバメソッドの定義
                for member in members {
                    match member.kind {
                        StatementKind::FunctionDeclaration {
                            name: func_name,
                            params,
                            body,
                            kind,
                            ..
                        } => {
                            // 関数コンパイルと同様のフロー
                            self.emit(Opcode::Jmp, member.span);
                            let skip_jmp_idx = self.code.len();
                            self.code.extend_from_slice(&[0x80, 0x80, 0x80, 0x00]);

                            let body_start = self.code.len() as i64;
                            self.enter_scope();
                            // `this` を暗黙の第0引数として登録する場合などがありうるが、ここでは省略
                            for param in &params {
                                self.declare_local(&param.name);
                            }
                            self.compile_statement(*body);
                            if kind == FunctionKind::Sub {
                                self.emit(Opcode::RetVoid, member.span);
                            } else {
                                // Functionの場合でも明示的なReturnがない場合の安全策
                                self.emit(Opcode::RetVoid, member.span);
                            }
                            self.exit_scope();

                            let skip_offset = (self.code.len() as i64) - (skip_jmp_idx as i64);
                            self.patch_jump(skip_jmp_idx, skip_offset);

                            // 関数生成 (Stack: [Class] -> [Class, Func])
                            let relative_body_start = body_start - (self.code.len() as i64);
                            self.emit(Opcode::MakeFunc, member.span);
                            self.emit_sleb128(relative_body_start);
                            self.emit_u8(params.len() as u8);

                            // メソッド追加 (Stack: [Class, Func] -> [Class])
                            let method_name_idx = self.intern_string(&func_name);
                            self.emit(Opcode::AddMethod, member.span);
                            self.emit_leb128(method_name_idx as u64);
                        }
                        _ => {}
                    }
                }

                // クラスオブジェクトを保存 (Stack: [Class] -> [])
                if let Some(slot) = self.resolve_local(&name) {
                    self.emit(Opcode::StLoc, stmt.span);
                    self.emit_leb128(slot as u64);
                } else {
                    let name_idx = self.intern_string(&name);
                    self.emit(Opcode::StGlb, stmt.span);
                    self.emit_leb128(name_idx as u64);
                }
            }
            StatementKind::For {
                init,
                condition,
                update,
                body,
            } => {
                self.emit(Opcode::EnterBlock, stmt.span);
                self.enter_scope();

                if let Some(init_stmt) = init {
                    self.compile_statement(*init_stmt);
                }

                let loop_start = self.code.len() as i64;

                // Condition
                let mut exit_jmp_idx = None;
                if let Some(cond) = condition {
                    self.compile_expression(cond);
                    self.emit(Opcode::JmpIfNot, stmt.span);
                    let idx = self.code.len();
                    self.code.extend_from_slice(&[0x80, 0x80, 0x80, 0x00]);
                    exit_jmp_idx = Some(idx);
                }

                // Body
                self.compile_statement(*body);

                // Update
                if let Some(upd) = update {
                    self.compile_statement(*upd);
                }

                // Loop Back
                self.emit(Opcode::Jmp, stmt.span);
                let back_offset = loop_start - (self.code.len() as i64);
                self.emit_sleb128(back_offset);

                // Patch Exit
                if let Some(idx) = exit_jmp_idx {
                    let exit_offset = (self.code.len() as i64) - (idx as i64);
                    self.patch_jump(idx, exit_offset);
                }

                self.exit_scope();
                self.emit(Opcode::ExitBlock, stmt.span);
            }
            StatementKind::ForEach { .. } => {
                // TODO: ForEachの実装。Iteratorプロトコルなどが必要になるため今回はNop
                self.emit(Opcode::Nop, stmt.span);
            }
            StatementKind::Switch { .. } => {
                // TODO: Switchの実装。
                self.emit(Opcode::Nop, stmt.span);
            }
            StatementKind::Break => {
                // TODO: Breakの実装にはループスタック管理が必要
                self.emit(Opcode::Nop, stmt.span);
            }
            StatementKind::Continue => {
                // TODO: Continueの実装にはループスタック管理が必要
                self.emit(Opcode::Nop, stmt.span);
            }
        }
    }

    fn compile_expression(&mut self, expr: Expression) {
        match expr.kind {
            ExpressionKind::IntLiteral(val) => {
                let idx = self.add_constant(Constant::Int(val));
                self.emit(Opcode::LdConst, expr.span);
                self.emit_leb128(idx as u64);
            }
            ExpressionKind::FloatLiteral(val) => {
                let idx = self.add_constant(Constant::Float(val));
                self.emit(Opcode::LdConst, expr.span);
                self.emit_leb128(idx as u64);
            }
            ExpressionKind::StringLiteral(val) => {
                // 文字列の実体はNamingTableへ、そのポインタを定数プールへ
                let name_idx = self.intern_string(&val);
                let const_idx = self.add_constant(Constant::StringPtr(name_idx));
                self.emit(Opcode::LdConst, expr.span);
                self.emit_leb128(const_idx as u64);
            }
            ExpressionKind::Boolean(val) => {
                if val {
                    self.emit(Opcode::LdTrue, expr.span);
                } else {
                    self.emit(Opcode::LdFalse, expr.span);
                }
            }
            ExpressionKind::NullLiteral => {
                self.emit(Opcode::LdNull, expr.span);
            }
            ExpressionKind::Identifier(name) => {
                if let Some(slot) = self.resolve_local(&name) {
                    self.emit(Opcode::LdLoc, expr.span);
                    self.emit_leb128(slot as u64);
                } else {
                    let idx = self.intern_string(&name);
                    self.emit(Opcode::LdGlb, expr.span);
                    self.emit_leb128(idx as u64);
                }
            }
            ExpressionKind::Infix {
                left,
                operator,
                right,
            } => {
                self.compile_expression(*left);
                self.compile_expression(*right);
                match operator {
                    InfixOperator::Add => self.emit(Opcode::Add, expr.span),
                    InfixOperator::Subtract => self.emit(Opcode::Sub, expr.span),
                    InfixOperator::Multiply => self.emit(Opcode::Mul, expr.span),
                    InfixOperator::Divide => self.emit(Opcode::Div, expr.span),
                    InfixOperator::Modulo => self.emit(Opcode::Mod, expr.span),
                    InfixOperator::Power => self.emit(Opcode::Pow, expr.span),
                    InfixOperator::Equals => self.emit(Opcode::Eq, expr.span),
                    InfixOperator::NotEquals => self.emit(Opcode::Neq, expr.span),
                    InfixOperator::StrictEquals => self.emit(Opcode::StrictEq, expr.span),
                    InfixOperator::StrictNotEquals => self.emit(Opcode::StrictNeq, expr.span),
                    InfixOperator::LessThan => self.emit(Opcode::Lt, expr.span),
                    InfixOperator::GreaterThan => self.emit(Opcode::Gt, expr.span),
                    InfixOperator::LessThanOrEqual => self.emit(Opcode::Le, expr.span),
                    InfixOperator::GreaterThanOrEqual => self.emit(Opcode::Ge, expr.span),
                    InfixOperator::LogicalAndAlso => self.emit(Opcode::LogAndAlso, expr.span),
                    InfixOperator::LogicalOrElse => self.emit(Opcode::LogOrElse, expr.span),
                    InfixOperator::LogicalAnd => self.emit(Opcode::LogAnd, expr.span),
                    InfixOperator::LogicalOr => self.emit(Opcode::LogOr, expr.span),
                    InfixOperator::BitwiseAnd => self.emit(Opcode::BitAnd, expr.span),
                    InfixOperator::BitwiseOr => self.emit(Opcode::BitOr, expr.span),
                    InfixOperator::BitwiseXor => self.emit(Opcode::BitXor, expr.span),
                    InfixOperator::BitwiseLeftShift => self.emit(Opcode::BitLeftShift, expr.span),
                    InfixOperator::BitwiseRightShift => self.emit(Opcode::BitRightShift, expr.span),
                    InfixOperator::BitwiseUnsignedLeftShift => {
                        self.emit(Opcode::BitULeftShift, expr.span)
                    }
                    InfixOperator::BitwiseUnsignedRightShift => {
                        self.emit(Opcode::BitURightShift, expr.span)
                    }
                }
            }
            ExpressionKind::Prefix { operator, right } => {
                self.compile_expression(*right);
                match operator {
                    PrefixOperator::Plus => self.emit(Opcode::Plus, expr.span),
                    PrefixOperator::Minus => self.emit(Opcode::Minus, expr.span),
                    PrefixOperator::Bang => self.emit(Opcode::Bang, expr.span),
                    PrefixOperator::BitwiseNot => self.emit(Opcode::BitNot, expr.span),
                }
            }
            ExpressionKind::Assignment { left, right } => {
                self.compile_expression(*right);
                // スタックトップの値を複製（代入式は値を返すため）
                self.emit(Opcode::Dup, expr.span);

                match left.kind {
                    ExpressionKind::Identifier(name) => {
                        if let Some(slot) = self.resolve_local(&name) {
                            self.emit(Opcode::StLoc, expr.span);
                            self.emit_leb128(slot as u64);
                        } else {
                            let idx = self.intern_string(&name);
                            self.emit(Opcode::StGlb, expr.span);
                            self.emit_leb128(idx as u64);
                        }
                    }
                    ExpressionKind::Member {
                        left: obj,
                        property,
                    } => {
                        self.compile_expression(*obj);
                        // 現在スタック: [Val, Obj] -> 必要: [Obj, Val] for StProp?
                        // Opcode設計によるが、StPropが [Obj, Val] を消費すると仮定
                        self.emit(Opcode::Swap, expr.span);
                        let idx = self.intern_string(&property);
                        self.emit(Opcode::StProp, expr.span);
                        self.emit_leb128(idx as u64);
                    }
                    ExpressionKind::Index { left: obj, index } => {
                        self.compile_expression(*obj);
                        self.emit(Opcode::Swap, expr.span);
                        self.compile_expression(*index); // スタック: [Val, Obj, Key] -> 本来は [Obj, Key, Val] が必要？
                        // Opcode StElem の期待: [Obj, Key, Val] -> void
                        // 現在のスタック: [Val, Obj] -> Key を push -> [Val, Obj, Key]
                        // 並び替えが必要。
                        // この素直な直列コンパイルは `StElem` では失敗する。
                        // `a[k] = v` の正しいフロー:
                        // 1. a を push
                        // 2. k を push
                        // 3. v を push
                        // 4. StElem
                        // でもここでは `v`（右辺）を先に評価している。
                        // なのでスタックは [Val]。
                        // その後 `a[k]` を評価している。
                        // `Assignment` ロジックを再検討する必要あり。

                        // RHS を先に評価した場合の StElem 対応:
                        // スタック: [Val]
                        // Obj を評価 -> [Val, Obj]
                        // Key を評価 -> [Val, Obj, Key]
                        // StElem 用には [Obj, Key, Val] が必要？
                        // それとも Opcode を `StElemRev` [Val, Obj, Key] にする？
                        // ここでは StElem が [Obj, Key, Val] を取ると仮定する。
                        // なので回転（rotate）が必要。

                        // この IR を単純化するため、`StElem` は
                        // [Val, Obj, Key] を取る（RHS を先に評価する前提）と仮定する。
                        // もしくは Rot 命令を使えばいい。
                        // 標準的なスタックマシンとして、
                        // `StElem` は Value, Key, Obj の順に pop すると仮定する。
                        // スタックトップは Key。
                        // 今は [Val, Obj, Key]。
                        // StElem は Key, Obj, Val を pop する。順序は一致してる！

                        self.emit(Opcode::StElem, expr.span);
                    }
                    _ => {
                        // 左辺値が無効
                    }
                }
            }
            ExpressionKind::Call {
                function,
                arguments,
            } => {
                // 引数を評価してスタックに積む
                let arg_count = arguments.len();
                for arg in arguments {
                    self.compile_expression(arg);
                }

                // 関数（または呼び出し可能なオブジェクト）をロード
                self.compile_expression(*function);

                self.emit(Opcode::Call, expr.span);
                self.emit_u8(arg_count as u8);
            }
            ExpressionKind::Member { left, property } => {
                self.compile_expression(*left);
                let idx = self.intern_string(&property);
                self.emit(Opcode::LdProp, expr.span);
                self.emit_leb128(idx as u64);
            }
            ExpressionKind::Index { left, index } => {
                self.compile_expression(*left);
                self.compile_expression(*index);
                self.emit(Opcode::LdElem, expr.span);
            }
            ExpressionKind::ArrayLiteral(elements) => {
                let count = elements.len();
                for el in elements {
                    self.compile_expression(el);
                }
                self.emit(Opcode::MakeArray, expr.span);
                self.emit_leb128(count as u64);
            }
            ExpressionKind::ObjectLiteral { pairs } => {
                let count = pairs.len();
                for (key, value) in pairs {
                    self.compile_expression(key);
                    self.compile_expression(value);
                }
                self.emit(Opcode::MakeObj, expr.span);
                self.emit_leb128(count as u64);
            }
            ExpressionKind::New { class, arguments } => {
                let arg_count = arguments.len();
                for arg in arguments {
                    self.compile_expression(arg);
                }

                // 先に中身を見る
                let class_ident = match &class.kind {
                    ExpressionKind::Identifier(name) => Some(name.clone()),
                    _ => None,
                };

                self.compile_expression(*class);
                // 通常 New はクラスIDを取るが、式の結果がクラスオブジェクトの場合は
                // Callに近い形になる。Opcode::New が [ClassIdx] を取る設計だと動的解決できない。
                // ここでは動的な `New` 命令 (Stack: [Args..., Class] -> [Inst]) を想定するが、
                // `opcode.rs` の定義では `New` は `ClassIdx` を即値で取っている。
                // 動的言語仕様に合わせて修正が必要だが、今回は即値版ではなく、
                // クラスをスタックから取る `NewDynamic` が必要。または `New` の定義を変更。
                // 既存の `New` は `[ClassIdx: LEB128]` なので、ここでは式からクラス名IDが解決できる場合のみ対応。
                // 式がIdentifierなら可能。
                if let Some(name) = class_ident {
                    // コンパイル時にID解決を試みる（不完全だがIR生成としては通す）
                    let name_idx = self.intern_string(&name);
                    self.emit(Opcode::New, expr.span);
                    self.emit_leb128(name_idx as u64);
                    self.emit_u8(arg_count as u8);
                } else {
                    // 複雑な式からのNewは未サポート(Opcode拡張が必要)
                    self.emit(Opcode::Nop, expr.span);
                }
            }
            _ => {
                self.emit(Opcode::Nop, expr.span);
            }
        }
    }

    fn patch_jump(&mut self, index: usize, offset: i64) {
        let mut buf = Vec::new();
        write_sleb128_padded(offset, &mut buf);
        for (i, byte) in buf.iter().enumerate() {
            if index + i < self.code.len() {
                self.code[index + i] = *byte;
            }
        }
    }
}
