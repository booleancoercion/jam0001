use std::iter::Peekable;

use crate::lexer::{Lexer, Token, TokenKind};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Lit),
    Ident(String),
    BinaryOp(TokenKind, Box<Expr>, Box<Expr>),
    UnaryOp(TokenKind, Box<Expr>),
    FnCall(String, Vec<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Lit {
    Int(usize),
    Str(String),
    Bool(bool),
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

    #[inline]
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
    pub fn consume(&mut self, expected: TokenKind) {
        let token = self.next().unwrap();
        assert_eq!(token.kind, expected);
    }
}
