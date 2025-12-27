use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Token {
    // Special tokens
    Eof,
    Illegal(String),

    // Identifiers and literals
    Ident(String),
    Int(i64),
    Float(f64),
    String(String),

    // Operators
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    Eq,       // ==
    StrictEq, // ===
    Dot,      // .
    NotEq,    // != (Bonus, good to have)
    StrictNotEq, // !== (Bonus, good to have)
    Lt,       // <
    Gt,       // >
    Bang,     // !

    // Delimiters
    Comma,     // ,
    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]

    // Keywords
    Function, // function
    Sub,      // sub
    Class,    // class
    If,       // if
    Else,     // else
    For,      // for
    While,    // while
    In,       // in
    Of,       // of
    Return,   // return
    True,     // true
    False,    // false
    And,      // and
    Or,       // or
    Override, // @override (as a keyword for simplicity)

    // Logical Operators
    LogicalAnd, // &&
    LogicalOr,  // ||
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8,               // current char under examination
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0; // ASCII code for "NUL"
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    if self.peek_char() == b'=' {
                        self.read_char();
                        Token::StrictEq
                    } else {
                        Token::Eq
                    }
                } else {
                    Token::Assign
                }
            }
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                     if self.peek_char() == b'=' {
                        self.read_char();
                        Token::StrictNotEq
                    } else {
                        Token::NotEq
                    }
                } else {
                    Token::Bang
                }
            }
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
            b'.' => Token::Dot,
            b'<' => Token::Lt,
            b'>' => Token::Gt,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'[' => Token::LBracket,
            b']' => Token::RBracket,
            b'&' => {
                if self.peek_char() == b'&' {
                    self.read_char();
                    Token::LogicalAnd
                } else {
                    Token::Illegal(self.ch.to_string())
                }
            }
            b'|' => {
                if self.peek_char() == b'|' {
                    self.read_char();
                    Token::LogicalOr
                } else {
                    Token::Illegal(self.ch.to_string())
                }
            }
            b'"' => self.read_string(),
            b'0'..=b'9' => return self.read_number(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();
                return match ident.as_str() {
                    "function" => Token::Function,
                    "sub" => Token::Sub,
                    "class" => Token::Class,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "for" => Token::For,
                    "while" => Token::While,
                    "in" => Token::In,
                    "of" => Token::Of,
                    "return" => Token::Return,
                    "true" => Token::True,
                    "false" => Token::False,
                    "and" => Token::And,
                    "or" => Token::Or,
                    _ => Token::Ident(ident),
                };
            }
             b'@' => {
                // For now, we only support @override
                let ident = self.read_identifier();
                if ident == "override" {
                    Token::Override
                } else {
                    Token::Illegal(format!("@{}", ident))
                }
            }
            0 => Token::Eof,
            _ => Token::Illegal(self.ch.to_string()),
        };

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> Token {
        let position = self.position;
        let mut is_float = false;

        // Check for base prefixes
        if self.ch == b'0' {
            match self.peek_char() {
                b'x' | b'X' => return self.read_hex_number(),
                b'b' | b'B' => return self.read_binary_number(),
                _ => {}
            }
        }

        let mut number_str = String::new();
        while self.ch.is_ascii_digit() || self.ch == b'_' || self.ch == b'.' {
            if self.ch == b'.' {
                if is_float { break; } // no more than one dot
                is_float = true;
                number_str.push('.');
            } else if self.ch != b'_' {
                 number_str.push(self.ch as char);
            }
            self.read_char();
        }

        if is_float {
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

    fn read_hex_number(&mut self) -> Token {
        self.read_char(); // skip '0'
        self.read_char(); // skip 'x'
        let position = self.position;
        while self.ch.is_ascii_hexdigit() || self.ch == b'_' || self.ch == b'.' {
            self.read_char();
        }
        let number_str: String = self.input[position..self.position].chars().filter(|&c| c != '_').collect();

        // Handle hex float like 0xf.f
        if number_str.contains('.') {
             // Basic hex float parsing, e.g., "A.B" -> 10.6875
            let parts: Vec<&str> = number_str.split('.').collect();
            if parts.len() == 2 {
                let integer_part = i64::from_str_radix(parts[0], 16).unwrap_or(0);
                let fractional_part = parts[1].chars().enumerate().fold(0.0, |acc, (i, c)| {
                    acc + c.to_digit(16).unwrap_or(0) as f64 / 16.0_f64.powi(i as i32 + 1)
                });
                return Token::Float(integer_part as f64 + fractional_part);
            } else {
                return Token::Illegal(number_str);
            }
        }

        match i64::from_str_radix(&number_str, 16) {
            Ok(i) => Token::Int(i),
            Err(_) => Token::Illegal(number_str),
        }
    }

    fn read_binary_number(&mut self) -> Token {
        self.read_char(); // skip '0'
        self.read_char(); // skip 'b'
        let position = self.position;
        while self.ch == b'0' || self.ch == b'1' || self.ch == b'_' {
            self.read_char();
        }
        let number_str: String = self.input[position..self.position].chars().filter(|&c| c != '_').collect();
        match i64::from_str_radix(&number_str, 2) {
            Ok(i) => Token::Int(i),
            Err(_) => Token::Illegal(number_str),
        }
    }

    fn read_string(&mut self) -> Token {
        self.read_char(); // skip opening '"'
        let position = self.position;
        while self.ch != b'"' && self.ch != 0 {
            self.read_char();
        }
        let s = self.input[position..self.position].to_string();
        if self.ch == b'"' {
           // self.read_char(); // skip closing '"'
        }
        Token::String(s)
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }
}
