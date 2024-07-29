use crate::expr::Expr;

/*
program        → statement* EOF ;

statement      → exprStmt
               | printStmt ;

exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;
 */
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
}