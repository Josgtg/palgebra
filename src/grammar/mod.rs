#![allow(warnings)]

mod expr_formatter;

use std::collections::btree_set::Union;
use std::default;
use std::fmt::{Binary, Debug, Display};

use crate::tests::ast_printer;
use crate::token::Token;

use expr_formatter::format_expression;

enum BinarySameOperator {
    Both,
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
            _ => self.unparenthesized() == other.unparenthesized()
        }
    }

    fn association(&self, token: Token, operator: Token) -> bool {
        true
    }

    pub fn or_association(&self, token: Token) -> bool {
        self.association(token, Token::Or)
    }

    pub fn and_association(&self, token: Token) -> bool {
        self.association(token, Token::And)
    }

    // FIXME: Should return a reference
    /// Removes all unnecessary groups
    pub fn unparenthesized(&self) -> Expr {
        match self {
            Expr::Grouping(e) => e.unparenthesized(),
            Expr::Binary(_, _, _) => Expr::unparenthesized_binary(self.clone()),
            Expr::Unary(o, e) => Expr::unary(o.clone(), e.unparenthesized().clone()),
            _ => self.clone()
        }
    }

    /// Removes groupings, but preserves significant ones in binary operations
    fn unparenthesized_binary(expression: Expr) -> Expr {
        let mut left: Expr;
        let operator: Token;
        let mut right: Expr;

        if let Expr::Binary(l, o, r) = expression {
            left = *l;
            operator = o;
            right = *r;
        } else {
            return expression;
        }

        if let Expr::Grouping(e) = left {
            let etmp = e.unparenthesized();
            if let Expr::Binary(_, _, _) = etmp {
                left = match Expr::same_binary_operator(&etmp) {
                    // Left side has the same operator as the main binary operation, so the parenthesis is not applied to simplify the expression
                    // (q & s) & p -> q & s & p
                    BinarySameOperator::Both | BinarySameOperator::Left => etmp,
                    _ => Expr::grouping(etmp)
                }
            } else {
                left = etmp;
            }
        } else {
            left = left.unparenthesized();
        }

        if let Expr::Grouping(e) = right {
            let etmp = e.unparenthesized();
            if let Expr::Binary(_, _, _) = etmp {
                right = match Expr::same_binary_operator(&etmp) {
                    // Right side has the same operator as the main binary operation, so the parenthesis is not applied to simplify the expression
                    // q & (s & p) -> q & s & p
                    BinarySameOperator::Both | BinarySameOperator::Right => etmp,
                    _ => Expr::grouping(etmp)
                }
            } else {
                right = etmp;
            }
        } else {
            right = right.unparenthesized();
        }

        Expr::binary(left, operator, right)
    }

    /// Returns true if the operators contained in a binary operation are the same
    fn same_binary_operator(expression: &Expr) -> BinarySameOperator {
        if let Expr::Binary(left, operator, right) = expression {
            let left = left.unparenthesized();
            let right = right.unparenthesized();
            let operator_left: Option<&Token> = match &left {
                Expr::Binary(_, o, _) => Some(o),
                _ => None
            };
            let operator_right: Option<&Token> = match &right {
                Expr::Binary(_, o, _) => Some(o),
                _ => None
            };

            return match (operator_left, operator_right) {
                (Some(t1), Some(t2)) => {
                    if t1 == operator {
                        if t2 == operator {
                            BinarySameOperator::Both
                        } else {
                            BinarySameOperator::Left
                        }
                    } else if t2 == operator {
                        BinarySameOperator::Right
                    } else {
                        BinarySameOperator::None
                    }
                }
                (Some(t), None) => {
                    if t == operator {
                        BinarySameOperator::Left
                    } else {
                        BinarySameOperator::None 
                    }
                }
                (None, Some(t)) => {
                    if t == operator {
                        BinarySameOperator::Right
                    } else {
                        BinarySameOperator::None
                    }
                },
                (None, None) => BinarySameOperator::None
            };
        }
        BinarySameOperator::None
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
        self == &Expr::Null
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
