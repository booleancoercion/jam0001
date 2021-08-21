pub mod expr;

use std::{fmt, iter::Peekable};

use crate::lexer::{Lexer, Span, Token, TokenKind};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Lit),
    Ident(String),
    BinaryOp(TokenKind, Box<Expr>, Box<Expr>),
    UnaryOp(TokenKind, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expr::Literal(l) => l.to_string(),
                Expr::Ident(i) => i.to_string(),
                Expr::BinaryOp(op, lhs, rhs) => format!("({} {} {})", op, lhs, rhs),
                Expr::UnaryOp(op, expr) => format!("({} {})", op, expr),
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Lit {
    Int(i64),
    Str(String),
    Bool(bool),
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Int(i) => i.to_string(),
                Self::Str(s) => s.to_string(),
                Self::Bool(b) => b.to_string(),
            }
        )
    }
}

/// The parser which contains the input string and lexer
pub struct Parser<'input> {
    /// In order for `Token::text` to work
    input: &'input str,
    /// The lexer wrapped in `Peekable`
    tokens: Peekable<Lexer<'input>>,
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            tokens: Lexer::new(input).peekable(),
        }
    }

    /// Get the source text of a given token
    pub fn text(&self, token: Token) -> &'input str {
        token.text(&self.input)
    }

    /// Look ahead to the next token without consuming it
    pub fn peek(&mut self) -> TokenKind {
        self.tokens
            .peek()
            .map(|token| token.kind)
            .unwrap_or(TokenKind::Eof)
    }

    /// Peek ahead to the next token and check if its `TokenKind` is `kind`
    pub fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == kind
    }

    /// Consume and return the next token
    pub fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    /// Consume token and check that it's `TokenKind` is as `expected`
    pub fn consume(&mut self, expected: TokenKind) -> Result<(), String> {
        let token = self.next().unwrap();
        if token.kind != expected {
            Err(self.fmt_error(
                token.span,
                format!("Expected {}, got {}", expected, token.kind),
            ))
        } else {
            Ok(())
        }
    }

    /// Format error with line, column and message
    pub fn fmt_error(&self, span: Span, msg: String) -> String {
        let (line, column) = span.get_line_and_column(self.input);
        format!("Error at {}:{} = {}", line, column, msg)
    }
}
