mod interpreter;
mod lexer;
mod parser;

use std::env;
use std::process;

fn main() {
    let filename = env::args()
        .nth(1)
        .expect("program must be run with an argument which is the file name");

    let source = std::fs::read_to_string(filename).expect("error reading source file");

    let mut parser = parser::Parser::new(&source);
    let mut stmts = vec![];

    loop {
        match parser.parse_stmt() {
            Ok(stmt) => stmts.push(stmt),
            Err(why) if &*why == "Error: Unexpected EOF" => break,
            Err(why) => {
                eprintln!("SYNTAX ERROR - {}", why);
                process::exit(1);
            }
        }
    }

    let mut interpreter = interpreter::Interpreter::new();

    for i in 0..stmts.len() {
        // TODO: make this smarter. the index should move according
        // to comment operations, for example if a comment is copied
        // to an area before the instruction pointer. perhaps put
        // the whole evaluating logic inside the Interpreter?
        interpreter.eval(&stmts[i]);
    }
}
