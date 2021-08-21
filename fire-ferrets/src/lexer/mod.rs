mod token_kind;
mod types;

pub use token_kind::*;
pub use types::*;

use logos::Logos;

pub struct Lexer<'input> {
    generated: logos::SpannedIter<'input, LogosToken>,
    eof: bool,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            generated: LogosToken::lexer(input).spanned(),
            eof: false,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    /// Wrapper around `logos::SpannedIter::next` that transforms the span + token kind into our custom `Token` object
    fn next(&mut self) -> Option<Self::Item> {
        match self.generated.next() {
            Some((token, span)) => Some(Token {
                kind: TokenKind::from(token),
                span: span.into(),
            }),
            None if self.eof => None,
            None => {
                self.eof = true;
                Some(Token {
                    kind: TokenKind::Eof,
                    span: (0..0).into(),
                })
            }
        }
    }
}

#[test]
/// ah yes very exhaustive
fn test_lexer() {
    let test = "set variable 123
print variable * variable";
    let tokens = Lexer::new(test).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token {
                kind: TokenKind::Set,
                span: Span {
                    start: 0_usize,
                    end: 3_usize
                }
            },
            Token {
                kind: TokenKind::Ident,
                span: Span {
                    start: 4_usize,
                    end: 12_usize
                }
            },
            Token {
                kind: TokenKind::IntLit,
                span: Span {
                    start: 13_usize,
                    end: 16_usize
                }
            },
            Token {
                kind: TokenKind::Newline,
                span: Span {
                    start: 16_usize,
                    end: 17_usize
                }
            },
            Token {
                kind: TokenKind::Print,
                span: Span {
                    start: 17_usize,
                    end: 22_usize
                }
            },
            Token {
                kind: TokenKind::Ident,
                span: Span {
                    start: 23_usize,
                    end: 31_usize
                }
            },
            Token {
                kind: TokenKind::Multiply,
                span: Span {
                    start: 32_usize,
                    end: 33_usize
                }
            },
            Token {
                kind: TokenKind::Ident,
                span: Span {
                    start: 34_usize,
                    end: 42_usize
                }
            },
            Token {
                kind: TokenKind::Eof,
                span: Span {
                    start: 0_usize,
                    end: 0_usize
                }
            },
        ]
    )
}
