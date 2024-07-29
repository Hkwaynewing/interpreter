use std::fmt;
use std::fmt::Display;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal_str: Option<String>,
    pub literal_num: Option<f32>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: usize, literal_str: Option<String>, literal_num: Option<f32>) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string(),
            line,
            literal_str,
            literal_num,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal_str = match &self.literal_str {
            Some(literal) => literal.clone(),
            None => "nil".to_string(),
        };
        write!(f, "Token(type: {}, lexeme: {}, leteral: {})", self.token_type, self.lexeme, literal_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    PRINT,

    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
