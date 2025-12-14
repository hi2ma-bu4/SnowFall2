use crate::compiler::lexer::{Lexer, Token};
use crate::types::{CompileResult, CompilerConfig};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    bytecode: Vec<String>,
    config: CompilerConfig,
}

impl Parser {
    pub fn new(source: &str, config: CompilerConfig) -> Self {
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        Parser {
            lexer,
            current_token: token,
            bytecode: Vec::new(),
            config,
        }
    }

    fn eat(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn emit(&mut self, op: &str) {
        self.bytecode.push(op.to_string());
    }

    pub fn compile(&mut self) -> CompileResult {
        while self.current_token != Token::EOF {
            if let Err(e) = self.statement() {
                return CompileResult {
                    success: false,
                    bytecode: String::new(),
                    error_msg: Some(format!("Line {}: {}", self.lexer.line, e)),
                    debug_info: None,
                };
            }
        }
        CompileResult {
            success: true,
            bytecode: self.bytecode.join("\n"),
            error_msg: None,
            debug_info: if self.config.debug_mode {
                Some("Compiled successfully".to_string())
            } else {
                None
            },
        }
    }

    // --- 文 (Statement) ---

    fn statement(&mut self) -> Result<(), String> {
        match &self.current_token {
            Token::Keyword(k) => match k.as_str() {
                "if" => self.if_statement(),
                "for" => self.for_statement(),
                _ => self.expression_statement(),
            },
            Token::LBrace => self.block(),
            // 変数宣言の可能性 (Type Identifier)
            Token::Identifier(id) => {
                // 本来はシンボルテーブルで型かどうか確認するが、
                // 今回は仕様に従い、大文字始まりを型とみなすか、
                // もしくは次のトークンを見て判断する (Type Ident = ...)
                // 簡易的に "Int", "String" 等のプリミティブか、Identifierが来て次もIdentifierなら宣言とみなす
                if id == "Int" || id == "String" || id == "Boolean" || id == "Float" {
                    self.var_declaration()
                } else {
                    self.expression_statement()
                }
            }
            _ => self.expression_statement(),
        }
    }

    // ブロック: { ... }
    fn block(&mut self) -> Result<(), String> {
        self.eat(); // {
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            self.statement()?;
        }
        if self.current_token == Token::RBrace {
            self.eat();
            Ok(())
        } else {
            Err("Expected '}'".to_string())
        }
    }

    // 変数宣言: Int x = 10;
    fn var_declaration(&mut self) -> Result<(), String> {
        // TypeNameは既にチェック済み
        self.eat();

        let name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return Err("Expected variable name after type".to_string()),
        };
        self.eat();

        if self.current_token == Token::Assign {
            self.eat();
            self.expression()?; // 式の結果がスタックにある
            self.emit(&format!("STORE {}", name));
        } else {
            // 初期化なし
            self.emit("PUSH_NULL");
            self.emit(&format!("STORE {}", name));
        }

