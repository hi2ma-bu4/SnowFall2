use crate::compiler::ast::{AstNode, Statement, Expression};
use crate::compiler::lexer::{Lexer, Token};
use std::mem;

/// # Parser Extensibility Strategy
///
/// The parser is designed to be easily extensible with new syntax, such as a `match` statement.
/// The core of this strategy lies in the modular `parse_statement` function, which acts as a
/// dispatch hub.
///
/// ## To Add a New `match` Statement:
///
/// 1.  **Lexer:** Add a `Match` variant to the `Token` enum in `lexer.rs`.
/// 2.  **AST:** Add a `Match` variant to the `Statement` enum in `ast.rs`, defining the
///     structure of the `match` statement (e.g., the value being matched and a vector of arms).
/// 3.  **Parser:**
///     *   Create a new parsing function, `fn parse_match_statement(&mut self) -> Option<Statement>`.
///         This function will contain all the logic for parsing the `match` keyword, the
///         expression, and its arms, consuming tokens and building the `AstNode::Match`.
///     *   Add a single line to the `parse_statement` function:
///         `Token::Match => self.parse_match_statement(),`
///
/// This "plugin-style" approach ensures that the logic for each grammatical construct is
/// encapsulated within its own function, minimizing the need to modify existing parser code
/// and reducing the risk of introducing regressions.

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
    Cast,        // (Type)X
}

fn token_to_precedence(token: &Token) -> Precedence {
    match token {
        Token::Eq | Token::StrictEq | Token::NotEq | Token::StrictNotEq => Precedence::Equals,
        Token::Lt | Token::Gt => Precedence::LessGreater,
        Token::Plus | Token::Minus => Precedence::Sum,
        Token::Slash | Token::Asterisk => Precedence::Product,
        Token::LParen => Precedence::Call,
        Token::Dot => Precedence::Call, // Member access has the same precedence as a call
        _ => Precedence::Lowest,
    }
}

