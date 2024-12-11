#![allow(warnings)]

use std::fmt::Display;

use crate::token::Token;
use crate::tests::ast_printer;

#[derive(Eq, PartialEq, Clone)]
pub enum Expr {
    Literal(Token),
    Grouping(Box<Expr>),
    Operation(Token),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Null,
}

impl Expr {
    pub fn literal(token: Token) -> Self {
        Expr::Literal(token)
    }

    pub fn grouping(expression: Expr) -> Self {
        Expr::Grouping(Box::new(expression))
    }

    pub fn operation(operator: Token) -> Self {
        Expr::Operation(operator)
    }

    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary(Box::new(left), operator, Box::new(right))
    }

    pub fn unary(operator: Token, right: Expr) -> Self {
        Expr::Unary(operator, Box::new(right))
    }

    pub fn null() -> Self {
        Expr::Null
    }

    pub fn is_null(&self) -> bool {
        self == &Expr::Null
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&ast_printer::print_ast(self))
    }
}
