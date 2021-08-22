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
            Ok(Stmt::Set(text.to_string(), self.expr()?))
        } else {
            Err(self.fmt_error(
                ident.span,
                format!("Expected identifier, got {}", ident.kind),
            ))
        }
    }

    fn parse_push(&mut self) -> StmtResult {
        self.next().unwrap();
        Ok(Stmt::Push(self.expr()?))
    }

    fn parse_check(&mut self) -> StmtResult {
        self.next().unwrap();
        Ok(Stmt::Check(self.expr()?))
    }

    fn parse_pop(&mut self) -> StmtResult {
        self.next().unwrap();
        Ok(Stmt::Pop)
    }

    fn parse_print(&mut self) -> StmtResult {
        self.next().unwrap();
        Ok(Stmt::Print(self.expr()?))
    }
}

#[test]
fn stmt_repl() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let stmt = Parser::new(&input).parse_stmt();
        match stmt {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("{}", e),
        }
    }
}
