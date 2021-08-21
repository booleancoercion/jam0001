use crate::lexer::TokenKind;

pub enum Expr {
    Literal(Lit),
    Ident(String),
    BinaryOp(TokenKind, Box<Expr>, Box<Expr>),
    UnaryOp(TokenKind, Box<Expr>),
    FnCall(String, Vec<Expr>),
}

pub enum Lit {
    Int(usize),
    Str(String),
    Bool(bool),
}
