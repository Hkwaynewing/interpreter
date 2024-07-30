- Scanner: str -> Result<Token, Error>
  `2 + 1;` -> `Token::Number(2), Token::Plus, Token::Number(1), Token::Semicolon`
  `var a = 2;` -> `Token::Var, Token::Identifier("a"), Token::Equal, Token::Number(2), Token::Semicolon`
  `print a;` -> `Token::Print, Token::Identifier("a"), Token::Semicolon`
- Parser: Vec<Token> -> Result<Stmt, Error>
  `_` -> `Stmt::Expr(Expr::Binary(Expr::Literal(2), Plus, Expr::Literal(1)))`
  `_` -> `Stmt::Var(Identifier("a"), Expr::Literal(2))`
  `_` -> `Stmt::Print(Expr::Variable(Identifier("a")))`
- Interpreter: Stmt -> Result<Value, Error>
  `evaluate(Stmt::Expr::Binary)`
  `Expr::Literal(2)` -> `Value::Number(2)`
  `Expr::Literal(1)` -> `Value::Number(1)`
  `Token::Plus` -> `Value::Number(3)`

  `evalueate(Expr::Literal(2))` -> `Value::Number(2)`
  `Environment.define(Token::Identifier("a"), Value::Number(2)`

  `evaluate(Stmt::Print(Expr::Variable(Identifier("a"))))`
  `Environment.get(Token::Identifier("a"))` -> `Value::Number(2)`
    