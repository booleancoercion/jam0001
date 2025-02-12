pub mod expr;
pub mod stmt;

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
pub enum Stmt {
    Set(String, Expr),
    Push(Expr),
    Check(Expr),
    Pop,
    Print(Expr),
    CommentOp(SpanLit),
    Uncomment(String),
    Copy(String, HalfSpanLit),
    Move(String, HalfSpanLit),
    Comment(Comment),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Comment {
    Valid(Vec<Stmt>),
    Invalid,
    Empty,
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Stmt::Set(ident, expr) => format!("(set {} {})", ident, expr),
                Stmt::Push(expr) => format!("(push {})", expr),
                Stmt::Check(expr) => format!("(check {}", expr),
                Stmt::Pop => "(pop)".to_string(),
                Stmt::Print(expr) => format!("(print {})", expr),
                Stmt::CommentOp(spanlit) => format!("(comment {})", spanlit),
                Stmt::Uncomment(ident) => format!("(uncomment {})", ident),
                Stmt::Copy(ident, halfspanlit) => format!("(copy {} {})", ident, halfspanlit),
                Stmt::Move(ident, halfspanlit) => format!("(move {} {})", ident, halfspanlit),
                Stmt::Comment(Comment::Valid(stmts)) => {
                    let mut buf = "[".to_string();
                    for stmt in stmts {
                        buf.push_str(&format!("{}, ", stmt));
                    }
                    buf.push(']');
                    format!("(commentlit ({}))", buf)
                }
                Stmt::Comment(Comment::Invalid | Comment::Empty) => format!("(commentlit)"),
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NumKind {
    Abs,
    Neg,
    Pos,
}

impl fmt::Display for NumKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Abs => "",
                Self::Neg => "-",
                Self::Pos => "+",
            }
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HalfSpanLit(NumKind, usize);

impl fmt::Display for HalfSpanLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}{}]", self.0, self.1)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SpanLit(pub HalfSpanLit, pub HalfSpanLit);

impl fmt::Display for SpanLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.0, self.1)
    }
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
        token.text(self.input)
    }

    /// Look ahead to the next token without consuming it
    pub fn peek(&mut self) -> TokenKind {
        self.tokens
            .peek()
            .map(|token| token.kind)
            .unwrap_or(TokenKind::Eof)
    }

    /// Peek ahead to the next token and check if its `TokenKind` is `kind`
    pub fn _at(&mut self, kind: TokenKind) -> bool {
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
