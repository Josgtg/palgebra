#![allow(warnings)]

mod expr_formatter;

use std::collections::btree_set::Union;
use std::default;
use std::fmt::{Binary, Debug, Display};

use crate::tests::ast_printer;
use crate::token::Token;

use expr_formatter::format_expression;

pub enum BinarySide {
    Left,
    Right,
    None
}

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
    pub fn is_same(&self, other: &Expr) -> bool {
        match (self, other) {
            (Expr::Grouping(e1), Expr::Grouping(e2)) => e1.is_same(e2),
            (Expr::Binary(l1, o1, r1), Expr::Binary(l2, o2, r2)) => {
                (l1.is_same(&l2) || l1.is_same(&r2)) && o1 == o2 && (r1.is_same(&r2) || r1.is_same(&l2))
            }
            (Expr::Unary(o1, e1), Expr::Unary(o2, e2)) => {
                o1 == o2 && e1.is_same(&e2)
            }
            (s, o) => s.unparenthesized() == o.unparenthesized()
        }
    }

    // FIXME: Should return a reference
    /// Removes all unnecessary groups
    pub fn unparenthesized(&self) -> Expr {
        match self {
            Expr::Grouping(e) => e.unparenthesized(),
            Expr::Binary(..) => Expr::unparenthesized_binary(self.clone()),
            Expr::Unary(o, e) => Expr::unary(o.clone(), e.unparenthesized()),
            default => default.clone()
        }
    }

    /// Removes groupings, but preserves significant ones in binary operations
    fn unparenthesized_binary(expression: Expr) -> Expr {
        if let Expr::Binary(l, operator, r) = expression {
            let mut left: Expr = l.unparenthesized();
            let mut right: Expr = r.unparenthesized();

            if let Expr::Binary(..) = &left {
                left = Expr::grouping(left);
            }
    
            if let Expr::Binary(..) = &right {
                right = Expr::grouping(right);
            }

            Expr::binary(left, operator, right)
        } else {
            expression.unparenthesized()
        }
    }

    pub fn in_binary(&self, expression: &Expr) -> BinarySide {
        if let Expr::Binary(left, _, right) = self {
            return if left.is_same(expression) {
                BinarySide::Left
            } else if right.is_same(expression) {
                BinarySide::Right
            } else {
                BinarySide::None
            };
        }
        BinarySide::None
    }
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
        self.is_same(&Expr::Null)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {        
        f.write_str(&format_expression(self))
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&ast_printer::print_ast(self))
    }
}
