use crate::grammar::Expr;
use crate::structs::Expression;
use crate::token::Token;

pub fn interpret(expression: &Expression) -> bool {
    interpret_expr(expression.get())
}

fn interpret_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Grouping(expr) => interpret_expr(expr),
        Expr::Binary(left, op, right) => binary(left, op, right),
        Expr::Unary(op, right) => unary(op, right),
        Expr::Literal(t) => literal(t),
        // Should never get here
        _ => panic!("interpreter is not working properly"),
    }
}

fn literal(t: &Token) -> bool {
    t == &Token::True
}

fn binary(left: &Expr, op: &Token, right: &Expr) -> bool {
    match op {
        Token::And => interpret_expr(left) && interpret_expr(right),
        Token::Or => interpret_expr(left) || interpret_expr(right),
        Token::IfOnlyIf => if_only_if(interpret_expr(left), interpret_expr(right)),
        Token::IfThen => if_then(interpret_expr(left), interpret_expr(right)),
        // Should never get here
        _ => panic!("interpreter is not working properly"),
    }
}

fn unary(op: &Token, right: &Expr) -> bool {
    match op {
        Token::Not => !interpret_expr(right),
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
