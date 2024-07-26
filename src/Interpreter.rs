use crate::error::{Error, runtime_error};
use crate::error::Error::RuntimeError;
use crate::expr::Expr;
use crate::token::TokenType;

#[derive(Debug, Clone)]
pub enum Value { // In java version the return type is Object
    Number(f32),
    String(String),
    Bool(bool),
    Nil,
}

pub fn interpret(expr: Expr) {
    match evaluate(expr) {
        Ok(val) => println!("{}", stringify(val)),
        Err(e) => runtime_error(e)
    }
}

fn stringify(val: Value) -> String {
    match val {
        Value::Number(n) => n.to_string(),
        Value::String(s) => s,
        Value::Bool(b) => b.to_string(),
        Value::Nil => "nil".to_string(),
    }
}

fn evaluate(expr: Expr) -> Result<Value, Error> {
    match expr {
        Expr::LiteralNum(num) => Ok(Value::Number(num.unwrap())),
        Expr::LiteralStr(s) => Ok(Value::String(s.unwrap())),
        Expr::LiteralBool(b) => Ok(Value::Bool(b.unwrap())),
        Expr::Grouping(expr) => evaluate(*expr),
        Expr::Unary(op, expr) => {
            let right = evaluate(*expr)?;
            match op.token_type {
                TokenType::Minus => match right {
                    Value::Number(n) => Ok(Value::Number(-n)),
                    _ => Err(RuntimeError(Option::from("Operand must be a number".to_string())))
                },
                TokenType::Bang => match right {
                    Value::Bool(b) => Ok(Value::Bool(!b)),
                    _ => Err(RuntimeError(Option::from("Operand must be a boolean".to_string())))
                },
                _ => Err(RuntimeError(Option::from("Invalid unary operator".to_string())))
            }
        }

        Expr::Binary(left, op, right) => {
            let left = evaluate(*left)?;
            let right = evaluate(*right)?;
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
                _ => Err(RuntimeError(Option::from("Operands must be numbers".to_string()))),
            }
        }
        _ => Err(RuntimeError(Option::from("Invalid expression".to_string()))),
    }
}