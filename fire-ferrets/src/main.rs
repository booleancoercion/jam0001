use std::{env, fs, process};

use interpreter::Interpreter;
use parser::{Parser, Stmt};

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().unwrap();
    let contents = fs::read_to_string(filename).unwrap();

    let mut parser = Parser::new(&contents);
    let mut stmts: Vec<Stmt> = Vec::new();
    loop {
        let stmt = parser.parse_stmt();
        match stmt {
            Ok(stmt) => stmts.push(stmt),
            Err(e) => {
                if e == *"Error: Unexpected EOF" {
                    break;
                } else {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        }
    }

    let mut interpreter = Interpreter::new();
    match interpreter.run(&stmts) {
        Ok(u) => u,
        Err(e) => eprintln!("{}", e),
    }
}