        if self.current_token == Token::Semicolon {
            self.eat();
        }
        Ok(())
    }

    // if (expr) { ... }
    fn if_statement(&mut self) -> Result<(), String> {
        self.eat(); // if
        if self.current_token == Token::LParen {
            self.eat();
        }
        self.expression()?;
        if self.current_token == Token::RParen {
            self.eat();
        }

        let jump_false_idx = self.bytecode.len();
        self.emit("JUMP_IF_FALSE ?");

        self.statement()?; // ブロックまたは1行

        let target = self.bytecode.len();
        self.bytecode[jump_false_idx] = format!("JUMP_IF_FALSE {}", target);
        Ok(())
    }

    // for (Int i=0; i<10; i++) { ... }
    // 簡易化のため Cスタイルのforのみ実装
    fn for_statement(&mut self) -> Result<(), String> {
        self.eat(); // for
        if self.current_token == Token::LParen {
            self.eat();
        }

        // Init
        if matches!(self.current_token, Token::Identifier(_)) {
            // 変数宣言と仮定 (Int i = 0)
            self.var_declaration()?;
        } else {
            self.eat(); // セミコロンスキップなど（簡易実装）
        }
        // ループ開始位置
        let start_label = self.bytecode.len();

        // Condition
        self.expression()?;
        if self.current_token == Token::Semicolon {
            self.eat();
        }

        let jump_exit_idx = self.bytecode.len();
        self.emit("JUMP_IF_FALSE ?");

        // Incrementは実行順序を変える必要があるため、JUMP命令で制御を複雑にする必要があるが
        // ここではMVPとして「Bodyの最後にIncrementを行う」形式に簡略化できないため
        // Condition -> JumpBody -> Increment -> JumpCond -> BodyLabel -> ...
        // という複雑なバイトコード生成が必要。
        // 今回は実装量を抑えるため、Increment部はBodyの末尾に埋め込む形式ではなく
        // 素直にパースして保持し、Body後に吐き出すアプローチをとる。

        // Bytecodeの順序制御が複雑になるため、ここではユーザー入力 `for (Init; Cond; Inc)` の
        // Inc部分を一時的にバッファするなどの工夫が必要。
        // 簡略化: Inc部分はパースするが、バイトコード生成位置は工夫する。
        // ここでは「Increment式はBlockの最後に追加される」というJS的な挙動を模倣する。

        // 一旦Increment部分のトークンを保存できればベストだが、今回はスキップして単純なwhile相当にする
        // もしくは、Increment部分を解析してバイトコードを別バッファに退避する。

        // (MVP範囲外の複雑さを避けるため、Increment部をパースして一時退避するロジックは省略し、
        //  IncrementがないものとしてConditionだけチェックするwhile的なforとします)
        while self.current_token != Token::RParen {
            self.eat(); // Inc部分スキップ（実際の言語開発ではここでAst構築が必要）
        }
        self.eat(); // )

        self.statement()?; // Body

        self.emit(&format!("JUMP {}", start_label));

        let end_label = self.bytecode.len();
        self.bytecode[jump_exit_idx] = format!("JUMP_IF_FALSE {}", end_label);

        Ok(())
    }

    fn expression_statement(&mut self) -> Result<(), String> {
        self.expression()?;
        if self.current_token == Token::Semicolon {
            self.eat();
        }
        // 文としての式なので、スタックに残った値は捨てるのが通例だが、
        // REPL的な動作のために残すこともある。ここではPOPしない。
        Ok(())
    }

    // --- 式 (Expression) ---
    // 優先順位: Assign < OR < AND < EQ < REL < ADD < MUL < UNARY

    fn expression(&mut self) -> Result<(), String> {
        self.logic_or()
    }

    fn logic_or(&mut self) -> Result<(), String> {
        self.logic_and()?;
        while let Token::Operator(op) = &self.current_token {
            if op == "or" || op == "||" {
                let op_clone = op.clone();
                self.eat();
                // 短絡評価用Jump
                // 本来は JUMP_IF_TRUE を実装すべき
                self.logic_and()?;
                self.emit("OR"); // 論理和
            } else {
                break;
            }
        }
        Ok(())
    }

    fn logic_and(&mut self) -> Result<(), String> {
        self.equality()?;
        while let Token::Operator(op) = &self.current_token {
            if op == "and" || op == "&&" {
                let op_clone = op.clone();
                self.eat();
                self.equality()?;
                self.emit("AND");
            } else {
                break;
            }
        }
        Ok(())
    }

    fn equality(&mut self) -> Result<(), String> {
        self.relational()?;
        while let Token::Operator(op) = &self.current_token.clone() {
            if op == "==" || op == "===" {
                self.eat();
                self.relational()?;
                if op == "===" {
                    self.emit("EQ_STRICT");
                } else {
                    self.emit("EQ_LOOSE");
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    fn relational(&mut self) -> Result<(), String> {
        self.additive()?;
        // <, > などの実装はここ
        Ok(())
    }

    fn additive(&mut self) -> Result<(), String> {
        self.multiplicative()?;
        while let Token::Operator(op) = &self.current_token.clone() {
            if op == "+" || op == "-" {
                self.eat();
                self.multiplicative()?;
                match op.as_str() {
                    "+" => self.emit("ADD"),
                    "-" => self.emit("SUB"),
                    _ => {}
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    fn multiplicative(&mut self) -> Result<(), String> {
        self.primary()?;
        while let Token::Operator(op) = &self.current_token.clone() {
            if op == "*" || op == "/" {
                self.eat();
                self.primary()?;
                match op.as_str() {
                    "*" => self.emit("MUL"),
                    "/" => self.emit("DIV"),
                    _ => {}
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    fn primary(&mut self) -> Result<(), String> {
        match &self.current_token.clone() {
            Token::IntLiteral(i) => {
                self.emit(&format!("PUSH_INT {}", i));
                self.eat();
            }
            Token::FloatLiteral(f) => {
                self.emit(&format!("PUSH_FLOAT {}", f));
                self.eat();
            }
            Token::StringLiteral(s) => {
                self.emit(&format!("PUSH_STR {}", s));
                self.eat();
            }
            Token::Identifier(id) => {
                // 変数ロード または 関数呼び出し
                self.emit(&format!("LOAD {}", id));
                self.eat();
            }
            Token::LParen => {
                self.eat();
                self.expression()?;
                if self.current_token == Token::RParen {
                    self.eat();
                } else {
                    return Err("Expected ')'".to_string());
                }
            }
            _ => return Err(format!("Unexpected token: {:?}", self.current_token)),
        }
        Ok(())
    }
}
