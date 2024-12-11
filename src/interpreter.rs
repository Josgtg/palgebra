use crate::grammar::Expr;
use crate::token::Token;

pub fn interpret(proposition: &Expr) -> bool {
    match proposition {
        Expr::Grouping(expr) => interpret(expr),
        Expr::Binary(left, op, right) => binary(left, op, right),
        Expr::Unary(op, right) => unary(op, right),
        Expr::Literal(t) => literal(t),
        // Should never get here
        _ => false,
    }
}

fn literal(t: &Token) -> bool {
    t == &Token::True
}

fn binary(left: &Expr, op: &Token, right: &Expr) -> bool {
    match op {
        Token::And => interpret(left) && interpret(right),
        Token::Or => interpret(left) || interpret(right),
        Token::IfOnlyIf => if_only_if(interpret(left), interpret(right)),
        Token::IfThen => if_then(interpret(left), interpret(right)),
        // Should never get here
        _ => false,
    }
}

fn unary(op: &Token, right: &Expr) -> bool {
    match op {
        Token::Not => !interpret(right),
        // Should never get here
        _ => false,
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
