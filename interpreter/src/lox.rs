
use std::{fs, io};
use std::io::Write;
use std::process::exit;

//mod scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox{
    pub fn new() -> Self {
        Lox {
            had_error: false,
        }
    }


    pub fn run_file (&mut self, path: &String) {
        let code = fs::read_to_string(path).unwrap();

        self.run(&code);

        if self.had_error {
            exit(0);
        }

    }

    pub fn run_prompt (&mut self) {
        let input = io::stdin();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut buffer = String::new();
            let line = input.read_line(&mut buffer);
            match line {
                //I dont think this is right 
                Ok(0) => break,
                Ok(_) => {
                    self.run(&buffer);
                    self.had_error = false;
                }
                _ => break,
            }
        }
    }

    pub fn run(&mut self, source: &String) {
        println!("{}", source);
        //not implemented yet
        //let mut scanner = scanner::Scanner.new(source);
        //let tokens = scanner.scanTokens();

        // for token in tokens {
        //     println!("{}", token);
        // }
    }

    pub fn error(&mut self, line: u64, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: u64, err: &str, message: &str) {
        println!("[line {0}] Error{1}: {2}", line, err, message);
        self.had_error = true;
    }
}
