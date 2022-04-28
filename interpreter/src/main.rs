mod lox;

use std::env::args;

use crate::lox::Lox;


fn main() {
    let args: Vec<String> = args().collect();
    let mut lox: Lox = Lox::new();

    if args.len() < 1 {
        println!("Usage: rustlox [script]");
        std::process::exit(64);
    } 
    else if args.len() == 2 {
        lox.run_file(&args[1]);
    }
    else {
        lox.run_prompt();
    }
}


