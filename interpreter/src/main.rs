mod lox;

use std::env::args;

use crate::lox::Lox;

//mod astprinter;

mod tokentype;
mod token;
mod object;
mod scanner;
mod error;
mod expr;
mod stmt;
mod parser;
mod interpreter;
mod environment;
mod loxcallable;
mod loxfunction;
mod nativefunction;


fn main() {
    let args: Vec<String> = args().collect();
    let mut lox: Lox = Lox::new();

    if args.len() < 1 {
        println!("Usage: rustlox [script]");
        std::process::exit(64);
    } 
    else if args.len() == 2 {
        lox.run_file(&args[1]).expect("Could not run file");
    }
    else {
        lox.run_prompt();
    }
}


