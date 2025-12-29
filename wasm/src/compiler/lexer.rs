use crate::common::Token;

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

        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    if self.peek_char() == b'=' {
                        self.read_char();
                        // (===)
                        Token::StrictEqual
                    } else {
                        // (==)
                        Token::Equal
                    }
                } else {
                    // (=)
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => {
                if self.peek_char() == b'*' {
                    self.read_char();
                    // (**)
                    Token::Power
                } else {
                    // (*)
                    Token::Asterisk
                }
            }
            b'/' => Token::Slash,
            b'%' => Token::Percent,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    if self.peek_char() == b'=' {
                        self.read_char();
                        // (!==)
                        Token::StrictNotEqual
                    } else {
                        // (!=)
                        Token::NotEqual
                    }
                } else {
                    // (!)
                    Token::Bang
                }
            }
            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    // (<=)
                    Token::LessThanOrEqual
                } else if self.peek_char() == b'<' {
                    self.read_char();
                    if self.peek_char() == b'<' {
                        self.read_char();
                        // (<<<)
                        Token::BitwiseUnsignedLeftShift
                    } else {
                        // (<<)
                        Token::BitwiseLeftShift
                    }
                } else {
                    // (<)
                    Token::LessThan
                }
            }
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    // (>=)
                    Token::GreaterThanOrEqual
                } else if self.peek_char() == b'>' {
                    self.read_char();
                    if self.peek_char() == b'>' {
                        self.read_char();
                        // (>>>)
                        Token::BitwiseUnsignedRightShift
                    } else {
                        // (>>)
                        Token::BitwiseRightShift
                    }
                } else {
                    // (>)
                    Token::GreaterThan
                }
            }
            b'.' => Token::Dot,
            b',' => Token::Comma,
            b':' => Token::Colon,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'[' => Token::LBracket,
            b']' => Token::RBracket,
            b'&' => {
                if self.peek_char() == b'&' {
                    self.read_char();
                    // (&&)
                    Token::LogicalAnd
                } else {
                    // (&)
                    Token::BitwiseAnd
                }
            }
            b'|' => {
                if self.peek_char() == b'|' {
                    self.read_char();
                    // (||)
                    Token::LogicalOr
                } else {
                    // (|)
                    Token::BitwiseOr
                }
            }
            b'^' => Token::BitwiseXor,
            b'~' => Token::BitwiseNot,
            b'"' => self.read_string(),
            b'\'' => self.read_string(),
            b'0'..=b'9' => return self.read_number(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();
                return match ident.as_str() {
                    "function" => Token::Function,
                    "sub" => Token::Sub,
                    "class" => Token::Class,
                    "extends" => Token::Extends,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "for" => Token::For,
                    "while" => Token::While,
                    "in" => Token::In,
                    "of" => Token::Of,
                    "switch" => Token::Switch,
                    "case" => Token::Case,
                    "default" => Token::Default,
                    "break" => Token::Break,
                    "continue" => Token::Continue,
                    "return" => Token::Return,
                    "true" => Token::True,
                    "false" => Token::False,
                    "null" => Token::Null,
                    "and" => Token::And,
                    "or" => Token::Or,
                    _ => Token::Identifiers(ident),
                };
            }
            0 => Token::Eof,
            _ => Token::Illegal(self.ch.to_string()),
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

        let mut dot_count: i32 = 0;
        let mut number_str = String::new();

        let mut prev_was_digit = false;
        let mut prev_was_underscore = false;

        while self.ch.is_ascii_digit() || self.ch == b'_' || self.ch == b'.' {
            match self.ch {
                b'_' => {
                    // 先頭 or '.' 直後は NG
                    if number_str.is_empty() || !prev_was_digit {
                        return Token::Illegal(number_str);
                    }
                    prev_was_underscore = true;
                }
                b'.' => {
                    // '_' 直後は NG
                    if prev_was_underscore {
                        return Token::Illegal(number_str);
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
            return Token::Illegal(number_str);
        }

        if dot_count == 1 {
            let (int_str, frac_str) = match number_str.split_once('.') {
                Some(v) => v,
                None => return Token::Illegal(number_str),
            };
            if int_str.is_empty() && frac_str.is_empty() {
                return Token::Illegal(number_str);
            }

            if int_str.is_empty() {
                number_str = format!("0.{}", frac_str);
            } else if frac_str.is_empty() {
                number_str = format!("{}.0", int_str);
            }

            match number_str.parse::<f64>() {
                Ok(f) => Token::Float(f),
                Err(_) => Token::Illegal(number_str),
            }
        } else {
            match number_str.parse::<i64>() {
                Ok(i) => Token::Int(i),
                Err(_) => Token::Illegal(number_str),
            }
        }
    }

    /// 16進数リテラルを読み取ります
    fn read_hex_number(&mut self) -> Token {
        self.read_char(); // skip '0'
        self.read_char(); // skip 'x'

        let mut dot_count: i32 = 0;
        let mut number_str = String::new();

        let mut prev_was_digit = false;
        let mut prev_was_underscore = false;

        while self.ch.is_ascii_hexdigit() || self.ch == b'_' || self.ch == b'.' {
            match self.ch {
                b'_' => {
                    // 先頭 or '.' 直後は NG
                    if number_str.is_empty() || !prev_was_digit {
                        return Token::Illegal(number_str);
                    }
                    prev_was_underscore = true;
                }
                b'.' => {
                    // '_' 直後は NG
                    if prev_was_underscore {
                        return Token::Illegal(number_str);
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
            return Token::Illegal(number_str);
        }

        // 0xf.f のような16進浮動小数点数を処理します
        if dot_count == 1 {
            // 基本的な16進浮動小数点解析 (例: "A.B" -> 10.6875)
            let (int_str, frac_str) = match number_str.split_once('.') {
                Some(v) => v,
                None => return Token::Illegal(number_str),
            };

            if int_str.is_empty() && frac_str.is_empty() {
                return Token::Illegal(number_str);
            }

            let integer_part = if int_str.is_empty() {
                0.0
            } else {
                match i64::from_str_radix(int_str, 16) {
                    Ok(v) => v as f64,
                    Err(_) => return Token::Illegal(number_str),
                }
            };

            let mut fractional_part: f64 = 0.0;
            let mut base: f64 = 16.0;

            for c in frac_str.chars() {
                let digit = match c.to_digit(16) {
                    Some(d) => d as f64,
                    None => return Token::Illegal(number_str),
                };
                fractional_part += digit / base;
                base *= 16.0;
            }

            return Token::Float(integer_part + fractional_part);
        }

        match i64::from_str_radix(&number_str, 16) {
            Ok(i) => Token::Int(i),
            Err(_) => Token::Illegal(number_str),
        }
    }

    /// 2進数リテラルを読み取ります
    fn read_binary_number(&mut self) -> Token {
        self.read_char(); // skip '0'
        self.read_char(); // skip 'b'

        let mut dot_count: i32 = 0;
        let mut number_str = String::new();

        let mut prev_was_digit = false;
        let mut prev_was_underscore = false;

        while self.ch == b'0' || self.ch == b'1' || self.ch == b'_' || self.ch == b'.' {
            match self.ch {
                b'_' => {
                    // 先頭 or '.' 直後は NG
                    if number_str.is_empty() || !prev_was_digit {
                        return Token::Illegal(number_str);
                    }
                    prev_was_underscore = true;
                }
                b'.' => {
                    // '_' 直後は NG
                    if prev_was_underscore {
                        return Token::Illegal(number_str);
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
            return Token::Illegal(number_str);
        }

        // 0b1.1 のような2進浮動小数点数を処理します
        if dot_count == 1 {
            // 基本的な2進浮動小数点解析 (例: "1.1" -> 1.5)
            let (int_str, frac_str) = match number_str.split_once('.') {
                Some(v) => v,
                None => return Token::Illegal(number_str),
            };

            if int_str.is_empty() && frac_str.is_empty() {
                return Token::Illegal(number_str);
            }

            let integer_part = if int_str.is_empty() {
                0.0
            } else {
                match i64::from_str_radix(int_str, 2) {
                    Ok(v) => v as f64,
                    Err(_) => return Token::Illegal(number_str),
                }
            };

            let mut fractional_part: f64 = 0.0;
            let mut base: f64 = 2.0;

            for c in frac_str.chars() {
                let digit = match c {
                    '0' => 0.0,
                    '1' => 1.0,
                    _ => return Token::Illegal(number_str),
                };
                fractional_part += digit / base;
                base *= 2.0;
            }

            return Token::Float(integer_part + fractional_part);
        }

        match i64::from_str_radix(&number_str, 2) {
            Ok(i) => Token::Int(i),
            Err(_) => Token::Illegal(number_str),
        }
    }

    fn read_string(&mut self) -> Token {
        let quote_char = self.ch;
        self.read_char(); // skip opening '"' or '\''
        let position = self.position;
        let mut old_ch: u8 = 0;
        while old_ch == b'\\' || self.ch != quote_char && self.ch != 0 {
            old_ch = self.ch;
            self.read_char();
        }
        let s = self.input[position..self.position].to_string();
        if self.ch == quote_char {
            // self.read_char(); // skip closing '"'
        }
        Token::String(s)
    }
}
