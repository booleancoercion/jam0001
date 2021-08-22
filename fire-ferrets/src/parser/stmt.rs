use crate::lexer::TokenKind;

use super::{Parser, Stmt};

type StmtResult = Result<Stmt, String>;

impl Parser<'_> {
    pub fn parse_stmt(&mut self) -> StmtResult {
        match self.peek() {
            TokenKind::Set => self.parse_set(),
            TokenKind::Push => self.parse_push(),
            TokenKind::Check => self.parse_check(),
            TokenKind::Pop => self.parse_pop(),
            TokenKind::Print => self.parse_print(),
            TokenKind::CommentOp => self.parse_commentop(),
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

    fn parse_set(&mut self) -> StmtResult {
        self.next().unwrap();
        let ident = self.next().unwrap();

        if let TokenKind::Ident = ident.kind {
            let text = self.text(ident);
            let expr = self.expr()?;
            self.consume(TokenKind::Newline)?;
            Ok(Stmt::Set(text.to_string(), expr))
        } else {
            Err(self.fmt_error(
                ident.span,
                format!("Expected identifier, got {}", ident.kind),
            ))
        }
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
    }
}
