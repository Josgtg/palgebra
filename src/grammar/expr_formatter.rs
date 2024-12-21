use super::Expr;
use crate::token::Token;

pub fn format_expression(expression: &Expr) -> String {
    as_str(expression)
}

fn as_str(expression: &Expr) -> String {
    match expression {
        Expr::Literal(value) => literal(value),
        Expr::Grouping(expression) => grouping(expression),
        Expr::Binary(left, op, right) => binary(left, op, right),
        Expr::Unary(op, right) => unary(op, right),
        Expr::Operation(op) => literal(op),
        Expr::Null => String::new(),
    }
}

fn literal(value: &Token) -> String {
    value.as_char().to_string()
}

fn grouping(expression: &Expr) -> String {
    let mut s = String::new();
    s.push('(');
    s.push_str(&as_str(expression));
    s.push(')');
    s
}

fn binary(left: &Expr, op: &Token, right: &Expr) -> String {
    format!("{} {} {}", as_str(left), op.as_char(), as_str(right))
}

fn unary(op: &Token, right: &Expr) -> String {
    format!("{}{}", op.as_char(), as_str(right))
}
