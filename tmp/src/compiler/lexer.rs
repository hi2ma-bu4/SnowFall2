use crate::token::{Token, TokenType};
use crate::utils::{
    Span, KEYWORD_CONST, KEYWORD_ELSE, KEYWORD_FALSE, KEYWORD_FN, KEYWORD_IF, KEYWORD_LET,
    KEYWORD_RETURN, KEYWORD_TRUE,
};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.position += c.len_utf8();
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(c)
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let start_pos = self.position;
        let start_col = self.column;
        let start_line = self.line;

        let char_opt = self.read_char();

        let token_type = match char_opt {
            Some('=') => {
                if let Some(&'=') = self.peek_char() {
                    self.read_char();
                    TokenType::Eq
                } else {
                    TokenType::Assign
                }
            }
            Some('+') => TokenType::Plus,
            Some('-') => TokenType::Minus,
            Some('*') => TokenType::Asterisk,
            Some('/') => TokenType::Slash,
            Some('!') => {
                if let Some(&'=') = self.peek_char() {
                    self.read_char();
                    TokenType::NotEq
                } else {
                    TokenType::Bang
                }
            }
            Some('<') => TokenType::Lt,
            Some('>') => TokenType::Gt,
            Some(',') => TokenType::Comma,
            Some(':') => TokenType::Colon,
            Some(';') => TokenType::Semicolon,
            Some('(') => TokenType::LParen,
            Some(')') => TokenType::RParen,
            Some('{') => TokenType::LBrace,
            Some('}') => TokenType::RBrace,
            Some('"') => self.read_string(),
            Some(c) if is_letter(c) => {
                return self.read_identifier(c, start_line, start_col, start_pos)
            }
            Some(c) if c.is_ascii_digit() => {
                return self.read_number(c, start_line, start_col, start_pos)
            }
            None => TokenType::EOF,
            Some(_) => TokenType::Illegal,
        };

        let literal = match token_type {
            TokenType::EOF => "".to_string(),
            TokenType::StringLiteral(ref s) => s.clone(),
            _ => self.input[start_pos..self.position].to_string(),
        };

        Token {
            token_type,
            literal,
            span: Span::new(start_line, start_col, start_pos, self.position),
        }
    }

    fn read_identifier(&mut self, first: char, line: usize, col: usize, start: usize) -> Token {
        let mut literal = String::from(first);
        while let Some(&c) = self.peek_char() {
            if is_letter(c) || c.is_ascii_digit() {
                literal.push(c);
                self.read_char();
            } else {
                break;
            }
        }

        let token_type = match literal.as_str() {
            KEYWORD_LET => TokenType::Let,
            KEYWORD_CONST => TokenType::Const,
            KEYWORD_FN => TokenType::Fn,
            KEYWORD_RETURN => TokenType::Return,
            KEYWORD_TRUE => TokenType::True,
            KEYWORD_FALSE => TokenType::False,
            KEYWORD_IF => TokenType::If,
            KEYWORD_ELSE => TokenType::Else,
            _ => TokenType::Identifier(literal.clone()),
        };

        Token {
            token_type,
            literal,
            span: Span::new(line, col, start, self.position),
        }
    }

    fn read_number(&mut self, first: char, line: usize, col: usize, start: usize) -> Token {
        let mut literal = String::from(first);
        let mut is_float = false;

        while let Some(&c) = self.peek_char() {
            if c.is_ascii_digit() {
                literal.push(c);
                self.read_char();
            } else if c == '.' && !is_float {
                is_float = true;
                literal.push(c);
                self.read_char();
            } else {
                break;
            }
        }

        let token_type = if is_float {
            TokenType::Float(literal.parse().unwrap_or(0.0))
        } else {
            TokenType::Int(literal.parse().unwrap_or(0))
        };

        Token {
            token_type,
            literal,
            span: Span::new(line, col, start, self.position),
        }
    }

    fn read_string(&mut self) -> TokenType {
        let mut string_content = String::new();
        while let Some(&c) = self.peek_char() {
            if c == '"' {
                self.read_char(); // Consume closing quote
                break;
            }
            string_content.push(c);
            self.read_char();
        }
        TokenType::StringLiteral(string_content)
    }
}

fn is_letter(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}
