use std::env::args;
use std::{fs, io};

fn run_file (path: &String) {
    let code = fs::read_to_string(path).unwrap();

    run(&code);

}

fn run_prompt () {
    let input = io::stdin();

    loop {
        println!("> ");
        let mut buffer = String::new();
        let line = input.read_line(&mut buffer);
        match line {
            Ok(0) => break,
            Ok(_) => {
                println!("{}", buffer);
            }
            _ => break,
        }
    }
}

fn run(_source: &String) {

    println!("run ran lol");
}

fn main() {
    println!("hello?");
    let args: Vec<String> = args().collect();

    if args.len() < 1 {
        println!("Usage: rustlox [script]");
        std::process::exit(64);
    } 
    else if args.len() == 2 {
        run_file(&args[1]);
    }
    else {
        run_prompt();
    }
}


