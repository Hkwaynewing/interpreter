use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::parser::Parser;

mod token;
mod scanner;
mod error;
mod expr;
mod ast_printer;
mod parser;
mod interpreter;
mod stmt;

static HAD_ERROR: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str) {
    match fs::read_to_string(Path::new(path)) {
        Ok(contents) => {
            run(&contents);
            if *HAD_ERROR.lock().unwrap() {
                std::process::exit(65);
            }
        }
        Err(err) => eprintln!("Error reading file {}: {}", path, err),
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    println!("Running prompt. Type your commands below:");

    loop {
        input.clear();
        print!("> ");
        stdout.flush().unwrap();
        match stdin.read_line(&mut input) {
            Ok(0) => break, // EOF reached
            Ok(_) => {
                let trim = input.trim();
                if trim.is_empty() {
                    continue;
                }
                run(trim);
                *HAD_ERROR.lock().unwrap() = false;
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}

fn run(source: &str) {
    match scanner::scan_tokens(source.to_string()) {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(stmt) => {
                    interpreter::interpret(stmt);
                }
                Err(_) => { std::process::exit(-1); }
            };
        }
        Err(_) => {
            std::process::exit(-1);
        }
    };
}
