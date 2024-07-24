use crate::expr::Expr;
use crate::token::{Token, TokenType};

/*
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ; (...)* means 0 or more
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
 */
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for &token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let token_types = &[TokenType::BangEqual, TokenType::EqualEqual];
        let mut expr = self.comparison();
        while self.match_token(token_types) {
            let op = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right))
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let token_types = &[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual];
        let mut expr = self.term();
        while self.match_token(token_types) {
            let op = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right))
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let token_types = &[TokenType::Minus, TokenType::Plus];
        let mut expr = self.factor();
        while self.match_token(token_types) {
            let op = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right))
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let token_types = &[TokenType::Slash, TokenType::Star];
        let mut expr = self.unary();
        while self.match_token(token_types) {
            let op = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right))
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(op, Box::new(right));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(&[TokenType::Number]) {
            return Expr::LiteralNum(self.previous().clone().literal_num);
        }
        if self.match_token(&[TokenType::String]) {
            return Expr::LiteralStr(self.previous().clone().literal_str.clone());
        }
        if self.match_token(&[TokenType::True]) {
            return Expr::LiteralBool(Some(true));
        }
        if self.match_token(&[TokenType::False]) {
            return Expr::LiteralBool(Some(false));
        }
        if self.match_token(&[TokenType::Nil]) {
            return Expr::LiteralStr(None);
        }
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping(Box::new(expr));
        }
        unreachable!("Expect expression.")
    }

    fn check(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.current_token().token_type == token_type
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current - 1];
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) {
        if self.check(token_type) {
            self.advance();
            return;
        }
        panic!("{}", msg)
    }
}

