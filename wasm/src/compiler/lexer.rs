use crate::common::{
    DelimiterToken, KeywordToken, LiteralToken, OperatorToken, Span, Token, TokenKind,
};

pub struct Lexer<'a> {
    input: &'a str,
    /// 入力内の現在位置 (現在の文字を指します)
    position: usize,
    /// 入力内の現在の読み取り位置 (現在の文字の後)
    read_position: usize,
    /// 現在調査中の文字
    ch: u8,
    /// 現在の行番号
    pub line: u32,
    /// 現在の列番号
    pub column: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
            line: 1,
            column: 1,
        };
        l.read_char();
        l
    }

    /// 次の文字を読み込み、`ch`フィールドを更新します
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0; // ASCII code for "NUL"
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;

        if self.ch == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }

    /// 次の文字を調査しますが、`ch`フィールドは変更しません
    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    /// 変数名やキーワードを読み取ります
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    /// 次のトークンを取得します
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let start_pos = self.position;
        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    if self.peek_char() == b'=' {
                        self.read_char();
                        // (===)
                        Token {
                            kind: TokenKind::Operator(OperatorToken::StrictEqual),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        }
                    } else {
                        // (==)
                        Token {
                            kind: TokenKind::Operator(OperatorToken::Equal),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        }
                    }
                } else {
                    // (=)
                    Token {
                        kind: TokenKind::Operator(OperatorToken::Assign),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                }
            }
            b'+' => Token {
                kind: TokenKind::Operator(OperatorToken::Plus),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b'-' => Token {
                kind: TokenKind::Operator(OperatorToken::Minus),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b'*' => {
                if self.peek_char() == b'*' {
                    self.read_char();
                    // (**)
                    Token {
                        kind: TokenKind::Operator(OperatorToken::Power),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                } else {
                    // (*)
                    Token {
                        kind: TokenKind::Operator(OperatorToken::Asterisk),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                }
            }
            b'/' => Token {
                kind: TokenKind::Operator(OperatorToken::Slash),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b'%' => Token {
                kind: TokenKind::Operator(OperatorToken::Percent),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    if self.peek_char() == b'=' {
                        self.read_char();
                        // (!==)
                        Token {
                            kind: TokenKind::Operator(OperatorToken::StrictNotEqual),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        }
                    } else {
                        // (!=)
                        Token {
                            kind: TokenKind::Operator(OperatorToken::NotEqual),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        }
                    }
                } else {
                    // (!)
                    Token {
                        kind: TokenKind::Operator(OperatorToken::Bang),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                }
            }
            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    // (<=)
                    Token {
                        kind: TokenKind::Operator(OperatorToken::LessThanOrEqual),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                } else if self.peek_char() == b'<' {
                    self.read_char();
                    if self.peek_char() == b'<' {
                        self.read_char();
                        // (<<<)
                        Token {
                            kind: TokenKind::Operator(OperatorToken::BitwiseUnsignedLeftShift),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        }
                    } else {
                        // (<<)
                        Token {
                            kind: TokenKind::Operator(OperatorToken::BitwiseLeftShift),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        }
                    }
                } else {
                    // (<)
                    Token {
                        kind: TokenKind::Operator(OperatorToken::LessThan),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                }
            }
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    // (>=)
                    Token {
                        kind: TokenKind::Operator(OperatorToken::GreaterThanOrEqual),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                } else if self.peek_char() == b'>' {
                    self.read_char();
                    if self.peek_char() == b'>' {
                        self.read_char();
                        // (>>>)
                        Token {
                            kind: TokenKind::Operator(OperatorToken::BitwiseUnsignedRightShift),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        }
                    } else {
                        // (>>)
                        Token {
                            kind: TokenKind::Operator(OperatorToken::BitwiseRightShift),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        }
                    }
                } else {
                    // (>)
                    Token {
                        kind: TokenKind::Operator(OperatorToken::GreaterThan),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                }
            }
            b'.' => Token {
                kind: TokenKind::Delimiter(DelimiterToken::Dot),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b',' => Token {
                kind: TokenKind::Delimiter(DelimiterToken::Comma),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b':' => Token {
                kind: TokenKind::Delimiter(DelimiterToken::Colon),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b';' => Token {
                kind: TokenKind::Delimiter(DelimiterToken::Semicolon),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b'(' => Token {
                kind: TokenKind::Delimiter(DelimiterToken::LParen),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b')' => Token {
                kind: TokenKind::Delimiter(DelimiterToken::RParen),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b'{' => Token {
                kind: TokenKind::Delimiter(DelimiterToken::LBrace),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b'}' => Token {
                kind: TokenKind::Delimiter(DelimiterToken::RBrace),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b'[' => Token {
                kind: TokenKind::Delimiter(DelimiterToken::LBracket),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b']' => Token {
                kind: TokenKind::Delimiter(DelimiterToken::RBracket),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b'&' => {
                if self.peek_char() == b'&' {
                    self.read_char();
                    // (&&)
                    Token {
                        kind: TokenKind::Operator(OperatorToken::LogicalAnd),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                } else {
                    // (&)
                    Token {
                        kind: TokenKind::Operator(OperatorToken::BitwiseAnd),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                }
            }
            b'|' => {
                if self.peek_char() == b'|' {
                    self.read_char();
                    // (||)
                    Token {
                        kind: TokenKind::Operator(OperatorToken::LogicalOr),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                } else {
                    // (|)
                    Token {
                        kind: TokenKind::Operator(OperatorToken::BitwiseOr),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                }
            }
            b'^' => Token {
                kind: TokenKind::Operator(OperatorToken::BitwiseXor),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b'~' => Token {
                kind: TokenKind::Operator(OperatorToken::BitwiseNot),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            b'"' => self.read_string(),
            b'\'' => self.read_string(),
            b'0'..=b'9' => return self.read_number(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();
                return match ident.as_str() {
                    "function" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Function),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "sub" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Sub),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "class" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Class),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "extends" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Extends),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "constructor" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Constructor),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "new" => Token {
                        kind: TokenKind::Keyword(KeywordToken::New),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "if" => Token {
                        kind: TokenKind::Keyword(KeywordToken::If),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "else" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Else),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "for" => Token {
                        kind: TokenKind::Keyword(KeywordToken::For),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "while" => Token {
                        kind: TokenKind::Keyword(KeywordToken::While),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "in" => Token {
                        kind: TokenKind::Keyword(KeywordToken::In),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "of" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Of),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "switch" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Switch),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "case" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Case),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "default" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Default),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "break" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Break),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "continue" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Continue),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "return" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Return),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "true" => Token {
                        kind: TokenKind::Keyword(KeywordToken::True),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "false" => Token {
                        kind: TokenKind::Keyword(KeywordToken::False),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "null" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Null),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "and" => Token {
                        kind: TokenKind::Keyword(KeywordToken::And),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    "or" => Token {
                        kind: TokenKind::Keyword(KeywordToken::Or),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                    _ => Token {
                        kind: TokenKind::Identifier(ident),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    },
                };
            }
            0 => Token {
                kind: TokenKind::Eof,
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            _ => Token {
                kind: TokenKind::Illegal(self.ch.to_string()),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
        };

        self.read_char();
        tok
    }

    /// 空白文字とコメントをスキップします
    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                // 標準の空白文字
                b' ' | b'\r' | b'\t' | b'\n' => self.read_char(),
                // コメント候補の開始
                b'/' => {
                    if self.peek_char() == b'/' {
                        // コメント (// ...)
                        // 行末まで読み込む
                        while self.ch != b'\n' && self.ch != 0 {
                            self.read_char();
                        }
                    } else if self.peek_char() == b'*' {
                        // ブロックコメント (/* ... */)
                        self.read_char(); // '*'を読み込む
                        self.read_char(); // コメント内に入る

                        while !(self.ch == b'*' && self.peek_char() == b'/') && self.ch != 0 {
                            self.read_char();
                        }

                        // '*/'を読み込む
                        if self.ch != 0 {
                            self.read_char(); // '*'を読み込む
                            self.read_char(); // '/'を読み込む
                        }
                    } else {
                        // コメントではないため、呼び出し元が処理できるようにループを中断します。
                        return;
                    }
                }
                // 空白やコメントではないので終了します
                _ => return,
            }
        }
    }

    /// 数字リテラルを読み取ります (整数および浮動小数点数)
    fn read_number(&mut self) -> Token {
        // 基数の接頭辞を確認する
        if self.ch == b'0' {
            match self.peek_char() {
                b'x' | b'X' => return self.read_hex_number(),
                b'b' | b'B' => return self.read_binary_number(),
                _ => {}
            }
        }
        let start_pos = self.position;

        let mut dot_count: i32 = 0;
        let mut number_str = String::new();

        let mut prev_was_digit = false;
        let mut prev_was_underscore = false;

        while self.ch.is_ascii_digit() || self.ch == b'_' || self.ch == b'.' {
            match self.ch {
                b'_' => {
                    // 先頭 or '.' 直後は NG
                    if number_str.is_empty() || !prev_was_digit {
                        return Token {
                            kind: TokenKind::Illegal(number_str),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        };
                    }
                    prev_was_underscore = true;
                }
                b'.' => {
                    // '_' 直後は NG
                    if prev_was_underscore {
                        return Token {
                            kind: TokenKind::Illegal(number_str),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        };
                    }
                    dot_count += 1;
                    if dot_count > 1 {
                        break;
                    }
                    number_str.push('.');
                    prev_was_digit = false;
                    prev_was_underscore = false;
                }
                _ => {
                    // 数字
                    number_str.push(self.ch as char);
                    prev_was_digit = true;
                    prev_was_underscore = false;
                }
            }
            self.read_char();
        }

        // 末尾 '_' は NG
        if prev_was_underscore {
            return Token {
                kind: TokenKind::Illegal(number_str),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            };
        }

        if dot_count >= 1 {
            let (int_str, frac_str) = match number_str.split_once('.') {
                Some(v) => v,
                None => {
                    return Token {
                        kind: TokenKind::Illegal(number_str),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                }
            };
            if int_str.is_empty() && frac_str.is_empty() {
                return Token {
                    kind: TokenKind::Illegal(number_str),
                    span: Span {
                        start: start_pos,
                        end: self.position + 1,
                    },
                };
            }

            if int_str.is_empty() {
                number_str = format!("0.{}", frac_str);
            } else if frac_str.is_empty() {
                number_str = format!("{}.0", int_str);
            }

            match number_str.parse::<f64>() {
                Ok(f) => Token {
                    kind: TokenKind::Literal(LiteralToken::Float(f)),
                    span: Span {
                        start: start_pos,
                        end: self.position + 1,
                    },
                },
                Err(_) => Token {
                    kind: TokenKind::Illegal(number_str),
                    span: Span {
                        start: start_pos,
                        end: self.position + 1,
                    },
                },
            }
        } else {
            match number_str.parse::<i64>() {
                Ok(i) => Token {
                    kind: TokenKind::Literal(LiteralToken::Int(i)),
                    span: Span {
                        start: start_pos,
                        end: self.position + 1,
                    },
                },
                Err(_) => Token {
                    kind: TokenKind::Illegal(number_str),
                    span: Span {
                        start: start_pos,
                        end: self.position + 1,
                    },
                },
            }
        }
    }

    /// 16進数リテラルを読み取ります
    fn read_hex_number(&mut self) -> Token {
        self.read_char(); // skip '0'
        self.read_char(); // skip 'x'

        let start_pos = self.position;

        let mut dot_count: i32 = 0;
        let mut number_str = String::new();

        let mut prev_was_digit = false;
        let mut prev_was_underscore = false;

        while self.ch.is_ascii_hexdigit() || self.ch == b'_' || self.ch == b'.' {
            match self.ch {
                b'_' => {
                    // 先頭 or '.' 直後は NG
                    if number_str.is_empty() || !prev_was_digit {
                        return Token {
                            kind: TokenKind::Illegal(number_str),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        };
                    }
                    prev_was_underscore = true;
                }
                b'.' => {
                    // '_' 直後は NG
                    if prev_was_underscore {
                        return Token {
                            kind: TokenKind::Illegal(number_str),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        };
                    }
                    dot_count += 1;
                    if dot_count > 1 {
                        break;
                    }
                    number_str.push('.');
                    prev_was_digit = false;
                    prev_was_underscore = false;
                }
                _ => {
                    // 数字
                    number_str.push(self.ch as char);
                    prev_was_digit = true;
                    prev_was_underscore = false;
                }
            }
            self.read_char();
        }

        // 末尾 '_' は NG
        if prev_was_underscore {
            return Token {
                kind: TokenKind::Illegal(number_str),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            };
        }

        // 0xf.f のような16進浮動小数点数を処理します
        if dot_count >= 1 {
            // 基本的な16進浮動小数点解析 (例: "A.B" -> 10.6875)
            let (int_str, frac_str) = match number_str.split_once('.') {
                Some(v) => v,
                None => {
                    return Token {
                        kind: TokenKind::Illegal(number_str),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                }
            };

            if int_str.is_empty() {
                return Token {
                    kind: TokenKind::Illegal(number_str),
                    span: Span {
                        start: start_pos,
                        end: self.position + 1,
                    },
                };
            }

            let integer_part = match i64::from_str_radix(int_str, 16) {
                Ok(v) => v as f64,
                Err(_) => {
                    return Token {
                        kind: TokenKind::Illegal(number_str),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                }
            };

            let mut fractional_part: f64 = 0.0;
            let mut base: f64 = 16.0;

            for c in frac_str.chars() {
                let digit = match c.to_digit(16) {
                    Some(d) => d as f64,
                    None => {
                        return Token {
                            kind: TokenKind::Illegal(number_str),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        }
                    }
                };
                fractional_part += digit / base;
                base *= 16.0;
            }

            return Token {
                kind: TokenKind::Literal(LiteralToken::Float(integer_part + fractional_part)),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            };
        }

        match i64::from_str_radix(&number_str, 16) {
            Ok(i) => Token {
                kind: TokenKind::Literal(LiteralToken::Int(i)),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            Err(_) => Token {
                kind: TokenKind::Illegal(number_str),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
        }
    }

    /// 2進数リテラルを読み取ります
    fn read_binary_number(&mut self) -> Token {
        self.read_char(); // skip '0'
        self.read_char(); // skip 'b'

        let start_pos = self.position;

        let mut dot_count: i32 = 0;
        let mut number_str = String::new();

        let mut prev_was_digit = false;
        let mut prev_was_underscore = false;

        while self.ch == b'0' || self.ch == b'1' || self.ch == b'_' || self.ch == b'.' {
            match self.ch {
                b'_' => {
                    // 先頭 or '.' 直後は NG
                    if number_str.is_empty() || !prev_was_digit {
                        return Token {
                            kind: TokenKind::Illegal(number_str),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        };
                    }
                    prev_was_underscore = true;
                }
                b'.' => {
                    // '_' 直後は NG
                    if prev_was_underscore {
                        return Token {
                            kind: TokenKind::Illegal(number_str),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        };
                    }
                    dot_count += 1;
                    if dot_count > 1 {
                        break;
                    }
                    number_str.push('.');
                    prev_was_digit = false;
                    prev_was_underscore = false;
                }
                _ => {
                    // 数字
                    number_str.push(self.ch as char);
                    prev_was_digit = true;
                    prev_was_underscore = false;
                }
            }
            self.read_char();
        }

        // 末尾 '_' は NG
        if prev_was_underscore {
            return Token {
                kind: TokenKind::Illegal(number_str),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            };
        }

        // 0b1.1 のような2進浮動小数点数を処理します
        if dot_count >= 1 {
            // 基本的な2進浮動小数点解析 (例: "1.1" -> 1.5)
            let (int_str, frac_str) = match number_str.split_once('.') {
                Some(v) => v,
                None => {
                    return Token {
                        kind: TokenKind::Illegal(number_str),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                }
            };

            if int_str.is_empty() {
                return Token {
                    kind: TokenKind::Illegal(number_str),
                    span: Span {
                        start: start_pos,
                        end: self.position + 1,
                    },
                };
            }

            let integer_part = match i64::from_str_radix(int_str, 2) {
                Ok(v) => v as f64,
                Err(_) => {
                    return Token {
                        kind: TokenKind::Illegal(number_str),
                        span: Span {
                            start: start_pos,
                            end: self.position + 1,
                        },
                    }
                }
            };

            let mut fractional_part: f64 = 0.0;
            let mut base: f64 = 2.0;

            for c in frac_str.chars() {
                let digit = match c {
                    '0' => 0.0,
                    '1' => 1.0,
                    _ => {
                        return Token {
                            kind: TokenKind::Illegal(number_str),
                            span: Span {
                                start: start_pos,
                                end: self.position + 1,
                            },
                        }
                    }
                };
                fractional_part += digit / base;
                base *= 2.0;
            }

            return Token {
                kind: TokenKind::Literal(LiteralToken::Float(integer_part + fractional_part)),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            };
        }

        match i64::from_str_radix(&number_str, 2) {
            Ok(i) => Token {
                kind: TokenKind::Literal(LiteralToken::Int(i)),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
            Err(_) => Token {
                kind: TokenKind::Illegal(number_str),
                span: Span {
                    start: start_pos,
                    end: self.position + 1,
                },
            },
        }
    }

    fn read_string(&mut self) -> Token {
        let start_pos = self.position;

        let quote_char: u8 = self.ch;
        self.read_char(); // skip opening '"' or '\''
        let position = self.position;
        let mut old_ch: u8 = 0;
        while old_ch == b'\\' || self.ch != quote_char && self.ch != 0 {
            old_ch = self.ch;
            self.read_char();
        }
        let s = self.input[position..self.position].to_string();

        Token {
            kind: TokenKind::Literal(LiteralToken::String(s)),
            span: Span {
                start: start_pos,
                end: self.position + 1,
            },
        }
    }
}
