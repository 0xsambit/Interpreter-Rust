use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    Ident(String),
    BinaryOp(Box<Expr>, Token, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Expr,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Function(Function),
    Expr(Expr),
}
