use crate::error::{Error, error_tok};
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};

/*
expression     → assignment ;
assignment     → IDENTIFIER "=" assignment
               | equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ; (...)* means 0 or more
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ") | IDENTIFIER" ;
 */
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        Ok(statements)
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.equality();
        if self.match_token(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment();
            match expr {
                Ok(expr) => match value {
                    Ok(value) => {
                        if let Expr::Variable(name) = expr {
                            return Ok(Expr::Assign(name, Box::new(value)));
                        }
                        error_tok(&equals, "Invalid assignment target.");
                        return Err(Error::ParseError(Option::from("Invalid assignment target.".to_string())));
                    }
                    Err(e) => return Err(e),
                },
                Err(e) => return Err(e),
            };
        }
        expr
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let token_types = &[TokenType::BangEqual, TokenType::EqualEqual];
        self.parse_binary_expr(token_types, Parser::comparison)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let token_types = &[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual];
        self.parse_binary_expr(token_types, Parser::term)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let token_types = &[TokenType::Minus, TokenType::Plus];
        self.parse_binary_expr(token_types, Parser::factor)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let token_types = &[TokenType::Slash, TokenType::Star];
        self.parse_binary_expr(token_types, Parser::unary)
    }

    fn parse_binary_expr<F>(&mut self, token_types: &[TokenType], next_fn: F) -> Result<Expr, Error>
    where
        F: Fn(&mut Self) -> Result<Expr, Error>,
    {
        let mut left = next_fn(self)?;
        while self.match_token(token_types) {
            let op = self.previous().clone();
            match next_fn(self) {
                Ok(right) => left = Expr::Binary(Box::new(left), op, Box::new(right)),
                Err(e) => return Err(e),
            };
        }
        Ok(left)
    }


    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous().clone();
            match self.unary() {
                Ok(right) => return Ok(Expr::Unary(op, Box::new(right))),
                Err(e) => return Err(e),
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.match_token(&[TokenType::Number]) {
            return Ok(Expr::LiteralNum(self.previous().clone().literal_num));
        }
        if self.match_token(&[TokenType::String]) {
            return Ok(Expr::LiteralStr(self.previous().clone().literal_str.clone()));
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::LiteralBool(Some(true)));
        }
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::LiteralBool(Some(false)));
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::LiteralStr(None));
        }
        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous().clone()));
        }
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        Err(Error::ParseError(Option::from("Expect expression.".to_string())))
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

    fn check(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.current_token().token_type == token_type
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
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

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<&Token, Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        error_tok(self.current_token(), msg);
        Err(Error::ParseError(Option::from(msg.to_string())))
    }
    fn synchonize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            let token_type = self.current_token().token_type;
            if token_type == TokenType::Class
                || token_type == TokenType::Fun
                || token_type == TokenType::Var
                || token_type == TokenType::For
                || token_type == TokenType::If
                || token_type == TokenType::While
                || token_type == TokenType::Print
                || token_type == TokenType::Return {
                return;
            } else {
                self.advance();
            }
        }
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            return self.block_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }

    fn declaration(&mut self) -> Result<Stmt, Error> {
        if self.match_token(&[TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?.clone();
        let value = match self.match_token(&[TokenType::Equal]) {
            true => Some(self.expression()?),
            false => None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var(name, value))
    }
    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression(value))
    }
    fn block_statement(&mut self) -> Result<Stmt, Error> {
        let mut stmts = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            stmts.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(Stmt::Block(stmts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let tokens = vec![
            Token::new(TokenType::Number, "123", 1, None, Some(123.0)),
            Token::new(TokenType::Star, "*", 1, None, None),
            Token::new(TokenType::Number, "45.67", 1, None, Some(45.67)),
            Token::new(TokenType::Eof, "", 1, None, None),
        ];
        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(stmts) => {
                for stmt in stmts {
                    let ast_printer = crate::ast_printer::print_stmt(&stmt);
                    assert_eq!(ast_printer, "(* 123 45.67)");
                }
            }
            Err(e) => {
                assert!(false);
            }
        }
    }
}

