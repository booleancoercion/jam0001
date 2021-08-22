use crate::lexer::TokenKind;

use super::{HalfSpanLit, NumKind, Parser, SpanLit, Stmt};

type StmtResult = Result<Stmt, String>;

impl Parser<'_> {
    pub fn parse_stmt(&mut self) -> StmtResult {
        match self.peek() {
            TokenKind::Set => self.parse_set(),
            TokenKind::Push => self.parse_push(),
            TokenKind::Check => self.parse_check(),
            TokenKind::Pop => self.parse_pop(),
            TokenKind::Print => self.parse_print(),
            TokenKind::CommentOp => self.parse_comment_op(),
            TokenKind::Uncomment => self.parse_uncomment(),
            TokenKind::Copy => self.parse_copy(),
            TokenKind::Move => self.parse_move(),
            TokenKind::Comment => self.parse_comment(),
            TokenKind::Eof => Err("Error: Unexpected EOF".to_string()),
            _ => {
                let token = self.next().unwrap();
                Err(self.fmt_error(
                    token.span,
                    format!("Expected statement, got {}", token.kind),
                ))
            }
        }
    }

    fn ident(&mut self) -> Result<String, String> {
        let ident = self.next().unwrap();
        if let TokenKind::Ident = ident.kind {
            Ok(self.text(ident).to_string())
        } else {
            Err(self.fmt_error(
                ident.span,
                format!("Expected identifier, got {}", ident.kind),
            ))
        }
    }

    fn span_num(&mut self) -> Result<HalfSpanLit, String> {
        let token = self.next().unwrap();

        match token.kind {
            TokenKind::Minus => {
                let num = self.next().unwrap();
                if num.kind != TokenKind::IntLit {
                    return Err(self.fmt_error(
                        num.span,
                        format!("Expected integer literal, got {}", token.kind),
                    ));
                }
                let text = self.text(num);
                Ok(HalfSpanLit(NumKind::Neg, text.parse().unwrap()))
            }
            TokenKind::Plus => {
                let num = self.next().unwrap();
                if num.kind != TokenKind::IntLit {
                    return Err(self.fmt_error(
                        num.span,
                        format!("Expected integer literal, got {}", token.kind),
                    ));
                }
                let text = self.text(num);
                Ok(HalfSpanLit(NumKind::Pos, text.parse().unwrap()))
            }
            TokenKind::IntLit => {
                let text = self.text(token);
                Ok(HalfSpanLit(NumKind::Abs, text.parse().unwrap()))
            }
            _ => Err(self.fmt_error(
                token.span,
                format!("Expected +, - or integer literal, got {}", token.kind),
            )),
        }
    }

    fn halfspan(&mut self) -> Result<HalfSpanLit, String> {
        self.consume(TokenKind::LeftBracket)?;
        let num = self.span_num()?;
        self.consume(TokenKind::RightBracket)?;
        Ok(num)
    }

    fn span(&mut self) -> Result<SpanLit, String> {
        self.consume(TokenKind::LeftBracket)?;
        let num1 = self.span_num()?;
        self.consume(TokenKind::Colon)?;
        let num2 = self.span_num()?;
        self.consume(TokenKind::RightBracket)?;

        Ok(SpanLit(num1, num2))
    }

    fn parse_set(&mut self) -> StmtResult {
        self.next().unwrap();

        let text = self.ident()?;
        let expr = self.expr()?;
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Set(text.to_string(), expr))
    }

    fn parse_push(&mut self) -> StmtResult {
        self.next().unwrap();
        let expr = self.expr()?;
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Push(expr))
    }

    fn parse_pop(&mut self) -> StmtResult {
        self.next().unwrap();
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Pop)
    }

    fn parse_print(&mut self) -> StmtResult {
        self.next().unwrap();
        let expr = self.expr()?;
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Print(expr))
    }

    fn parse_check(&mut self) -> StmtResult {
        self.next().unwrap();
        let expr = self.expr()?;
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Check(expr))
    }

    fn parse_commentop(&mut self) -> StmtResult {
        self.next().unwrap();
        let span = self.span()?;
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::CommentOp(span))
    }

    fn parse_uncomment(&mut self) -> StmtResult {
        self.next().unwrap();
        let ident = self.ident()?;
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Uncomment(ident))
    }

    fn parse_copy(&mut self) -> StmtResult {
        self.next().unwrap();
        let ident = self.ident()?;
        let halfspan = self.halfspan()?;
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Copy(ident, halfspan))
    }

    fn parse_move(&mut self) -> StmtResult {
        self.next().unwrap();
        let ident = self.ident()?;
        let halfspan = self.halfspan()?;
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Move(ident, halfspan))
    }
}
