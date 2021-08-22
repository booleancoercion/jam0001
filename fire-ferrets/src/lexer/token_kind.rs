/*
    The two enums solution is probably far from ideal but the Eof variant is needed for parsing
*/

use std::fmt::Display;

use logos::Logos;

#[derive(Logos, Clone, Debug, PartialEq)]
/// An enum of all tokens for logos
pub enum LogosToken {
    #[token("check")]
    Check,

    #[token("comment")]
    CommentOp,

    #[token("uncomment")]
    Uncomment,

    #[token("copy")]
    Copy,

    #[token("move")]
    Move,

    #[token("pop")]
    Pop,

    #[token("print")]
    Print,

    #[token("push")]
    Push,

    #[token("set")]
    Set,

    #[regex(r#"([A-Za-z]|_)([A-Za-z]|_|\d)*"#)]
    Ident,

    #[regex("[0-9]+")]
    IntLit,

    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    StringLit,

    #[regex(r#"\[(\+|-|)[0-9]+\]"#)]
    HalfSpan,

    #[regex(r#"\[(\+|-|)[0-9]+:(\+|-|)[0-9]+\]"#)]
    FullSpan,

    #[regex(r#"\{[^\}]*\}"#)]
    Comment,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("and")]
    And,

    #[token("not")]
    Not,

    #[token("or")]
    Or,

    #[token("\n")]
    #[token("\r\n")]
    Newline,

    #[token("?")]
    Question,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("<")]
    Less,

    #[token(">")]
    Greater,

    #[token("<=")]
    LessEq,

    #[token(">=")]
    GreaterEq,

    #[token("!=")]
    NotEq,

    #[token("==")]
    Equals,

    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// This is necessary because logos doesn't provide an `#[end]` attribute anymore,
/// so we'll have to manually map `LogosToken` to `TokenKind`,
/// and when we receive `None` from the lexer, turn it into the `Eof` variant
pub enum TokenKind {
    Check,
    CommentOp,
    Uncomment,
    Copy,
    Move,
    Pop,
    Print,
    Push,
    Set,
    Ident,
    IntLit,
    StringLit,
    HalfSpan,
    FullSpan,
    Comment,
    True,
    False,
    And,
    Not,
    Or,
    Newline,
    Question,
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Multiply,
    Divide,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    NotEq,
    Equals,
    Error,
    Eof,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Check => "check",
                Self::CommentOp => "comment operation",
                Self::Uncomment => "uncomment",
                Self::Copy => "copy",
                Self::Move => "move",
                Self::Pop => "pop",
                Self::Print => "print",
                Self::Push => "push",
                Self::Set => "set",
                Self::Ident => "identifier",
                Self::IntLit => "integer literal",
                Self::StringLit => "string literal",
                Self::HalfSpan => "half span",
                Self::FullSpan => "full span",
                Self::Comment => "comment literal",
                Self::True => "true",
                Self::False => "false",
                Self::And => "and",
                Self::Not => "not",
                Self::Or => "or",
                Self::Newline => "newline",
                Self::Question => "question mark",
                Self::LeftParen => "(",
                Self::RightParen => ")",
                Self::Plus => "+",
                Self::Minus => "-",
                Self::Multiply => "*",
                Self::Divide => "/",
                Self::Less => "<",
                Self::Greater => ">",
                Self::LessEq => "<=",
                Self::GreaterEq => ">=",
                Self::NotEq => "!=",
                Self::Equals => "==",
                Self::Error => "error",
                Self::Eof => "EOF",
            }
        )
    }
}

impl From<LogosToken> for TokenKind {
    fn from(logos_token: LogosToken) -> Self {
        match logos_token {
            LogosToken::Check => Self::Check,
            LogosToken::CommentOp => Self::CommentOp,
            LogosToken::Uncomment => Self::Uncomment,
            LogosToken::Copy => Self::Copy,
            LogosToken::Move => Self::Move,
            LogosToken::Pop => Self::Pop,
            LogosToken::Print => Self::Print,
            LogosToken::Push => Self::Push,
            LogosToken::Set => Self::Set,
            LogosToken::Ident => Self::Ident,
            LogosToken::IntLit => Self::IntLit,
            LogosToken::StringLit => Self::StringLit,
            LogosToken::HalfSpan => Self::HalfSpan,
            LogosToken::FullSpan => Self::FullSpan,
            LogosToken::Comment => Self::Comment,
            LogosToken::True => Self::True,
            LogosToken::False => Self::False,
            LogosToken::And => Self::And,
            LogosToken::Not => Self::Not,
            LogosToken::Or => Self::Or,
            LogosToken::Newline => Self::Newline,
            LogosToken::Question => Self::Question,
            LogosToken::LeftParen => Self::LeftParen,
            LogosToken::RightParen => Self::RightParen,
            LogosToken::Plus => Self::Plus,
            LogosToken::Minus => Self::Minus,
            LogosToken::Multiply => Self::Multiply,
            LogosToken::Divide => Self::Divide,
            LogosToken::Less => Self::Less,
            LogosToken::Greater => Self::Greater,
            LogosToken::LessEq => Self::LessEq,
            LogosToken::GreaterEq => Self::GreaterEq,
            LogosToken::NotEq => Self::NotEq,
            LogosToken::Equals => Self::Equals,
            LogosToken::Error => Self::Error,
        }
    }
}