/// # Error Recovery Strategy
///
/// The parser is designed to report multiple syntax errors in a single pass rather than
/// stopping at the first one. This is achieved through a combination of techniques:
///
/// 1.  **Error Collection:** The `Parser` struct contains an `errors` vector. When a parsing
///     function encounters an invalid token or structure, it pushes a descriptive error
///     message into this vector instead of panicking or exiting immediately.
///
/// 2.  **Optional Return Types:** Most parsing functions (`parse_statement`, `parse_expression`)
///     return an `Option<T>`. If parsing fails, they return `None`. This allows the caller
///     (e.g., the main `parse_program` loop) to simply discard the failed statement or
///     expression and continue parsing from the next token, effectively skipping the
///     erroneous section.
///
/// 3.  **Synchronization (Future Improvement):** For more robust recovery, a "synchronization"
///     step could be implemented. After an error, the parser could advance until it finds a
///     token that is likely to start a new, valid statement (like a semicolon or a keyword).
///     This would prevent a single error from causing a cascade of subsequent, spurious errors.
///
/// This approach provides a much better user experience by presenting a complete list of
/// syntax issues at once, which is more efficient than forcing the user to fix errors one by one.
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut p = Parser {
            lexer,
            cur_token: Token::Eof,
            peek_token: Token::Eof,
            errors: Vec::new(),
        };
        // Load two tokens, so cur_token and peek_token are both set
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> AstNode {
        let mut statements = Vec::new();
        while self.cur_token != Token::Eof {
            match self.parse_statement() {
                Some(stmt) => statements.push(stmt),
                None => {}
            }
            self.next_token();
        }
        AstNode::Program(statements)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::Function => self.parse_function_statement(),
            Token::Sub => self.parse_sub_statement(),
            Token::If => self.parse_if_statement(),
            Token::Return => self.parse_return_statement(),
            Token::LParen => {
                if let Token::Ident(_) = self.peek_token {
                    // Could be a variable declaration `(Int) a = ...` or a cast `(Int) a;`
                    return self.parse_declaration_or_cast_statement();
                }
                self.parse_expression_statement()
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Expression(expr))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // Prefix parsing
        let mut left_exp = match self.cur_token.clone() {
            Token::Ident(name) => Expression::Identifier(name),
            Token::Int(val) => Expression::IntLiteral(val),
            Token::Float(val) => Expression::FloatLiteral(val),
            Token::String(val) => Expression::StringLiteral(val.clone()),
            Token::True => Expression::Boolean(true),
            Token::False => Expression::Boolean(false),
            Token::Bang | Token::Minus => self.parse_prefix_expression()?,
            Token::LParen => {
                 // Could be a grouped expression or a cast
                if self.peek_token_is_ident() {
                    // This is likely a C-style cast, e.g., (Int) my_var
                    return self.parse_cast_expression();
                } else {
                    self.parse_grouped_expression()?
                }
            }
            _ => {
                self.errors.push(format!("No prefix parse function for {:?}", self.cur_token));
                return None;
            }
        };

        // Infix parsing
        while !self.peek_token_is(&Token::Semicolon) && precedence < self.peek_precedence() {
             match self.peek_token {
                Token::Plus | Token::Minus | Token::Slash | Token::Asterisk | Token::Eq | Token::StrictEq | Token::NotEq | Token::StrictNotEq | Token::Lt | Token::Gt | Token::Dot => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                }
                Token::LParen => {
                    self.next_token();
                    left_exp = self.parse_call_expression(left_exp)?;
                }
                _ => return Some(left_exp),
            }
        }

        Some(left_exp)
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.cur_token.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;
        Some(Expression::Prefix {
            operator,
            right: Box::new(right),
        })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.cur_token.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Some(Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
     fn parse_cast_expression(&mut self) -> Option<Expression> {
        // Current token is '('. We already peeked to confirm next is Ident.
        self.next_token(); // Consume '('

        let type_name = match self.cur_token.clone() {
            Token::Ident(name) => name,
            _ => {
                self.errors.push(format!("Expected type name in cast, got {:?}", self.cur_token));
                return None;
            }
        };

        if !self.expect_peek(Token::RParen) { // Consume type name and expect ')'
            return None;
        }

        self.next_token(); // Consume ')' and move to the expression to be casted

        let expression = self.parse_expression(Precedence::Cast)?;

        Some(Expression::Cast {
            target_type: type_name,
            expression: Box::new(expression),
        })
    }


    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token(); // consume '('
        let exp = self.parse_expression(Precedence::Lowest);
        if !self.expect_peek(Token::RParen) {
            return None;
        }
        exp
    }

    fn parse_function_statement(&mut self) -> Option<Statement> {
        // `function` <return_type> <name> `(` ...

        if !self.expect_peek(Token::Ident("".into())) { // Expect return type
             self.errors.push("Expected return type.".into());
             return None;
        }
        let return_type = match self.cur_token.clone() { Token::Ident(s) => s, _ => unreachable!() };

        if !self.expect_peek(Token::Ident("".into())) { // Expect function name
             self.errors.push("Expected function name.".into());
             return None;
        }
        let name = match self.cur_token.clone() { Token::Ident(s) => s, _ => unreachable!() };

        if !self.expect_peek(Token::LParen) { return None; }

        // TODO: Parse parameters

        if !self.expect_peek(Token::RParen) { return None; }
        if !self.expect_peek(Token::LBrace) { return None; }

        let body = self.parse_block_statement()?;

        Some(Statement::Function { name, params: vec![], body: Box::new(body), return_type })
    }

    fn parse_sub_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(Token::Ident("".into())) { // Expect sub name
             self.errors.push("Expected sub name.".into());
             return None;
        }
        let name = match self.cur_token.clone() { Token::Ident(s) => s, _ => unreachable!() };

        if !self.expect_peek(Token::LParen) { return None; }

        // TODO: Parse parameters

        if !self.expect_peek(Token::RParen) { return None; }
        if !self.expect_peek(Token::LBrace) { return None; }

        let body = self.parse_block_statement()?;

        Some(Statement::Sub { name, params: vec![], body: Box::new(body) })
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        self.expect_peek(Token::LParen);
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(Token::RParen);
        self.expect_peek(Token::LBrace);
        let consequence = self.parse_block_statement()?;

        let mut alternative = None;
        if self.peek_token_is(&Token::Else) {
            self.next_token();
            self.expect_peek(Token::LBrace);
            alternative = Some(Box::new(self.parse_block_statement()?));
        }

        Some(Statement::If {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative,
        })
    }

    fn parse_block_statement(&mut self) -> Option<Statement> {
        let mut statements = Vec::new();
        self.next_token(); // consume '{'

        while !self.cur_token_is(&Token::RBrace) && !self.cur_token_is(&Token::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        Some(Statement::Block(statements))
    }

    fn cur_token_is(&self, t: &Token) -> bool {
        &self.cur_token == t
    }

    fn peek_token_is(&self, t: &Token) -> bool {
        &self.peek_token == t
    }
     fn peek_token_is_ident(&self) -> bool {
        matches!(self.peek_token, Token::Ident(_))
    }


    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token(); // consume 'return'

        let return_value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Return(return_value))
    }

    fn parse_declaration_or_cast_statement(&mut self) -> Option<Statement> {
        // We're at '(', peek is 'Ident'
        self.next_token(); // Consume '('
        let type_name = match self.cur_token.clone() {
            Token::Ident(s) => s,
            _ => return None, // Should not happen based on calling condition
        };

        if !self.expect_peek(Token::RParen) { return None; } // Consume type, expect ')'

        if !self.peek_token_is_ident() {
            // This is a cast of a more complex expression, e.g. (Int)(a+b)
            // Re-use expression parser for this.
            let inner_expr = self.parse_expression(Precedence::Cast)?;
            return Some(Statement::Expression(Expression::Cast {
                target_type: type_name,
                expression: Box::new(inner_expr)
            }));
        }

        self.next_token(); // Consume ')' to get to the identifier
        let name = match self.cur_token.clone() {
            Token::Ident(s) => s,
            _ => unreachable!(),
        };

        if !self.peek_token_is(&Token::Assign) {
            // It's a cast expression statement: `(Int) my_var;`
            let ident_expr = Expression::Identifier(name);
            let cast_expr = Expression::Cast {
                target_type: type_name,
                expression: Box::new(ident_expr),
            };
            if self.peek_token_is(&Token::Semicolon) {
                self.next_token();
            }
            return Some(Statement::Expression(cast_expr));
        }

        // It's a let statement
        self.next_token(); // Consume identifier, now at '='
        self.next_token(); // Consume '=', now at start of expression

        let value = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let { name, type_name, value })
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        // A generic check for Identifiers, as we don't care about the value here
        let check = match (&self.peek_token, &t) {
            (Token::Ident(_), Token::Ident(_)) => true,
            _ => self.peek_token_is(&t),
        };

        if check {
            self.next_token();
            true
        } else {
            self.peek_error(&t);
            false
        }
    }

    fn peek_error(&mut self, t: &Token) {
        self.errors.push(format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token
        ));
    }

    fn peek_precedence(&self) -> Precedence {
        token_to_precedence(&self.peek_token)
    }

    fn cur_precedence(&self) -> Precedence {
        token_to_precedence(&self.cur_token)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        // For now, we don't parse arguments
        if !self.expect_peek(Token::RParen) {
            return None;
        }
        Some(Expression::Call {
            function: Box::new(function),
            arguments: vec![],
        })
    }
}
