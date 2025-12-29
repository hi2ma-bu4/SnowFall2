use crate::compiler::ast::{BlockStatement, Expression, Parameter, Program, Statement};
use crate::compiler::lexer::Lexer;
use crate::token::{Token, TokenType};
use crate::utils::{CompilerError, CompilerResult, Span};

// 演算子の優先順位
#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

impl From<&TokenType> for Precedence {
    fn from(token: &TokenType) -> Self {
        match token {
            TokenType::Eq | TokenType::NotEq => Precedence::Equals,
            TokenType::Lt | TokenType::Gt => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Slash | TokenType::Asterisk => Precedence::Product,
            TokenType::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<CompilerError>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            cur_token,
            peek_token,
            errors: vec![],
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, Vec<CompilerError>> {
        let mut statements = vec![];

        while self.cur_token.token_type != TokenType::EOF {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => self.errors.push(e),
            }
            self.next_token();
        }

        if !self.errors.is_empty() {
            Err(self.errors.drain(..).collect())
        } else {
            Ok(Program { statements })
        }
    }

    fn parse_statement(&mut self) -> CompilerResult<Statement> {
        match self.cur_token.token_type {
            TokenType::Let | TokenType::Const => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Fn => self.parse_function_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> CompilerResult<Statement> {
        let start_span = self.cur_token.span;

        // let <ident>
        if !matches!(self.peek_token.token_type, TokenType::Identifier(_)) {
            return Err(CompilerError::new(
                "Expected identifier after let".into(),
                self.peek_token.span,
            ));
        }
        self.next_token();
        let name = self.cur_token.literal.clone();

        // Optional Type Annotation: let x: number
        let mut type_annotation = None;
        if self.peek_token.token_type == TokenType::Colon {
            self.next_token(); // consume ident
            self.next_token(); // consume colon
            if let TokenType::Identifier(ref t) = self.cur_token.token_type {
                type_annotation = Some(t.clone());
            } else {
                return Err(CompilerError::new(
                    "Expected type identifier".into(),
                    self.cur_token.span,
                ));
            }
        }

        // =
        if self.peek_token.token_type != TokenType::Assign {
            return Err(CompilerError::new(
                "Expected '='".into(),
                self.peek_token.span,
            ));
        }
        self.next_token(); // consume type or ident
        self.next_token(); // consume =

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        let end_span = value.span(); // 簡易的に値のspanを使う
        let span = Span::new(
            start_span.line,
            end_span.column,
            start_span.start_index,
            end_span.end_index,
        );

        Ok(Statement::Let {
            name,
            type_annotation,
            value,
            span,
        })
    }

    fn parse_return_statement(&mut self) -> CompilerResult<Statement> {
        let start_span = self.cur_token.span;
        self.next_token();

        let return_value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        let span = Span::new(
            start_span.line,
            return_value.span().column,
            start_span.start_index,
            return_value.span().end_index,
        );

        Ok(Statement::Return { return_value, span })
    }

    fn parse_function_statement(&mut self) -> CompilerResult<Statement> {
        let start_span = self.cur_token.span;

        // fn <name>
        if !matches!(self.peek_token.token_type, TokenType::Identifier(_)) {
            return Err(CompilerError::new(
                "Expected function name".into(),
                self.peek_token.span,
            ));
        }
        self.next_token();
        let name = self.cur_token.literal.clone();

        // (
        if self.peek_token.token_type != TokenType::LParen {
            return Err(CompilerError::new(
                "Expected '('".into(),
                self.peek_token.span,
            ));
        }
        self.next_token();

        // Params
        let params = self.parse_function_params()?;

        // Optional Return Type: ): type {
        let mut return_type = None;
        if self.peek_token.token_type == TokenType::Colon {
            self.next_token(); // consume ')'
            self.next_token(); // consume ':'
            if let TokenType::Identifier(ref t) = self.cur_token.token_type {
                return_type = Some(t.clone());
            } else {
                return Err(CompilerError::new(
                    "Expected type identifier".into(),
                    self.cur_token.span,
                ));
            }
        }

        // {
        if self.peek_token.token_type != TokenType::LBrace {
            return Err(CompilerError::new(
                "Expected '{'".into(),
                self.peek_token.span,
            ));
        }
        self.next_token();

        let body = self.parse_block_statement()?;
        let span = Span::new(
            start_span.line,
            body.span.column,
            start_span.start_index,
            body.span.end_index,
        );

        Ok(Statement::Function {
            name,
            params,
            return_type,
            body,
            span,
        })
    }

    fn parse_function_params(&mut self) -> CompilerResult<Vec<Parameter>> {
        let mut params = vec![];
        if self.peek_token.token_type == TokenType::RParen {
            self.next_token();
            return Ok(params);
        }

        self.next_token(); // start first param

        loop {
            let start_span = self.cur_token.span;
            let name = if let TokenType::Identifier(ref n) = self.cur_token.token_type {
                n.clone()
            } else {
                return Err(CompilerError::new(
                    "Expected parameter name".into(),
                    self.cur_token.span,
                ));
            };

            // param type: name: type
            if self.peek_token.token_type != TokenType::Colon {
                return Err(CompilerError::new(
                    "Expected ':' for parameter type".into(),
                    self.peek_token.span,
                ));
            }
            self.next_token(); // name
            self.next_token(); // colon

            let type_annotation = if let TokenType::Identifier(ref t) = self.cur_token.token_type {
                t.clone()
            } else {
                return Err(CompilerError::new(
                    "Expected type identifier".into(),
                    self.cur_token.span,
                ));
            };

            let end_span = self.cur_token.span;
            let span = Span::new(
                start_span.line,
                end_span.column,
                start_span.start_index,
                end_span.end_index,
            );
            params.push(Parameter {
                name,
                type_annotation,
                span,
            });

            if self.peek_token.token_type == TokenType::Comma {
                self.next_token();
                self.next_token();
            } else {
                break;
            }
        }

        if self.peek_token.token_type != TokenType::RParen {
            return Err(CompilerError::new(
                "Expected ')'".into(),
                self.peek_token.span,
            ));
        }
        self.next_token();

        Ok(params)
    }

    fn parse_block_statement(&mut self) -> CompilerResult<BlockStatement> {
        let start_span = self.cur_token.span;
        let mut statements = vec![];

        self.next_token(); // skip '{'

        while self.cur_token.token_type != TokenType::RBrace
            && self.cur_token.token_type != TokenType::EOF
        {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }

        let end_span = self.cur_token.span;
        let span = Span::new(
            start_span.line,
            end_span.column,
            start_span.start_index,
            end_span.end_index,
        );
        Ok(BlockStatement { statements, span })
    }

    fn parse_expression_statement(&mut self) -> CompilerResult<Statement> {
        let start_span = self.cur_token.span;
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        let end_span = expression.span();
        let span = Span::new(
            start_span.line,
            end_span.column,
            start_span.start_index,
            end_span.end_index,
        );

        Ok(Statement::ExpressionStmt { expression, span })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> CompilerResult<Expression> {
        let mut left_exp = match self.cur_token.token_type {
            TokenType::Identifier(ref i) => Expression::Identifier(i.clone(), self.cur_token.span),
            TokenType::Int(i) => Expression::Integer(i, self.cur_token.span),
            TokenType::Float(f) => Expression::Float(f, self.cur_token.span),
            TokenType::True => Expression::Boolean(true, self.cur_token.span),
            TokenType::False => Expression::Boolean(false, self.cur_token.span),
            TokenType::StringLiteral(ref s) => {
                Expression::StringLit(s.clone(), self.cur_token.span)
            }
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression()?,
            TokenType::LParen => self.parse_grouped_expression()?,
            TokenType::If => self.parse_if_expression()?,
            _ => {
                return Err(CompilerError::new(
                    format!(
                        "Unexpected token for expression start: {:?}",
                        self.cur_token.token_type
                    ),
                    self.cur_token.span,
                ))
            }
        };

        while self.peek_token.token_type != TokenType::Semicolon
            && precedence < Precedence::from(&self.peek_token.token_type)
        {
            match self.peek_token.token_type {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Slash
                | TokenType::Asterisk
                | TokenType::Eq
                | TokenType::NotEq
                | TokenType::Lt
                | TokenType::Gt => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                }
                TokenType::LParen => {
                    self.next_token();
                    left_exp = self.parse_call_expression(left_exp)?;
                }
                _ => return Ok(left_exp),
            }
        }

        Ok(left_exp)
    }

    fn parse_prefix_expression(&mut self) -> CompilerResult<Expression> {
        let start_span = self.cur_token.span;
        let operator = self.cur_token.token_type.clone();

        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;

        let span = Span::new(
            start_span.line,
            right.span().column,
            start_span.start_index,
            right.span().end_index,
        );
        Ok(Expression::Prefix {
            operator,
            right: Box::new(right),
            span,
        })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> CompilerResult<Expression> {
        let operator = self.cur_token.token_type.clone();
        let precedence = Precedence::from(&operator);

        self.next_token();
        let right = self.parse_expression(precedence)?;

        let span = Span::new(
            left.span().line,
            right.span().column,
            left.span().start_index,
            right.span().end_index,
        );
        Ok(Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            span,
        })
    }

    fn parse_grouped_expression(&mut self) -> CompilerResult<Expression> {
        self.next_token();
        let exp = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token.token_type != TokenType::RParen {
            return Err(CompilerError::new(
                "Expected ')'".into(),
                self.peek_token.span,
            ));
        }
        self.next_token();
        Ok(exp)
    }

    fn parse_if_expression(&mut self) -> CompilerResult<Expression> {
        let start_span = self.cur_token.span;

        if self.peek_token.token_type != TokenType::LParen {
            return Err(CompilerError::new(
                "Expected '(' after if".into(),
                self.peek_token.span,
            ));
        }
        self.next_token();
        self.next_token(); // inside parens

        let condition = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token.token_type != TokenType::RParen {
            return Err(CompilerError::new(
                "Expected ')'".into(),
                self.peek_token.span,
            ));
        }
        self.next_token();

        if self.peek_token.token_type != TokenType::LBrace {
            return Err(CompilerError::new(
                "Expected '{'".into(),
                self.peek_token.span,
            ));
        }
        self.next_token();

        let consequence = self.parse_block_statement()?;
        let mut alternative = None;

        if self.peek_token.token_type == TokenType::Else {
            self.next_token();
            if self.peek_token.token_type != TokenType::LBrace {
                return Err(CompilerError::new(
                    "Expected '{' after else".into(),
                    self.peek_token.span,
                ));
            }
            self.next_token();
            alternative = Some(self.parse_block_statement()?);
        }

        let end_span = alternative
            .as_ref()
            .map(|b| b.span)
            .unwrap_or(consequence.span);
        let span = Span::new(
            start_span.line,
            end_span.column,
            start_span.start_index,
            end_span.end_index,
        );

        Ok(Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
            span,
        })
    }

    fn parse_call_expression(&mut self, function: Expression) -> CompilerResult<Expression> {
        let args = self.parse_expression_list(TokenType::RParen)?;
        let end_span = self.cur_token.span;
        let span = Span::new(
            function.span().line,
            end_span.column,
            function.span().start_index,
            end_span.end_index,
        );

        Ok(Expression::Call {
            function: Box::new(function),
            arguments: args,
            span,
        })
    }

    fn parse_expression_list(&mut self, end: TokenType) -> CompilerResult<Vec<Expression>> {
        let mut list = vec![];

        if self.peek_token.token_type == end {
            self.next_token();
            return Ok(list);
        }

        self.next_token();
        list.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token.token_type == TokenType::Comma {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::Lowest)?);
        }

        if self.peek_token.token_type != end {
            return Err(CompilerError::new(
                "Expected end of argument list".into(),
                self.peek_token.span,
            ));
        }
        self.next_token();

        Ok(list)
    }
}
