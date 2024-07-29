- Scanner: str -> Result<Token, Error>
  `2 + 1;` -> `Token::Number(2), Token::Plus, Token::Number(1), Token::Semicolon`
- Parser: Vec<Token> -> Result<Stmt, Error>
  `_` -> `Stmt::Expr(Expr::Binary(Expr::Literal(2), Plus, Expr::Literal(1)))`
- Interpreter: Stmt -> Result<Value, Error>
  `evaluate(Stmt::Expr::Binary)`
  `Expr::Literal(2)` -> `Value::Number(2)`
  `Expr::Literal(1)` -> `Value::Number(1)`
  `Token::Plus` -> `Value::Number(3)`