use crate::expr::Expr;



pub fn print(expr: &Expr) -> String {
    match expr {
        Expr::Binary(left, op, right) => {
            format!("({} {} {})", op.lexeme, print(left), print(right))
        },
        Expr::Grouping(expr) => {
            format!("(group {})", print(expr))
        },
        Expr::LiteralNum(num) => {
            match num {
                Some(num) => format!("{}", num),
                None => "nil".to_string(),
            }
        },
        Expr::LiteralStr(string) => {
            match string {
                Some(string) => format!("\"{}\"", string),
                None => "nil".to_string(),
            }
        },
        Expr::Unary(op, right) => {
            format!("({} {})", op.lexeme, print(right))
        },
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenType};

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
            Token::new(TokenType::Star, "*",  1, None, None),
            Box::new(right),
        );

        assert_eq!(print(&expr), "(* (- 123) (group 45.67))");
    }
}