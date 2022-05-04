
use std::{fs, io};
use std::io::Write;
use std::process::exit;
use crate::error::*;
use crate::scanner;
use crate::parser::Parser;
use crate::astprinter::AstPrinter;

//mod scanner;

#[derive(Clone, Debug)]
pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox {
            had_error: false,
        }
    }


    pub fn run_file (&mut self, path: &String) -> io::Result<()>{
        let code = fs::read_to_string(path).unwrap();

        match self.run(code) {
            Ok(_) => {},
            Err(mut e) => {
                //TODO: I hate rust
                e.report(String::from(""));
                exit(0);
            }
            // _ => Ok(())
        }
        Ok(())
    }

    pub fn run_prompt (&mut self) {
        let input = io::stdin();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut buffer = String::new();
            let line = input.read_line(&mut buffer);
            match line {
                // TODO: if buffer is empty? 
                Ok(0) => break,
                Ok(_) => {
                    self.run(buffer);
                    self.had_error = false;
                }
                _ => break,
            }
        }
    }

    pub fn run(&mut self, source: String) -> Result<(), LoxError>{
        println!("{}", source);
        let mut scanner = scanner::Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        let mut parser = Parser::new(tokens);
        //let mut expression = parser.parse();
        match parser.parse() {
            None => {},
            Some(expr) => {
                let mut printer = AstPrinter{};
                println!("{}", printer.print(expr));
            }
        }

    
        Ok(())
    }
}
