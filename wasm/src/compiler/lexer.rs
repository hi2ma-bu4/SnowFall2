use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    Identifier(String), // 変数名や型名
    Keyword(String),    // if, for, while, return など
    Operator(String),   // +, ===, and, || など
    LBrace,
    RBrace, // { }
    LParen,
    RParen, // ( )
    Semicolon,
    Assign, // =
    EOF,
}

pub struct Lexer {
    chars: Peekable<IntoIter<char>>,
    pub line: usize,
    pub column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect::<Vec<_>>().into_iter().peekable(),
            line: 1,
            column: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some('\n') = c {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        c
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }

        let c = match self.advance() {
            Some(c) => c,
            None => return Token::EOF,
        };

        // 数値 (0x, 0b, Float)
        if c.is_digit(10) {
            let mut num_str = c.to_string();

            // 16進数/2進数判定
            if c == '0' {
                if let Some(&n) = self.peek() {
                    if n == 'x' || n == 'b' {
                        self.advance(); // x or b
                        num_str.push(n);
                        while let Some(&d) = self.peek() {
                            if d.is_digit(16) || d == '.' {
                                num_str.push(self.advance().unwrap());
                            } else {
                                break;
                            }
                        }
                        if num_str.starts_with("0x") {
                            let val = i64::from_str_radix(&num_str[2..], 16).unwrap_or(0);
                            return Token::IntLiteral(val);
                        } else if num_str.starts_with("0b") {
                            let val = i64::from_str_radix(&num_str[2..], 2).unwrap_or(0);
                            return Token::IntLiteral(val);
                        }
                    }
                }
            }

            while let Some(&d) = self.peek() {
                if d.is_digit(10) || d == '.' {
                    num_str.push(self.advance().unwrap());
                } else {
                    break;
                }
            }

            return if num_str.contains('.') {
                Token::FloatLiteral(num_str.parse().unwrap_or(0.0))
            } else {
                Token::IntLiteral(num_str.parse().unwrap_or(0))
            };
        }

        // 文字列
        if c == '"' {
            let mut s = String::new();
            while let Some(&n) = self.peek() {
                if n == '"' {
                    self.advance();
                    break;
                }
                s.push(self.advance().unwrap());
            }
            return Token::StringLiteral(s);
        }

        // 識別子・キーワード
        if c.is_alphabetic() || c == '_' {
            let mut ident = c.to_string();
            while let Some(&n) = self.peek() {
                if n.is_alphanumeric() || n == '_' {
                    ident.push(self.advance().unwrap());
                } else {
                    break;
                }
            }
            return match ident.as_str() {
                "if" | "else" | "for" | "while" | "return" | "in" => Token::Keyword(ident),
                "and" | "or" => Token::Operator(ident),
                // 型名もここではIdentifierとして扱い、Parserで区別する
                _ => Token::Identifier(ident),
            };
        }

        // 記号・演算子
        match c {
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ';' => Token::Semicolon,
            '=' => {
                if let Some(&'=') = self.peek() {
                    self.advance();
                    if let Some(&'=') = self.peek() {
                        self.advance();
                        Token::Operator("===".to_string())
                    } else {
                        Token::Operator("==".to_string())
                    }
                } else {
                    Token::Assign
                }
            }
            '&' => {
                if let Some(&'&') = self.peek() {
                    self.advance();
                    Token::Operator("&&".to_string())
                } else {
                    Token::Operator("&".to_string())
                }
            }
            '|' => {
                if let Some(&'|') = self.peek() {
                    self.advance();
                    Token::Operator("||".to_string())
                } else {
                    Token::Operator("|".to_string())
                }
            }
            '+' | '-' | '*' | '/' | '<' | '>' | '!' => {
                // 簡易実装: 1文字演算子として扱う (<=, >=などは必要に応じて拡張)
                Token::Operator(c.to_string())
            }
            _ => Token::Operator(c.to_string()),
        }
    }
}
