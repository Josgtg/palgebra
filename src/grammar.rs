#![allow(warnings)]

use crate::token::Token;
#[derive(Eq, PartialEq, Clone)]
pub enum Expr {
    Literal(Token),
    Grouping(Box<Expr>),
    Operation(Token),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Null,
}