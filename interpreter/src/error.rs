#[derive(Clone, Debug)]
pub enum LoxError {
    Error { line: usize, message: String },
}

impl LoxError {
    pub fn error(line: usize, message: String) -> LoxError{
        let mut err = LoxError::Error {
            line, message
        };
        err.report(String::from(""));
        //Lox.had_error = true;
        err
    }

    pub fn report(&mut self, err: String) {
        match self {
            LoxError::Error {line, message} => {
                eprintln!("[line {0}] Error{1}: {2}", line, err, message);
            }
        }

    }
}