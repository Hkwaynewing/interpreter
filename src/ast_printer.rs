use std::fmt::Display;

use crate::expr::Expr;
use crate::stmt::Stmt;

pub fn print(expr: &Expr) -> String {
    match expr {
        Expr::Binary(left, op, right) => {
            format!("({} {} {})", op.lexeme, print(left), print(right))
        }
        Expr::Grouping(expr) => {
            format!("(group {})", print(expr))
        }
        Expr::Unary(op, right) => {
            format!("({} {})", op.lexeme, print(right))
        }
        Expr::LiteralNum(opt) => print_literal(opt),
        Expr::LiteralStr(opt) => print_literal(opt),
        Expr::LiteralBool(opt) => print_literal(opt),
        Expr::Variable(_) => { todo!() }
        Expr::Assign(_, _) => { todo!() }
    }
}

pub fn print_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Expression(expr) => print(expr),
        Stmt::Print(expr) => format!("(print {})", print(expr)),
        _ => { todo!() }
    }
}

fn print_literal<T: Display>(opt: &Option<T>) -> String {
    match opt {
        Some(val) => format!("{}", val),
        None => "nil".to_string(),
    }
}


#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenType};

    use super::*;

    #[test]
    fn test_print() {
        let left = Expr::Unary(
            Token::new(TokenType::Minus, "-", 1, None, None),
            Box::new(Expr::LiteralNum(Some(123.0))),
        );
        let right = Expr::Grouping(
            Box::new(Expr::LiteralNum(Some(45.67)))
        );
        let expr = Expr::Binary(
            Box::new(left),
            Token::new(TokenType::Star, "*", 1, None, None),
            Box::new(right),
        );

        assert_eq!(print(&expr), "(* (- 123) (group 45.67))");
    }
}