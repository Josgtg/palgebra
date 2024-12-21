#![allow(unused)]

use crate::grammar::Expr;
use crate::token::Token;

pub fn interpret(expr: &Expr) -> bool {
    match expr {
        Expr::Grouping(expr) => interpret(expr),
        Expr::Binary(left, op, right) => binary(left, op, right),
        Expr::Unary(op, right) => unary(op, right),
        Expr::Literal(t) => literal(t),
        // Should never get here
        _ => panic!("interpreter is not working properly"),
    }
}

fn literal(t: &Token) -> bool {
    if t == &Token::True {
        true
    } else if t == &Token::False {
        false
    } else {
        panic!("expression provided to interpreter has undefined literals");
    }
}

fn binary(left: &Expr, op: &Token, right: &Expr) -> bool {
    match op {
        Token::And => interpret(left) && interpret(right),
        Token::Or => interpret(left) || interpret(right),
        Token::IfOnlyIf => if_only_if(interpret(left), interpret(right)),
        Token::IfThen => if_then(interpret(left), interpret(right)),
        // Should never get here
        _ => panic!("interpreter is not working properly"),
    }
}

fn unary(op: &Token, right: &Expr) -> bool {
    match op {
        Token::Not => !interpret(right),
        // Should never get here
        _ => panic!("interpreter is not working properly"),
    }
}

fn if_only_if(l: bool, r: bool) -> bool {
    if l {
        r
    } else {
        !r
    }
}

fn if_then(l: bool, r: bool) -> bool {
    if l {
        r
    } else {
        true
    }
}
