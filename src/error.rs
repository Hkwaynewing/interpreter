use crate::HAD_ERROR;
use crate::token::{Token, TokenType};

pub enum Error {
    ParseError(Option<String>),
    RuntimeError(Option<String>),
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn runtime_error(error: Error) {
    if let Error::RuntimeError(message) = error {
        if let Some(ref msg) = message {
            eprintln!("{}", msg);
        } else {
            eprintln!("Runtime error occurred");
        }
    }
    *HAD_ERROR.lock().unwrap() = true;
}

pub fn error_tok(token: &Token, message: &str) {
    match token.token_type {
        TokenType::Eof => report(token.line, " at end", message),
        _ => report(token.line, &format!(" at '{}'", token.lexeme), message),
    }
}

pub fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
    *HAD_ERROR.lock().unwrap() = true;
}
