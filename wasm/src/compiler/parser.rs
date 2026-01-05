use crate::common::error::SnowFallError;
use crate::common::{
    DelimiterToken, KeywordToken, LiteralToken, OperatorToken, Span, Token, TokenKind,
};
use crate::compiler::Lexer;
use crate::compiler::ast::{
    Binding, Expression, ExpressionKind, ForEachKind, FunctionKind, InfixOperator, Parameter,
    PrefixOperator, ProgramAst, Statement, StatementKind, VariableDeclarator,
};

/// 演算の優先順位
#[derive(PartialEq, PartialOrd)]
enum Precedence {
    /// 最低優先度
    Lowest,

    /// 代入演算子 (`=`)
    Assign,

    /// 論理OR (`||`)
    LogicalOr,
    /// 論理AND (`&&`)
    LogicalAnd,

    /// ビットOR (`|`)
    BitOr,
    /// ビットXOR (`^`)
    BitXor,
    /// ビットAND (`&`)
    BitAnd,

    /// 等価比較 (`==`, `!=`)
    Equals,
    /// 大小比較 (`<`, `<=`, `>`, `>=`)
    LessGreater,

    /// ビットシフト (`<<`, `>>`)
    Shift,

    /// 加減算 (`+`, `-`)
    Sum,
    /// 乗除算 (`*`, `/`, `%`)
    Product,

    /// べき乗 (`**`)
    Power,

    /// 前置演算子 (`-x`, `!x`)
    Prefix,
    /// 関数呼び出し (`fn()`)
    Call,
}

/// 中置演算子用の一時enum
enum InfixOpToken {
    Operator(OperatorToken),
    Keyword(KeywordToken),
}

/// パーサ内部で使用するResult型
type ParseResult<T> = Result<T, SnowFallError>;

