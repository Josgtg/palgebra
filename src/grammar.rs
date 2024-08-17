use crate::token::Token;

#[derive(Eq, PartialEq, Clone)]
pub enum Expr {
    Literal(Token),
    Grouping(Box<Expr>),
    Operation(Token),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Invalid,
    Null,
}

pub fn interpret(expr: Box<Expr>) -> Box<Expr> {
    let new_expr = match *expr {
        Expr::Literal(_) => *expr,
        Expr::Grouping(expr) => grouping(expr),
        Expr::Operation(op) => operation(op),
        Expr::Binary(left, op, right) => binary(left, op, right),
        Expr::Unary(op, right) => unary(op, right),
        Expr::Invalid => Expr::Invalid,
        Expr::Null => Expr::Null
    };
    Box::new(new_expr)
}

fn grouping(expr: Box<Expr>) -> Expr {
    Expr::Grouping(expr)
}

fn operation(op: Token) -> Expr {
    Expr::Operation(op)
}

fn binary(left: Box<Expr>, op: Token, right: Box<Expr>) -> Expr {
    Expr::Binary(left, op, right)
}

fn unary(op: Token, right: Box<Expr>) -> Expr {
    Expr::Unary(op, right)
}