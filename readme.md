- Scanner: str -> Result<Token, Error>
    1. `2 + 1;` -> `Token::Number(2), Token::Plus, Token::Number(1), Token::Semicolon`
    2. `var a = 2;` -> `Token::Var, Token::Identifier("a"), Token::Equal, Token::Number(2), Token::Semicolon`
    3. `print a;` -> `Token::Print, Token::Identifier("a"), Token::Semicolon`
    4. `a = 3` -> `Token::Identifier("a"), Token::Equal, Token::Number(3)`
- Parser: Vec<Token> -> Result<Stmt, Error>
    1. `_` -> `Stmt::Expr(Expr::Binary(Expr::Literal(2), Token::Plus, Expr::Literal(1)))`
    2. `_` -> `Stmt::Var(Token::Identifier("a"), Expr::Literal(2))`
    3. `_` -> `Stmt::Print(Expr::Variable(Identifier("a")))`
    4. `_` -> `Stmt::Expr(Expr::Assign(Token::Identifier("a"), Expr::Literal(3)))`
- Interpreter: Stmt -> Result<Value, Error>
    1. `evaluate(Stmt::Expr::Binary)`
       `Expr::Literal(2)` -> `Value::Number(2)`
       `Expr::Literal(1)` -> `Value::Number(1)`
       `Token::Plus` -> `Value::Number(3)`

    2. `evalueate(Expr::Literal(2))` -> `Value::Number(2)`
       `Environment.define(Token::Identifier("a"), Value::Number(2)`

    3. `evaluate(Stmt::Print(Expr::Variable(Identifier("a"))))`
       `Environment.get(Token::Identifier("a"))` -> `Value::Number(2)`

    4. `evaluate(Stmt::Expr(Expr::Assign(Token::Identifier("a"), Expr::Literal(3))))`
       `Environment.assign(Token::Identifier("a"), Value::Number(3))`  