/// 字句解析器(Lexer)を入力としてASTを構築する構文解析器
pub struct Parser<'a> {
    /// 字句解析器
    lexer: Lexer<'a>,
    /// 現在処理中のトークン
    cur_token: Token,
    /// 先読みトークン
    peek_token: Token,
    /// パース中に蓄積されたエラー
    errors: Vec<SnowFallError>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut p = Parser {
            lexer,
            cur_token: Token::eof(0),
            peek_token: Token::eof(0),
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        p
    }

    /// トークンを1つ進める
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        loop {
            match self.lexer.next_token() {
                Ok(token) => {
                    self.peek_token = token;
                    break;
                }
                Err(e) => {
                    self.errors.push(e);
                }
            }
        }
    }

    // ===== ヘルパーメソッド =====

    /// 次のトークンが指定した `TokenKind` と一致するか判定する
    fn peek_token_is(&self, kind: &TokenKind) -> bool {
        match (&self.peek_token.kind, kind) {
            (TokenKind::Identifier(_), TokenKind::Identifier(_)) => true,
            (TokenKind::Delimiter(d1), TokenKind::Delimiter(d2)) => d1 == d2,
            (TokenKind::Keyword(k1), TokenKind::Keyword(k2)) => k1 == k2,
            (TokenKind::Operator(o1), TokenKind::Operator(o2)) => o1 == o2,
            _ => false,
        }
    }

    /// 次のトークンが期待通りであれば消費する
    fn expect_peek(&mut self, expected: TokenKind) -> ParseResult<()> {
        if self.peek_token_is(&expected) {
            self.next_token();
            Ok(())
        } else {
            Err(SnowFallError::new_compiler_error(
                format!(
                    "Expected next token to be {:?}, got {:?} instead",
                    expected, self.peek_token.kind
                ),
                "SF0010".to_string(),
                self.lexer.line,
                self.lexer.column,
            ))
        }
    }

    /// 現在トークンの優先順位を取得する
    fn cur_precedence(&self) -> Precedence {
        self.token_precedence(&self.cur_token.kind)
    }

    /// 次トークンの優先順位を取得する
    fn peek_precedence(&self) -> Precedence {
        self.token_precedence(&self.peek_token.kind)
    }

    /// `TokenKind` から対応する優先順位を返す
    fn token_precedence(&self, kind: &TokenKind) -> Precedence {
        match kind {
            TokenKind::Operator(op) => match op {
                OperatorToken::Assign => Precedence::Assign,
                OperatorToken::LogicalOr => Precedence::LogicalOr,
                OperatorToken::LogicalAnd => Precedence::LogicalAnd,
                OperatorToken::BitwiseOr => Precedence::BitOr,
                OperatorToken::BitwiseXor => Precedence::BitXor,
                OperatorToken::BitwiseAnd => Precedence::BitAnd,
                OperatorToken::Equal
                | OperatorToken::NotEqual
                | OperatorToken::StrictEqual
                | OperatorToken::StrictNotEqual => Precedence::Equals,
                OperatorToken::BitwiseLeftShift
                | OperatorToken::BitwiseUnsignedLeftShift
                | OperatorToken::BitwiseRightShift
                | OperatorToken::BitwiseUnsignedRightShift => Precedence::Shift,
                OperatorToken::LessThan
                | OperatorToken::LessThanOrEqual
                | OperatorToken::GreaterThan
                | OperatorToken::GreaterThanOrEqual => Precedence::LessGreater,
                OperatorToken::Plus | OperatorToken::Minus => Precedence::Sum,
                OperatorToken::Asterisk | OperatorToken::Slash | OperatorToken::Percent => {
                    Precedence::Product
                }
                OperatorToken::Power => Precedence::Power,

                _ => Precedence::Lowest,
            },
            TokenKind::Delimiter(DelimiterToken::LParen) => Precedence::Call,
            TokenKind::Keyword(kw) => match kw {
                KeywordToken::Or => Precedence::LogicalOr,
                KeywordToken::And => Precedence::LogicalAnd,

                _ => Precedence::Lowest,
            },
            _ => Precedence::Lowest,
        }
    }

    // ===== エントリーポイント =====

    /// ソース全体を解析し `Program` を生成する
    pub fn parse_program(&mut self) -> Result<ProgramAst, Vec<SnowFallError>> {
        let mut statements = Vec::new();
        let start = self.cur_token.span.start;

        while self.cur_token.kind != TokenKind::Eof {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    self.errors.push(e);
                    self.next_token(); // 簡易的なエラーリカバリ
                }
            }
            self.next_token();
        }

        if !self.errors.is_empty() {
            return Err(self.errors.drain(..).collect());
        }

        let end = if !statements.is_empty() {
            statements.last().unwrap().span.end
        } else {
            start
        };

        Ok(ProgramAst {
            statements,
            span: Span { start, end },
        })
    }

    /// 1文（Statement）を解析する
    fn parse_statement(&mut self) -> ParseResult<Statement> {
        match self.cur_token.kind {
            TokenKind::Keyword(KeywordToken::Function) => self.parse_function_declaration(),
            TokenKind::Keyword(KeywordToken::Sub) => self.parse_sub_declaration(),
            TokenKind::Keyword(KeywordToken::Class) => self.parse_class_declaration(),
            TokenKind::Keyword(KeywordToken::For) => self.parse_for_statement(),
            TokenKind::Keyword(KeywordToken::If) => self.parse_if_statement(),
            TokenKind::Keyword(KeywordToken::While) => self.parse_while_statement(),
            TokenKind::Keyword(KeywordToken::Return) => self.parse_return_statement(),
            TokenKind::Delimiter(DelimiterToken::LBrace) => self.parse_block_statement(),
            TokenKind::Identifier(_) => {
                // "Int a" のように「識別子 -> 識別子」なら変数宣言とみなす
                if self.is_variable_declaration() {
                    self.parse_variable_declaration()
                } else {
                    self.parse_expression_statement()
                }
            }
            _ => self.parse_expression_statement(),
        }
    }

    /// ブロックコード解析 `{ ... }`
    fn parse_block_statement(&mut self) -> ParseResult<Statement> {
        let start = self.cur_token.span.start;
        self.next_token(); // consume '{'

        let mut statements = Vec::new();
        while self.cur_token.kind != TokenKind::Delimiter(DelimiterToken::RBrace)
            && self.cur_token.kind != TokenKind::Eof
        {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }

        Ok(Statement {
            kind: StatementKind::Block(statements),
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// 式のみからなる文（ExpressionStatement）を解析する
    fn parse_expression_statement(&mut self) -> ParseResult<Statement> {
        let start = self.cur_token.span.start;
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&TokenKind::Delimiter(DelimiterToken::Semicolon)) {
            self.next_token();
        }

        Ok(Statement {
            kind: StatementKind::Expression(expr),
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// 現在のトークンが型名で、次が変数名かどうかを判定する
    fn is_variable_declaration(&self) -> bool {
        matches!(self.cur_token.kind, TokenKind::Identifier(_))
            && matches!(self.peek_token.kind, TokenKind::Identifier(_))
    }

    /// 変数宣言: `Int a = 1, b = 2;`
    fn parse_variable_declaration(&mut self) -> ParseResult<Statement> {
        let start = self.cur_token.span.start;

        // 1. 型名を取得 (例: "Int")
        let type_name = if let TokenKind::Identifier(ref s) = self.cur_token.kind {
            s.clone()
        } else {
            return Err(SnowFallError::new_compiler_error(
                "Expected type name".into(),
                "SF0012".to_string(),
                self.lexer.line,
                self.lexer.column,
            ));
        };

        let mut declarators = Vec::new();

        // 2. 変数リストを解析
        loop {
            // 変数名へ移動
            self.expect_peek(TokenKind::Identifier("".to_string()))?;
            let var_name = if let TokenKind::Identifier(ref s) = self.cur_token.kind {
                s.clone()
            } else {
                unreachable!()
            };

            // 初期化式があるかチェック
            let mut value = None;
            if self.peek_token_is(&TokenKind::Operator(OperatorToken::Assign)) {
                self.next_token(); // Skip '='
                self.next_token(); // Expr の開始位置に移動
                value = Some(self.parse_expression(Precedence::Lowest)?);
            }

            declarators.push(VariableDeclarator {
                name: var_name,
                value,
            });

            // カンマがあれば継続、なければ終了
            if self.peek_token_is(&TokenKind::Delimiter(DelimiterToken::Comma)) {
                self.next_token();
            } else {
                break;
            }
        }

        self.expect_peek(TokenKind::Delimiter(DelimiterToken::Semicolon))?;

        Ok(Statement {
            kind: StatementKind::VariableDeclaration {
                type_name,
                declarators,
            },
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// 関数宣言: `function Int main() {}`
    fn parse_function_declaration(&mut self) -> ParseResult<Statement> {
        let start = self.cur_token.span.start;

        // functionキーワードの次は戻り値の型 (Intなど)
        self.expect_peek(TokenKind::Identifier("".to_string()))?;
        let return_type = if let TokenKind::Identifier(ref s) = self.cur_token.kind {
            Some(s.clone())
        } else {
            return Err(SnowFallError::new_compiler_error(
                "Expected return type".into(),
                "SF0013".to_string(),
                self.lexer.line,
                self.lexer.column,
            ));
        };

        // 関数名
        self.expect_peek(TokenKind::Identifier("".to_string()))?;
        let name = if let TokenKind::Identifier(ref s) = self.cur_token.kind {
            s.clone()
        } else {
            unreachable!()
        };

        let params = self.parse_parameters()?;

        self.expect_peek(TokenKind::Delimiter(DelimiterToken::LBrace))?;
        let body = Box::new(self.parse_block_statement()?);

        Ok(Statement {
            kind: StatementKind::FunctionDeclaration {
                kind: FunctionKind::Function,
                name,
                return_type,
                params,
                body,
            },
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// Sub関数宣言: `sub main() {}`
    fn parse_sub_declaration(&mut self) -> ParseResult<Statement> {
        let start = self.cur_token.span.start;

        // subキーワードの次はすぐに関数名 (戻り値なし)
        self.expect_peek(TokenKind::Identifier("".to_string()))?;
        let name = if let TokenKind::Identifier(ref s) = self.cur_token.kind {
            s.clone()
        } else {
            unreachable!()
        };

        let params = self.parse_parameters()?;

        self.expect_peek(TokenKind::Delimiter(DelimiterToken::LBrace))?;
        let body = Box::new(self.parse_block_statement()?);

        Ok(Statement {
            kind: StatementKind::FunctionDeclaration {
                kind: FunctionKind::Sub,
                name,
                return_type: None, // Subは戻り値なし
                params,
                body,
            },
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// パラメータ解析 `(Int a, Float b = 2)`
    fn parse_parameters(&mut self) -> ParseResult<Vec<Parameter>> {
        self.expect_peek(TokenKind::Delimiter(DelimiterToken::LParen))?;

        let mut params = Vec::new();
        if self.peek_token_is(&TokenKind::Delimiter(DelimiterToken::RParen)) {
            self.next_token();
            return Ok(params);
        }

        self.next_token();

        loop {
            // 型名
            let type_name = if let TokenKind::Identifier(ref s) = self.cur_token.kind {
                s.clone()
            } else {
                return Err(SnowFallError::new_compiler_error(
                    "Expected parameter type".into(),
                    "SF0014".to_string(),
                    self.lexer.line,
                    self.lexer.column,
                ));
            };

            // パラメータ名
            self.expect_peek(TokenKind::Identifier("".to_string()))?;
            let name = if let TokenKind::Identifier(ref s) = self.cur_token.kind {
                s.clone()
            } else {
                unreachable!()
            };

            let mut value = None;
            if self.peek_token_is(&TokenKind::Operator(OperatorToken::Assign)) {
                self.next_token();
                self.next_token();
                value = Some(self.parse_expression(Precedence::Lowest)?);
            }

            params.push(Parameter {
                name,
                type_name,
                value,
            });

            if self.peek_token_is(&TokenKind::Delimiter(DelimiterToken::Comma)) {
                self.next_token();
                self.next_token();
            } else {
                break;
            }
        }

        self.expect_peek(TokenKind::Delimiter(DelimiterToken::RParen))?;
        Ok(params)
    }

    /// return 文を解析する
    fn parse_return_statement(&mut self) -> ParseResult<Statement> {
        let start = self.cur_token.span.start;
        self.next_token();

        let value = if self.cur_token.kind == TokenKind::Delimiter(DelimiterToken::Semicolon) {
            None
        } else {
            let expr = self.parse_expression(Precedence::Lowest)?;
            if self.peek_token_is(&TokenKind::Delimiter(DelimiterToken::Semicolon)) {
                self.next_token();
            }
            Some(expr)
        };

        Ok(Statement {
            kind: StatementKind::Return(value),
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// if 文を解析する
    fn parse_if_statement(&mut self) -> ParseResult<Statement> {
        let start = self.cur_token.span.start;
        self.expect_peek(TokenKind::Delimiter(DelimiterToken::LParen))?;
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(TokenKind::Delimiter(DelimiterToken::RParen))?;

        self.next_token();
        let consequence = Box::new(self.parse_statement()?);

        let mut alternative = None;
        if self.peek_token_is(&TokenKind::Keyword(KeywordToken::Else)) {
            self.next_token(); // skip 'else'
            self.next_token(); // 次のStatementに移動
            alternative = Some(Box::new(self.parse_statement()?));
        }

        Ok(Statement {
            kind: StatementKind::If {
                condition,
                consequence,
                alternative,
            },
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// for 文を解析する
    fn parse_for_statement(&mut self) -> ParseResult<Statement> {
        let start = self.cur_token.span.start;
        self.expect_peek(TokenKind::Delimiter(DelimiterToken::LParen))?;
        self.next_token();

        if self.is_for_each_loop() {
            // forEach 文
            let binding = {
                let name = if let TokenKind::Identifier(ref s) = self.cur_token.kind {
                    s.clone()
                } else {
                    return Err(SnowFallError::new_compiler_error(
                        "Expected identifier in for-each loop".to_string(),
                        "SF0016".to_string(),
                        self.lexer.line,
                        self.lexer.column,
                    ));
                };
                Binding {
                    name,
                    type_name: None,
                }
            };

            self.next_token();
            let kind = match self.cur_token.kind {
                TokenKind::Keyword(KeywordToken::In) => ForEachKind::In,
                TokenKind::Keyword(KeywordToken::Of) => ForEachKind::Of,
                _ => {
                    return Err(SnowFallError::new_compiler_error(
                        "Expected 'in' or 'of' in for-each loop".to_string(),
                        "SF0017".to_string(),
                        self.lexer.line,
                        self.lexer.column,
                    ));
                }
            };
            self.next_token();
            let iterable = self.parse_expression(Precedence::Lowest)?;

            self.expect_peek(TokenKind::Delimiter(DelimiterToken::RParen))?;
            self.next_token();
            let body = Box::new(self.parse_statement()?);

            Ok(Statement {
                kind: StatementKind::ForEach {
                    binding,
                    iterable,
                    kind,
                    body,
                },
                span: Span {
                    start,
                    end: self.cur_token.span.end,
                },
            })
        } else {
            // for 文
            // 初期化
            let init = if self.cur_token.kind != TokenKind::Delimiter(DelimiterToken::Semicolon) {
                // ここではセミコロンを消費しないバージョンの文解析が必要
                if self.is_variable_declaration() {
                    Some(Box::new(self.parse_variable_declaration_for_for()?))
                } else {
                    Some(Box::new(self.parse_expression_statement_for_for()?))
                }
            } else {
                None
            };
            self.expect_peek(TokenKind::Delimiter(DelimiterToken::Semicolon))?;
            self.next_token();

            // 条件
            let condition =
                if self.cur_token.kind != TokenKind::Delimiter(DelimiterToken::Semicolon) {
                    Some(self.parse_expression(Precedence::Lowest)?)
                } else {
                    None
                };
            self.expect_peek(TokenKind::Delimiter(DelimiterToken::Semicolon))?;
            self.next_token();

            // 更新
            let update = if self.cur_token.kind != TokenKind::Delimiter(DelimiterToken::RParen) {
                Some(Box::new(self.parse_expression_statement_for_for()?))
            } else {
                None
            };
            self.expect_peek(TokenKind::Delimiter(DelimiterToken::RParen))?;
            self.next_token();
            let body = Box::new(self.parse_statement()?);
            Ok(Statement {
                kind: StatementKind::For {
                    init,
                    condition,
                    update,
                    body,
                },
                span: Span {
                    start,
                    end: self.cur_token.span.end,
                },
            })
        }
    }

    /// 推測的に先を見て、現在の `for` 構造が正しいかどうかを判断します。
    /// for-each ループ (`in` または `of`) または C スタイルの for ループ (`;`) です。
    /// これは `parse_for_statement` のヘルパーです。 `(` の直後にあることを前提としています。
    fn is_for_each_loop(&self) -> bool {
        let mut temp_lexer = self.lexer.clone();
        let mut temp_cur = self.cur_token.clone();
        let mut temp_peek = self.peek_token.clone();
        let mut paren_level = 1;

        loop {
            match &temp_cur.kind {
                // 最上位に「in」または「of」が見つかった場合、それは for-each です。
                TokenKind::Keyword(KeywordToken::In) | TokenKind::Keyword(KeywordToken::Of)
                    if paren_level == 1 =>
                {
                    return true;
                }
                // セミコロンが見つかった場合、それは C スタイルの for ループです。
                TokenKind::Delimiter(DelimiterToken::Semicolon) => {
                    return false;
                }
                TokenKind::Delimiter(DelimiterToken::LParen) => paren_level += 1,
                TokenKind::Delimiter(DelimiterToken::RParen) => {
                    paren_level -= 1;
                    // for ヘッダー `(...)` の終わりに達しました。
                    // 有効な foreach には「in」または「of」が必要なので、ここまで来ると、それは 1 つではありません。
                    if paren_level == 0 {
                        return false;
                    }
                }
                TokenKind::Eof => return false, // 予期せず入力の終わりに達しました
                _ => {}
            }

            // Advance tokens
            temp_cur = temp_peek;
            temp_peek = temp_lexer.next_token().unwrap_or(Token::eof(0));
        }
    }

    /// for文のinit/update用にセミコロンを消費しない`parse_expression_statement`
    fn parse_expression_statement_for_for(&mut self) -> ParseResult<Statement> {
        let start = self.cur_token.span.start;
        let expr = self.parse_expression(Precedence::Lowest)?;
        Ok(Statement {
            kind: StatementKind::Expression(expr),
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// for文のinit用にセミコロンを消費しない`parse_variable_declaration`
    fn parse_variable_declaration_for_for(&mut self) -> ParseResult<Statement> {
        let start = self.cur_token.span.start;
        let type_name = if let TokenKind::Identifier(ref s) = self.cur_token.kind {
            s.clone()
        } else {
            return Err(SnowFallError::new_compiler_error(
                "Expected type name".into(),
                "SF0012".to_string(),
                self.lexer.line,
                self.lexer.column,
            ));
        };

        let mut declarators = Vec::new();
        loop {
            self.expect_peek(TokenKind::Identifier("".to_string()))?;
            let var_name = if let TokenKind::Identifier(ref s) = self.cur_token.kind {
                s.clone()
            } else {
                unreachable!()
            };

            let mut value = None;
            if self.peek_token_is(&TokenKind::Operator(OperatorToken::Assign)) {
                self.next_token();
                self.next_token();
                value = Some(self.parse_expression(Precedence::Lowest)?);
            }

            declarators.push(VariableDeclarator {
                name: var_name,
                value,
            });

            if self.peek_token_is(&TokenKind::Delimiter(DelimiterToken::Comma)) {
                self.next_token();
            } else {
                break;
            }
        }

        Ok(Statement {
            kind: StatementKind::VariableDeclaration {
                type_name,
                declarators,
            },
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// while 文を解析する
    fn parse_while_statement(&mut self) -> ParseResult<Statement> {
        let start = self.cur_token.span.start;
        self.expect_peek(TokenKind::Delimiter(DelimiterToken::LParen))?;
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(TokenKind::Delimiter(DelimiterToken::RParen))?;
        self.next_token();
        let body = Box::new(self.parse_statement()?);

        Ok(Statement {
            kind: StatementKind::While { condition, body },
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// クラス宣言を解析する
    fn parse_class_declaration(&mut self) -> ParseResult<Statement> {
        let start = self.cur_token.span.start;
        self.expect_peek(TokenKind::Identifier("".to_string()))?;
        let name = if let TokenKind::Identifier(ref s) = self.cur_token.kind {
            s.clone()
        } else {
            unreachable!()
        };

        let mut superclass = None;
        if self.peek_token_is(&TokenKind::Keyword(KeywordToken::Extends)) {
            self.next_token();
            self.expect_peek(TokenKind::Identifier("".to_string()))?;
            if let TokenKind::Identifier(ref s) = self.cur_token.kind {
                superclass = Some(s.clone());
            }
        }

        self.expect_peek(TokenKind::Delimiter(DelimiterToken::LBrace))?;
        // メンバー解析
        let mut members = Vec::new();
        while !self.peek_token_is(&TokenKind::Delimiter(DelimiterToken::RBrace))
            && !self.peek_token_is(&TokenKind::Eof)
        {
            self.next_token();
            match self.cur_token.kind {
                TokenKind::Keyword(KeywordToken::Function) => {
                    members.push(self.parse_function_declaration()?);
                }
                TokenKind::Keyword(KeywordToken::Sub) => {
                    members.push(self.parse_sub_declaration()?);
                }
                _ => {
                    return Err(SnowFallError::new_compiler_error(
                        format!(
                            "Expected 'function' or 'sub' for class member, got {:?}",
                            self.cur_token.kind
                        ),
                        "SF0011".to_string(),
                        self.lexer.line,
                        self.lexer.column,
                    ));
                }
            }
        }
        self.expect_peek(TokenKind::Delimiter(DelimiterToken::RBrace))?;

        Ok(Statement {
            kind: StatementKind::ClassDeclaration {
                name,
                superclass,
                members,
            },
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// 与えられた優先順位より高い演算子を再帰的に解析する
    fn parse_expression(&mut self, precedence: Precedence) -> ParseResult<Expression> {
        // Prefix
        let mut left = match &self.cur_token.kind {
            TokenKind::Identifier(s) => Expression {
                kind: ExpressionKind::Identifier(s.clone()),
                span: self.cur_token.span,
            },
            TokenKind::Literal(lit) => self.parse_literal(lit)?,
            TokenKind::Keyword(KeywordToken::True) => Expression {
                kind: ExpressionKind::Boolean(true),
                span: self.cur_token.span,
            },
            TokenKind::Keyword(KeywordToken::False) => Expression {
                kind: ExpressionKind::Boolean(false),
                span: self.cur_token.span,
            },
            TokenKind::Keyword(KeywordToken::Null) => Expression {
                kind: ExpressionKind::NullLiteral,
                span: self.cur_token.span,
            },
            TokenKind::Operator(
                OperatorToken::Plus
                | OperatorToken::Minus
                | OperatorToken::Bang
                | OperatorToken::BitwiseNot,
            ) => self.parse_prefix()?,
            TokenKind::Delimiter(DelimiterToken::LParen) => self.parse_grouped()?,
            TokenKind::Delimiter(DelimiterToken::LBracket) => self.parse_array()?,
            TokenKind::Delimiter(DelimiterToken::LBrace) => self.parse_object()?, // またはblock
            _ => {
                return Err(SnowFallError::new_compiler_error(
                    format!("Unexpected token for expression: {:?}", self.cur_token),
                    "SF0015".to_string(),
                    self.lexer.line,
                    self.lexer.column,
                ));
            }
        };

        // Infix
        while !self.peek_token_is(&TokenKind::Delimiter(DelimiterToken::Semicolon))
            && precedence < self.peek_precedence()
        {
            match self.peek_token.kind {
                TokenKind::Operator(_) => {
                    self.next_token();
                    left = self.parse_infix(left)?;
                }
                TokenKind::Delimiter(DelimiterToken::LParen) => {
                    self.next_token();
                    left = self.parse_call(left)?;
                }
                TokenKind::Delimiter(DelimiterToken::LBracket) => {
                    self.next_token();
                    left = self.parse_index(left)?;
                }
                TokenKind::Delimiter(DelimiterToken::Dot) => {
                    self.next_token();
                    left = self.parse_member(left)?;
                }
                _ => return Ok(left),
            }
        }

        Ok(left)
    }

    /// リテラル値を `Expression` に変換する
    fn parse_literal(&self, lit: &LiteralToken) -> ParseResult<Expression> {
        let kind = match lit {
            LiteralToken::Int(v) => ExpressionKind::IntLiteral(*v),
            LiteralToken::Float(v) => ExpressionKind::FloatLiteral(*v),
            LiteralToken::String(v) => ExpressionKind::StringLiteral(v.clone()),
            LiteralToken::Boolean(v) => ExpressionKind::Boolean(*v),
        };
        Ok(Expression {
            kind,
            span: self.cur_token.span,
        })
    }

    /// 前置演算子（`-x`, `!x`）を解析する
    fn parse_prefix(&mut self) -> ParseResult<Expression> {
        let start = self.cur_token.span.start;
        let operator = match self.cur_token.kind {
            TokenKind::Operator(OperatorToken::Plus) => PrefixOperator::Plus,
            TokenKind::Operator(OperatorToken::Minus) => PrefixOperator::Minus,
            TokenKind::Operator(OperatorToken::Bang) => PrefixOperator::Bang,
            TokenKind::Operator(OperatorToken::BitwiseNot) => PrefixOperator::BitwiseNot,
            _ => unreachable!(),
        };
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;
        Ok(Expression {
            kind: ExpressionKind::Prefix {
                operator,
                right: Box::new(right),
            },
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// 中置演算子トークンを AST 用の `InfixOperator` に変換する
    fn convert_infix_operator(&self, op: InfixOpToken) -> InfixOperator {
        match op {
            InfixOpToken::Operator(op) => match op {
                OperatorToken::Plus => InfixOperator::Add,
                OperatorToken::Minus => InfixOperator::Subtract,
                OperatorToken::Asterisk => InfixOperator::Multiply,
                OperatorToken::Slash => InfixOperator::Divide,
                OperatorToken::Percent => InfixOperator::Modulo,
                OperatorToken::Power => InfixOperator::Power,
                OperatorToken::Equal => InfixOperator::Equals,
                OperatorToken::NotEqual => InfixOperator::NotEquals,
                OperatorToken::StrictEqual => InfixOperator::StrictEquals,
                OperatorToken::StrictNotEqual => InfixOperator::StrictNotEquals,
                OperatorToken::LessThan => InfixOperator::LessThan,
                OperatorToken::GreaterThan => InfixOperator::GreaterThan,
                OperatorToken::LessThanOrEqual => InfixOperator::LessThanOrEqual,
                OperatorToken::GreaterThanOrEqual => InfixOperator::GreaterThanOrEqual,
                OperatorToken::LogicalAnd => InfixOperator::LogicalAndAlso,
                OperatorToken::LogicalOr => InfixOperator::LogicalOrElse,
                OperatorToken::BitwiseAnd => InfixOperator::BitwiseAnd,
                OperatorToken::BitwiseOr => InfixOperator::BitwiseOr,
                OperatorToken::BitwiseXor => InfixOperator::BitwiseXor,
                OperatorToken::BitwiseLeftShift => InfixOperator::BitwiseLeftShift,
                OperatorToken::BitwiseRightShift => InfixOperator::BitwiseRightShift,
                OperatorToken::BitwiseUnsignedLeftShift => InfixOperator::BitwiseUnsignedLeftShift,
                OperatorToken::BitwiseUnsignedRightShift => {
                    InfixOperator::BitwiseUnsignedRightShift
                }
                _ => unreachable!(), // fallback or error
            },

            InfixOpToken::Keyword(keyword) => match keyword {
                KeywordToken::And => InfixOperator::LogicalAnd,
                KeywordToken::Or => InfixOperator::LogicalOr,
                _ => unreachable!(), // fallback or error
            },
        }
    }

    /// 中置演算子（`a + b`, `a = b` など）を解析する
    fn parse_infix(&mut self, left: Expression) -> ParseResult<Expression> {
        let start = left.span.start;
        let op_token = match &self.cur_token.kind {
            TokenKind::Operator(op) => InfixOpToken::Operator(op.clone()),
            TokenKind::Keyword(KeywordToken::And | KeywordToken::Or) => {
                InfixOpToken::Keyword(if let TokenKind::Keyword(k) = &self.cur_token.kind {
                    k.clone()
                } else {
                    unreachable!()
                })
            }
            _ => unreachable!(),
        };

        let precedence = self.cur_precedence();

        // 代入演算子の場合の特別処理（右結合）
        match op_token {
            InfixOpToken::Operator(OperatorToken::Assign) => {
                self.next_token();
                let right = self.parse_expression(Precedence::Lowest)?;
                return Ok(Expression {
                    kind: ExpressionKind::Assignment {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    span: Span {
                        start,
                        end: self.cur_token.span.end,
                    },
                });
            }
            _ => {}
        }

        let operator = self.convert_infix_operator(op_token);
        self.next_token();
        let right = self.parse_expression(precedence)?;

        Ok(Expression {
            kind: ExpressionKind::Infix {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            },
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// 括弧で囲まれた式を解析する
    fn parse_grouped(&mut self) -> ParseResult<Expression> {
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(TokenKind::Delimiter(DelimiterToken::RParen))?;
        Ok(expr)
    }

    /// 関数呼び出し式を解析する
    fn parse_call(&mut self, function: Expression) -> ParseResult<Expression> {
        let start = function.span.start;
        let arguments = self.parse_expression_list(DelimiterToken::RParen)?;
        Ok(Expression {
            kind: ExpressionKind::Call {
                function: Box::new(function),
                arguments,
            },
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// 添字アクセス式を解析する (`array[index]`)
    fn parse_index(&mut self, left: Expression) -> ParseResult<Expression> {
        let start = left.span.start;
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(TokenKind::Delimiter(DelimiterToken::RBracket))?;

        Ok(Expression {
            kind: ExpressionKind::Index {
                left: Box::new(left),
                index: Box::new(index),
            },
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// メンバーアクセス式を解析する (`object.property`)
    fn parse_member(&mut self, left: Expression) -> ParseResult<Expression> {
        let start = left.span.start;
        // 識別子を期待する
        self.expect_peek(TokenKind::Identifier("".to_string()))?;
        let prop = if let TokenKind::Identifier(ref s) = self.cur_token.kind {
            s.clone()
        } else {
            unreachable!()
        };

        Ok(Expression {
            kind: ExpressionKind::Member {
                left: Box::new(left),
                property: prop,
            },
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// カンマ区切りの式リストを解析する
    fn parse_expression_list(&mut self, end: DelimiterToken) -> ParseResult<Vec<Expression>> {
        let mut list = Vec::new();
        if self.peek_token_is(&TokenKind::Delimiter(end.clone())) {
            self.next_token();
            return Ok(list);
        }

        self.next_token();
        list.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(&TokenKind::Delimiter(DelimiterToken::Comma)) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::Lowest)?);
        }
        self.expect_peek(TokenKind::Delimiter(end))?;
        Ok(list)
    }

    /// 配列リテラルを解析する (`[a, b, c]`)
    fn parse_array(&mut self) -> ParseResult<Expression> {
        let start = self.cur_token.span.start;
        let elements = self.parse_expression_list(DelimiterToken::RBracket)?;
        Ok(Expression {
            kind: ExpressionKind::ArrayLiteral(elements),
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }

    /// オブジェクトリテラルを解析する (`{ key: value, ... }`)
    fn parse_object(&mut self) -> ParseResult<Expression> {
        let start = self.cur_token.span.start;
        // { key: value, ... }
        let mut pairs = Vec::new();

        while !self.peek_token_is(&TokenKind::Delimiter(DelimiterToken::RBrace)) {
            self.next_token();

            let key = self.parse_expression(Precedence::Lowest)?;

            self.expect_peek(TokenKind::Delimiter(DelimiterToken::Colon))?;
            self.next_token();
            let value = self.parse_expression(Precedence::Lowest)?;

            pairs.push((key, value));

            if !self.peek_token_is(&TokenKind::Delimiter(DelimiterToken::RBrace)) {
                self.expect_peek(TokenKind::Delimiter(DelimiterToken::Comma))?;
            }
        }
        self.expect_peek(TokenKind::Delimiter(DelimiterToken::RBrace))?;

        Ok(Expression {
            kind: ExpressionKind::ObjectLiteral { pairs },
            span: Span {
                start,
                end: self.cur_token.span.end,
            },
        })
    }
}
