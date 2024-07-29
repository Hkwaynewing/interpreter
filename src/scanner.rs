use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::error::{error, Error};
use crate::HAD_ERROR;
use crate::token::{Token, TokenType};

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TokenType::And);
    m.insert("class", TokenType::Class);
    m.insert("else", TokenType::Else);
    m.insert("false", TokenType::False);
    m.insert("for", TokenType::For);
    m.insert("fun", TokenType::Fun);
    m.insert("if", TokenType::If);
    m.insert("nil", TokenType::Nil);
    m.insert("or", TokenType::Or);
    m.insert("print", TokenType::Print);
    m.insert("return", TokenType::Return);
    m.insert("super", TokenType::Super);
    m.insert("this", TokenType::This);
    m.insert("true", TokenType::True);
    m.insert("var", TokenType::Var);
    m.insert("while", TokenType::While);
    m
});

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

pub fn scan_tokens(input: String) -> Result<Vec<Token>, Error> {
    let mut scanner: Scanner = Scanner::new(input.as_str());

    scanner.scan_tokens();

    if *HAD_ERROR.lock().unwrap() {
        Err(Error::ParseError(None))
    } else {
        Ok(scanner.tokens)
    }
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }

            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            '"' => self.string(),

            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    error(self.line, &format!("Unexpected character: {}", c));
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let result = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        result
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        true
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Untermited string.");
        }

        // The closing "
        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(TokenType::String, Some(value.to_string()), None);
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance(); // consume the "."
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let result = self.source[self.start..self.current].parse();
        self.add_token_literal(TokenType::Number, None, Some(result.unwrap()));
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let keywords = &self.source[self.start..self.current];
        let token_type = match KEYWORDS.get(keywords) {
            None => TokenType::Identifier,
            Some(&kt) => kt
        };
        self.add_token(token_type)
    }

    fn is_alpha_numeric(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal_str: Option<String>, literal_num: Option<f32>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text, self.line, literal_str, literal_num));
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None, None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_character_tokens() {
        let source = "!".to_string();

        match scan_tokens(source) {
            Ok(tokens) => {
                assert_eq!(tokens.len(), 2);
                assert_eq!(tokens[0].token_type, TokenType::Bang);
                assert_eq!(tokens[0].lexeme, "!");
                assert_eq!(tokens[1].token_type, TokenType::Eof);
            }
            Err(e) => panic!("Error: {:?}", e)
        }
    }

    #[test]
    fn test_single_character_with_equal() {
        let source = "!=".to_string();
        match scan_tokens(source) {
            Ok(tokens) => {
                assert_eq!(tokens.len(), 2);
                assert_eq!(tokens[0].token_type, TokenType::BangEqual);
                assert_eq!(tokens[0].lexeme, "!=");
                assert_eq!(tokens[1].token_type, TokenType::Eof);
            }
            Err(e) => panic!("Error: {:?}", e)
        }
    }

    #[test]
    fn test_keywords() {
        let source = "and class".to_string();

        match scan_tokens(source) {
            Ok(tokens) => {
                assert_eq!(tokens.len(), 3);
                assert_eq!(tokens[0].token_type, TokenType::And);
                assert_eq!(tokens[0].lexeme, "and");
                assert_eq!(tokens[1].token_type, TokenType::Class);
                assert_eq!(tokens[1].lexeme, "class");
                assert_eq!(tokens[2].token_type, TokenType::Eof);
            }
            Err(e) => panic!("Error: {:?}", e)
        }
    }
}
