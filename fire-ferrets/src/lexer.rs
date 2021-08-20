use logos::Logos;

#[derive(Logos, Clone, Debug, PartialEq)]
/// An enum of all possible tokens
pub enum TokenKind {
    #[token("check")]
    Check,

    #[token("cond_copy")]
    CondCopy,

    #[token("get")]
    Get,

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

    #[token(">=")]
    GreaterEq,

    #[token("!=")]
    NotEq,

    #[token("==")]
    Equals,

    #[regex(r"[ \t\f]+")]
    Whitespace,

    #[error]
    Error,
}
