use crate::{
    common::{
        DelimiterToken, KeywordToken, LiteralToken, OperatorToken, Token, TokenKind,
        error::SnowFallError,
    },
    create_token,
};

#[derive(Clone)]
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
    pub fn next_token(&mut self) -> Result<Token, SnowFallError> {
        self.skip_whitespace();

        let start_pos = self.position;
        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    if self.peek_char() == b'=' {
                        self.read_char();
                        // (===)
                        Ok(create_token!(
                            TokenKind::Operator(OperatorToken::StrictEqual),
                            start_pos,
                            self.position + 1
                        ))
                    } else {
                        // (==)
                        Ok(create_token!(
                            TokenKind::Operator(OperatorToken::Equal),
                            start_pos,
                            self.position + 1
                        ))
                    }
                } else {
                    // (=)
                    Ok(create_token!(
                        TokenKind::Operator(OperatorToken::Assign),
                        start_pos,
                        self.position + 1
                    ))
                }
            }
            b'+' => Ok(create_token!(
                TokenKind::Operator(OperatorToken::Plus),
                start_pos,
                self.position + 1
            )),
            b'-' => Ok(create_token!(
                TokenKind::Operator(OperatorToken::Minus),
                start_pos,
                self.position + 1
            )),
            b'*' => {
                if self.peek_char() == b'*' {
                    self.read_char();
                    // (**)
                    Ok(create_token!(
                        TokenKind::Operator(OperatorToken::Power),
                        start_pos,
                        self.position + 1
                    ))
                } else {
                    // (*)
                    Ok(create_token!(
                        TokenKind::Operator(OperatorToken::Asterisk),
                        start_pos,
                        self.position + 1
                    ))
                }
            }
            b'/' => Ok(create_token!(
                TokenKind::Operator(OperatorToken::Slash),
                start_pos,
                self.position + 1
            )),
            b'%' => Ok(create_token!(
                TokenKind::Operator(OperatorToken::Percent),
                start_pos,
                self.position + 1
            )),
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    if self.peek_char() == b'=' {
                        self.read_char();
                        // (!==)
                        Ok(create_token!(
                            TokenKind::Operator(OperatorToken::StrictNotEqual),
                            start_pos,
                            self.position + 1
                        ))
                    } else {
                        // (!=)
                        Ok(create_token!(
                            TokenKind::Operator(OperatorToken::NotEqual),
                            start_pos,
                            self.position + 1
                        ))
                    }
                } else {
                    // (!)
                    Ok(create_token!(
                        TokenKind::Operator(OperatorToken::Bang),
                        start_pos,
                        self.position + 1
                    ))
                }
            }
            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    // (<=)
                    Ok(create_token!(
                        TokenKind::Operator(OperatorToken::LessThanOrEqual),
                        start_pos,
                        self.position + 1
                    ))
                } else if self.peek_char() == b'<' {
                    self.read_char();
                    if self.peek_char() == b'<' {
                        self.read_char();
                        // (<<<)
                        Ok(create_token!(
                            TokenKind::Operator(OperatorToken::BitwiseUnsignedLeftShift),
                            start_pos,
                            self.position + 1
                        ))
                    } else {
                        // (<<)
                        Ok(create_token!(
                            TokenKind::Operator(OperatorToken::BitwiseLeftShift),
                            start_pos,
                            self.position + 1
                        ))
                    }
                } else {
                    // (<)
                    Ok(create_token!(
                        TokenKind::Operator(OperatorToken::LessThan),
                        start_pos,
                        self.position + 1
                    ))
                }
            }
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    // (>=)
                    Ok(create_token!(
                        TokenKind::Operator(OperatorToken::GreaterThanOrEqual),
                        start_pos,
                        self.position + 1
                    ))
                } else if self.peek_char() == b'>' {
                    self.read_char();
                    if self.peek_char() == b'>' {
                        self.read_char();
                        // (>>>)
                        Ok(create_token!(
                            TokenKind::Operator(OperatorToken::BitwiseUnsignedRightShift),
                            start_pos,
                            self.position + 1
                        ))
                    } else {
                        // (>>)
                        Ok(create_token!(
                            TokenKind::Operator(OperatorToken::BitwiseRightShift),
                            start_pos,
                            self.position + 1
                        ))
                    }
                } else {
                    // (>)
                    Ok(create_token!(
                        TokenKind::Operator(OperatorToken::GreaterThan),
                        start_pos,
                        self.position + 1
                    ))
                }
            }
            b'.' => Ok(create_token!(
                TokenKind::Delimiter(DelimiterToken::Dot),
                start_pos,
                self.position + 1
            )),
            b',' => Ok(create_token!(
                TokenKind::Delimiter(DelimiterToken::Comma),
                start_pos,
                self.position + 1
            )),
            b':' => Ok(create_token!(
                TokenKind::Delimiter(DelimiterToken::Colon),
                start_pos,
                self.position + 1
            )),
            b';' => Ok(create_token!(
                TokenKind::Delimiter(DelimiterToken::Semicolon),
                start_pos,
                self.position + 1
            )),
            b'(' => Ok(create_token!(
                TokenKind::Delimiter(DelimiterToken::LParen),
                start_pos,
                self.position + 1
            )),
            b')' => Ok(create_token!(
                TokenKind::Delimiter(DelimiterToken::RParen),
                start_pos,
                self.position + 1
            )),
            b'{' => Ok(create_token!(
                TokenKind::Delimiter(DelimiterToken::LBrace),
                start_pos,
                self.position + 1
            )),
            b'}' => Ok(create_token!(
                TokenKind::Delimiter(DelimiterToken::RBrace),
                start_pos,
                self.position + 1
            )),
            b'[' => Ok(create_token!(
                TokenKind::Delimiter(DelimiterToken::LBracket),
                start_pos,
                self.position + 1
            )),
            b']' => Ok(create_token!(
                TokenKind::Delimiter(DelimiterToken::RBracket),
                start_pos,
                self.position + 1
            )),
            b'&' => {
                if self.peek_char() == b'&' {
                    self.read_char();
                    // (&&)
                    Ok(create_token!(
                        TokenKind::Operator(OperatorToken::LogicalAnd),
                        start_pos,
                        self.position + 1
                    ))
                } else {
                    // (&)
                    Ok(create_token!(
                        TokenKind::Operator(OperatorToken::BitwiseAnd),
                        start_pos,
                        self.position + 1
                    ))
                }
            }
            b'|' => {
                if self.peek_char() == b'|' {
                    self.read_char();
                    // (||)
                    Ok(create_token!(
                        TokenKind::Operator(OperatorToken::LogicalOr),
                        start_pos,
                        self.position + 1
                    ))
                } else {
                    // (|)
                    Ok(create_token!(
                        TokenKind::Operator(OperatorToken::BitwiseOr),
                        start_pos,
                        self.position + 1
                    ))
                }
            }
            b'^' => Ok(create_token!(
                TokenKind::Operator(OperatorToken::BitwiseXor),
                start_pos,
                self.position + 1
            )),
            b'~' => Ok(create_token!(
                TokenKind::Operator(OperatorToken::BitwiseNot),
                start_pos,
                self.position + 1
            )),
            b'"' => self.read_string(),
            b'\'' => self.read_string(),
            b'0'..=b'9' => return self.read_number(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();
                let kind = match ident.as_str() {
                    "function" => TokenKind::Keyword(KeywordToken::Function),
                    "sub" => TokenKind::Keyword(KeywordToken::Sub),
                    "class" => TokenKind::Keyword(KeywordToken::Class),
                    "extends" => TokenKind::Keyword(KeywordToken::Extends),
                    "constructor" => TokenKind::Keyword(KeywordToken::Constructor),
                    "new" => TokenKind::Keyword(KeywordToken::New),
                    "if" => TokenKind::Keyword(KeywordToken::If),
                    "else" => TokenKind::Keyword(KeywordToken::Else),
                    "for" => TokenKind::Keyword(KeywordToken::For),
                    "while" => TokenKind::Keyword(KeywordToken::While),
                    "in" => TokenKind::Keyword(KeywordToken::In),
                    "of" => TokenKind::Keyword(KeywordToken::Of),
                    "switch" => TokenKind::Keyword(KeywordToken::Switch),
                    "case" => TokenKind::Keyword(KeywordToken::Case),
                    "default" => TokenKind::Keyword(KeywordToken::Default),
                    "break" => TokenKind::Keyword(KeywordToken::Break),
                    "continue" => TokenKind::Keyword(KeywordToken::Continue),
                    "return" => TokenKind::Keyword(KeywordToken::Return),
                    "true" => TokenKind::Keyword(KeywordToken::True),
                    "false" => TokenKind::Keyword(KeywordToken::False),
                    "null" => TokenKind::Keyword(KeywordToken::Null),
                    "and" => TokenKind::Keyword(KeywordToken::And),
                    "or" => TokenKind::Keyword(KeywordToken::Or),
                    _ => TokenKind::Identifier(ident),
                };
                return Ok(create_token!(kind, start_pos, self.position + 1));
            }
            0 => Ok(Token::eof(start_pos)),
            _ => {
                let error_tok = self.ch;
                self.read_char();
                return Err(SnowFallError::new_compiler_error(
                    format!("Unexpected character: {}", error_tok as char),
                    "SF0001".to_string(),
                    self.line,
                    self.column,
                ));
            }
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
    fn read_number(&mut self) -> Result<Token, SnowFallError> {
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
                        return Err(SnowFallError::new_compiler_error(
                            "Invalid number format: misplaced underscore".to_string(),
                            "SF0002".to_string(),
                            self.line,
                            self.column,
                        ));
                    }
                    prev_was_underscore = true;
                }
                b'.' => {
                    // '_' 直後は NG
                    if prev_was_underscore {
                        return Err(SnowFallError::new_compiler_error(
                            "Invalid number format: misplaced underscore".to_string(),
                            "SF0002".to_string(),
                            self.line,
                            self.column,
                        ));
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
            return Err(SnowFallError::new_compiler_error(
                "Invalid number format: misplaced underscore".to_string(),
                "SF0002".to_string(),
                self.line,
                self.column,
            ));
        }

        if dot_count >= 1 {
            let (int_str, frac_str) = match number_str.split_once('.') {
                Some(v) => v,
                None => {
                    return Err(SnowFallError::new_compiler_error(
                        "Invalid float format".to_string(),
                        "SF0002".to_string(),
                        self.line,
                        self.column,
                    ));
                }
            };
            if int_str.is_empty() && frac_str.is_empty() {
                return Err(SnowFallError::new_compiler_error(
                    "Invalid float format".to_string(),
                    "SF0002".to_string(),
                    self.line,
                    self.column,
                ));
            }

            if int_str.is_empty() {
                number_str = format!("0.{}", frac_str);
            } else if frac_str.is_empty() {
                number_str = format!("{}.0", int_str);
            }

            match number_str.parse::<f64>() {
                Ok(f) => Ok(create_token!(
                    TokenKind::Literal(LiteralToken::Float(f)),
                    start_pos,
                    self.position + 1
                )),
                Err(_) => Err(SnowFallError::new_compiler_error(
                    format!("Failed to parse float: {}", number_str),
                    "SF0002".to_string(),
                    self.line,
                    self.column,
                )),
            }
        } else {
            match number_str.parse::<i64>() {
                Ok(i) => Ok(create_token!(
                    TokenKind::Literal(LiteralToken::Int(i)),
                    start_pos,
                    self.position + 1
                )),
                Err(_) => Err(SnowFallError::new_compiler_error(
                    format!("Failed to parse integer: {}", number_str),
                    "SF0002".to_string(),
                    self.line,
                    self.column,
                )),
            }
        }
    }

    /// 16進数リテラルを読み取ります
    fn read_hex_number(&mut self) -> Result<Token, SnowFallError> {
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
                        return Err(SnowFallError::new_compiler_error(
                            "Invalid hex format: misplaced underscore".to_string(),
                            "SF0002".to_string(),
                            self.line,
                            self.column,
                        ));
                    }
                    prev_was_underscore = true;
                }
                b'.' => {
                    // '_' 直後は NG
                    if prev_was_underscore {
                        return Err(SnowFallError::new_compiler_error(
                            "Invalid hex format: misplaced underscore".to_string(),
                            "SF0002".to_string(),
                            self.line,
                            self.column,
                        ));
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
            return Err(SnowFallError::new_compiler_error(
                "Invalid hex format: misplaced underscore".to_string(),
                "SF0002".to_string(),
                self.line,
                self.column,
            ));
        }

        if self.ch.is_ascii_alphabetic() {
            return Err(SnowFallError::new_compiler_error(
                format!("Invalid character in hex literal: {}", self.ch as char),
                "SF0002".to_string(),
                self.line,
                self.column,
            ));
        }

        // 0xf.f のような16進浮動小数点数を処理します
        if dot_count >= 1 {
            // 基本的な16進浮動小数点解析 (例: "A.B" -> 10.6875)
            let (int_str, frac_str) = match number_str.split_once('.') {
                Some(v) => v,
                None => {
                    return Err(SnowFallError::new_compiler_error(
                        "Invalid hex float format".to_string(),
                        "SF0002".to_string(),
                        self.line,
                        self.column,
                    ));
                }
            };

            if int_str.is_empty() {
                return Err(SnowFallError::new_compiler_error(
                    "Invalid hex float format".to_string(),
                    "SF0002".to_string(),
                    self.line,
                    self.column,
                ));
            }

            let integer_part = match i64::from_str_radix(int_str, 16) {
                Ok(v) => v as f64,
                Err(_) => {
                    return Err(SnowFallError::new_compiler_error(
                        format!("Failed to parse hex integer part: {}", int_str),
                        "SF0002".to_string(),
                        self.line,
                        self.column,
                    ));
                }
            };

            let mut fractional_part: f64 = 0.0;
            let mut base: f64 = 16.0;

            for c in frac_str.chars() {
                let digit = match c.to_digit(16) {
                    Some(d) => d as f64,
                    None => {
                        return Err(SnowFallError::new_compiler_error(
                            format!("Invalid hex digit in fractional part: {}", c),
                            "SF0002".to_string(),
                            self.line,
                            self.column,
                        ));
                    }
                };
                fractional_part += digit / base;
                base *= 16.0;
            }

            return Ok(create_token!(
                TokenKind::Literal(LiteralToken::Float(integer_part + fractional_part)),
                start_pos,
                self.position + 1
            ));
        }

        match i64::from_str_radix(&number_str, 16) {
            Ok(i) => Ok(create_token!(
                TokenKind::Literal(LiteralToken::Int(i)),
                start_pos,
                self.position + 1
            )),
            Err(_) => Err(SnowFallError::new_compiler_error(
                format!("Failed to parse hex integer: {}", number_str),
                "SF0002".to_string(),
                self.line,
                self.column,
            )),
        }
    }

    /// 2進数リテラルを読み取ります
    fn read_binary_number(&mut self) -> Result<Token, SnowFallError> {
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
                        return Err(SnowFallError::new_compiler_error(
                            "Invalid binary format: misplaced underscore".to_string(),
                            "SF0002".to_string(),
                            self.line,
                            self.column,
                        ));
                    }
                    prev_was_underscore = true;
                }
                b'.' => {
                    // '_' 直後は NG
                    if prev_was_underscore {
                        return Err(SnowFallError::new_compiler_error(
                            "Invalid binary format: misplaced underscore".to_string(),
                            "SF0002".to_string(),
                            self.line,
                            self.column,
                        ));
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
            return Err(SnowFallError::new_compiler_error(
                "Invalid binary format: misplaced underscore".to_string(),
                "SF0002".to_string(),
                self.line,
                self.column,
            ));
        }

        if self.ch.is_ascii_alphanumeric() {
            return Err(SnowFallError::new_compiler_error(
                format!("Invalid character in binary literal: {}", self.ch as char),
                "SF0002".to_string(),
                self.line,
                self.column,
            ));
        }

        // 0b1.1 のような2進浮動小数点数を処理します
        if dot_count >= 1 {
            // 基本的な2進浮動小数点解析 (例: "1.1" -> 1.5)
            let (int_str, frac_str) = match number_str.split_once('.') {
                Some(v) => v,
                None => {
                    return Err(SnowFallError::new_compiler_error(
                        "Invalid binary float format".to_string(),
                        "SF0002".to_string(),
                        self.line,
                        self.column,
                    ));
                }
            };

            if int_str.is_empty() {
                return Err(SnowFallError::new_compiler_error(
                    "Invalid binary float format".to_string(),
                    "SF0002".to_string(),
                    self.line,
                    self.column,
                ));
            }

            let integer_part = match i64::from_str_radix(int_str, 2) {
                Ok(v) => v as f64,
                Err(_) => {
                    return Err(SnowFallError::new_compiler_error(
                        format!("Failed to parse binary integer part: {}", int_str),
                        "SF0002".to_string(),
                        self.line,
                        self.column,
                    ));
                }
            };

            let mut fractional_part: f64 = 0.0;
            let mut base: f64 = 2.0;

            for c in frac_str.chars() {
                let digit = match c {
                    '0' => 0.0,
                    '1' => 1.0,
                    _ => {
                        return Err(SnowFallError::new_compiler_error(
                            format!("Invalid binary digit in fractional part: {}", c),
                            "SF0002".to_string(),
                            self.line,
                            self.column,
                        ));
                    }
                };
                fractional_part += digit / base;
                base *= 2.0;
            }

            return Ok(create_token!(
                TokenKind::Literal(LiteralToken::Float(integer_part + fractional_part)),
                start_pos,
                self.position + 1
            ));
        }

        match i64::from_str_radix(&number_str, 2) {
            Ok(i) => Ok(create_token!(
                TokenKind::Literal(LiteralToken::Int(i)),
                start_pos,
                self.position + 1
            )),
            Err(_) => Err(SnowFallError::new_compiler_error(
                format!("Failed to parse binary integer: {}", number_str),
                "SF0002".to_string(),
                self.line,
                self.column,
            )),
        }
    }

    fn read_string(&mut self) -> Result<Token, SnowFallError> {
        let start_pos = self.position;

        let quote_char: u8 = self.ch;
        self.read_char(); // skip opening '"' or '\''
        let position = self.position;
        let mut old_ch: u8 = 0;
        while old_ch == b'\\' || self.ch != quote_char && self.ch != 0 {
            old_ch = self.ch;
            self.read_char();
        }

        if self.ch == 0 {
            return Err(SnowFallError::new_compiler_error(
                "Unterminated string".to_string(),
                "SF0003".to_string(),
                self.line,
                self.column,
            ));
        }

        let s = self.input[position..self.position].to_string();

        Ok(create_token!(
            TokenKind::Literal(LiteralToken::String(s)),
            start_pos,
            self.position
        ))
    }
}
