use crate::environment::Environment;
use crate::error::{Error, runtime_error};
use crate::error::Error::RuntimeError;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::TokenType;

#[derive(PartialEq, Debug, Clone)]
pub enum Value { // In java version the return type is Object
    Number(f32),
    String(String),
    Bool(bool),
    Nil,
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new()
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            match self.interpret_stmt(stmt) {
                Ok(_) => {}
                Err(e) => runtime_error(e),
            }
        }
    }

    fn interpret_stmt(&mut self, stmt: Stmt) -> Result<Value, Error> {
        match stmt {
            Stmt::Expression(expr) => self.evaluate(expr),
            Stmt::Print(expr) => {
                let result = self.evaluate(expr);
                if let Ok(ref val) = result {
                    println!("{}", self.stringify(val));
                }
                result
            }
            Stmt::Var(name, initializer) => {
                match initializer {
                    Some(expr) => {
                        let value = self.evaluate(expr)?;
                        self.environment.define(name.lexeme.clone(), value);
                        Ok(Value::Nil)
                    }
                    None => {
                        self.environment.define(name.lexeme.clone(), Value::Nil);
                        Ok(Value::Nil)
                    }
                }
            }
        }
    }

    fn stringify(&self, val: &Value) -> String {
        match val {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Nil => "nil".to_string(),
        }
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Value, Error> {
        match expr {
            Expr::LiteralNum(num) => Ok(Value::Number(num.unwrap())),
            Expr::LiteralStr(s) => Ok(Value::String(s.unwrap())),
            Expr::LiteralBool(b) => Ok(Value::Bool(b.unwrap())),
            Expr::Grouping(expr) => self.evaluate(*expr),
            Expr::Unary(op, expr) => {
                let right = self.evaluate(*expr)?;
                match op.token_type {
                    TokenType::Minus => match right {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(RuntimeError(Option::from("Operand must be a number".to_string()))),
                    },
                    TokenType::Bang => match right {
                        Value::Bool(b) => Ok(Value::Bool(!b)),
                        _ => Err(RuntimeError(Option::from("Operand must be a boolean".to_string()))),
                    },
                    _ => Err(RuntimeError(Option::from("Invalid unary operator".to_string()))),
                }
            }
            Expr::Binary(left, op, right) => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
                match op.token_type {
                    TokenType::Minus => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                        _ => Err(RuntimeError(Option::from("Operands must be numbers".to_string()))),
                    },
                    TokenType::Slash => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                        _ => Err(RuntimeError(Option::from("Operands must be numbers".to_string()))),
                    },
                    TokenType::Star => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                        _ => Err(RuntimeError(Option::from("Operands must be numbers".to_string()))),
                    },
                    TokenType::Plus => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                        (Value::String(l), Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
                        _ => Err(RuntimeError(Option::from("Operands must be both numbers or strings".to_string()))),
                    },
                    TokenType::Greater => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
                        _ => Err(RuntimeError(Option::from("Operands must be numbers".to_string()))),
                    },
                    TokenType::GreaterEqual => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
                        _ => Err(RuntimeError(Option::from("Operands must be numbers".to_string()))),
                    },
                    TokenType::Less => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
                        _ => Err(RuntimeError(Option::from("Operands must be numbers".to_string()))),
                    },
                    TokenType::LessEqual => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
                        _ => Err(RuntimeError(Option::from("Operands must be numbers".to_string()))),
                    },
                    TokenType::BangEqual => Ok(Value::Bool(!self.equals(left, right))),
                    TokenType::EqualEqual => Ok(Value::Bool(self.equals(left, right))),
                    _ => Err(RuntimeError(Option::from("Operands must be numbers".to_string()))),
                }
            }
            Expr::Variable(identifier) => {
                self.environment.get(&identifier)
            }
            Expr::Assign(name, value) => {
                let value = self.evaluate(*value)?;
                self.environment.assign(&name, value.clone())?;
                Ok(value)
            }
        }
    }

    fn equals(&self, lhs: Value, rhs: Value) -> bool {
        match (lhs, rhs) {
            (Value::Number(n1), Value::Number(n2)) => (n1 - n2).abs() < f32::EPSILON,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::Nil, Value::Nil) => true,
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    use super::*;

    #[test]
    fn test_evaluate() {
        let expr = Expr::Binary(
            Box::new(Expr::LiteralNum(Some(1.0))),
            Token::new(TokenType::Plus, "+", 1, None, None),
            Box::new(Expr::LiteralNum(Some(2.0))),
        );
        let mut interpreter = Interpreter::new();

        match interpreter.evaluate(expr) {
            Ok(val) => assert_eq!(val, Value::Number(3.0)),
            Err(e) => panic!("Error: {:?}", e)
        }

        let expr = Expr::Binary(
            Box::new(Expr::LiteralNum(Some(1.0))),
            Token::new(TokenType::Minus, "-", 1, None, None),
            Box::new(Expr::LiteralNum(Some(2.0))),
        );
        match interpreter.evaluate(expr) {
            Ok(val) => assert_eq!(val, Value::Number(-1.0)),
            Err(e) => panic!("Error: {:?}", e)
        }

        let expr = Expr::Binary(
            Box::new(Expr::LiteralNum(Some(1.0))),
            Token::new(TokenType::Star, "*", 1, None, None),
            Box::new(Expr::LiteralNum(Some(2.0))),
        );
        match interpreter.evaluate(expr) {
            Ok(val) => assert_eq!(val, Value::Number(2.0)),
            Err(e) => panic!("Error: {:?}", e)
        }

        let expr = Expr::Binary(
            Box::new(Expr::LiteralNum(Some(1.0))),
            Token::new(TokenType::Slash, "/", 1, None, None),
            Box::new(Expr::LiteralNum(Some(2.0))),
        );
        match interpreter.evaluate(expr) {
            Ok(val) => assert_eq!(val, Value::Number(0.5)),
            Err(e) => panic!("Error: {:?}", e)
        }

        let expr = Expr::Binary(
            Box::new(Expr::LiteralNum(Some(1.0))),
            Token::new(TokenType::Greater, ">", 1, None, None),
            Box::new(Expr::LiteralNum(Some(2.0))),
        );
        match interpreter.evaluate(expr) {
            Ok(val) => assert_eq!(val, Value::Bool(false)),
            Err(e) => panic!("Error: {:?}", e)
        }

        let expr = Expr::Binary(
            Box::new(Expr::LiteralNum(Some(1.0))),
            Token::new(TokenType::GreaterEqual, ">=", 1, None, None),
            Box::new(Expr::LiteralNum(Some(2.0))),
        );
        match interpreter.evaluate(expr) {
            Ok(val) => assert_eq!(val, Value::Bool(false)),
            Err(e) => panic!("Error: {:?}", e)
        }

        let expr = Expr::Binary(
            Box::new(Expr::LiteralNum(Some(1.0))),
            Token::new(TokenType::Less, "<", 1, None, None),
            Box::new(Expr::LiteralNum(Some(2.0))),
        );
        match interpreter.evaluate(expr) {
            Ok(val) => assert_eq!(val, Value::Bool(true)),
            Err(e) => panic!("Error: {:?}", e)
        }

        let expr = Expr::Binary(
            Box::new(Expr::LiteralNum(Some(1.0))),
            Token::new(TokenType::LessEqual, "<=", 1, None, None),
            Box::new(Expr::LiteralNum(Some(2.0))),
        );
        match interpreter.evaluate(expr) {
            Ok(val) => assert_eq!(val, Value::Bool(true)),
            Err(e) => panic!("Error: {:?}", e)
        }

        let expr = Expr::Binary(
            Box::new(Expr::LiteralNum(Some(1.0))),
            Token::new(TokenType::BangEqual, "!=", 1, None, None),
            Box::new(Expr::LiteralNum(Some(2.0))),
        );
        match interpreter.evaluate(expr) {
            Ok(val) => assert_eq!(val, Value::Bool(true)),
            Err(e) => panic!("Error: {:?}", e)
        }

        let expr = Expr::Binary(
            Box::new(Expr::LiteralNum(Some(1.0))),
            Token::new(TokenType::EqualEqual, "==", 1, None, None),
            Box::new(Expr::LiteralNum(Some(2.0))),
        );
        match interpreter.evaluate(expr) {
            Ok(val) => assert_eq!(val, Value::Bool(false)),
            Err(e) => panic!("Error: {:?}", e)
        }

        let expr = Expr::Binary(
            Box::new(Expr::LiteralStr(Some("Hello".to_string()))),
            Token::new(TokenType::Plus, "+", 1, None, None),
            Box::new(Expr::LiteralStr(Some(" World".to_string()))),
        );
        match interpreter.evaluate(expr) {
            Ok(val) => assert_eq!(val, Value::String("Hello World".to_string())),
            Err(e) => panic!("Error: {:?}", e)
        }

        let stmt = Stmt::Var(
            Token::new(TokenType::Identifier, "a", 1, None, None),
            Some(Expr::LiteralNum(Some(2.0))),
        );
        interpreter.interpret_stmt(stmt).unwrap();

        let stmt = Stmt::Print(Expr::Variable(Token::new(TokenType::Identifier, "a", 1, None, None)));
        match interpreter.interpret_stmt(stmt) {
            Ok(val) => assert_eq!(val, Value::Number(2.0)),
            Err(e) => panic!("Error: {:?}", e)
        }
    }
}