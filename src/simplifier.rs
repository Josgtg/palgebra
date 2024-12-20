use crate::grammar::Expr;
use crate::token::Token;

pub fn simplify(expression: Expr) -> Expr {
    let mut simplifier = Simplifier::new();
    simplifier.simplify(expression)
}

struct Simplifier {
    simplified: Expr,
}

impl Simplifier {
    fn new() -> Self {
        Simplifier {
            simplified: Expr::null(),
        }
    }
    
    fn simplify(&mut self, expression: Expr) -> Expr {
        self.absorbtion(expression);
        self.simplified.clone()
    }

    fn absorbtion(&mut self, expression: Expr) {
        self.simplified = expression.clone();
        match expression {
            // p | (!p & q) => p | q
            Expr::Binary(left, operator, r) => {
                let right: Expr;
                let mut unary = false;
                if let Expr::Grouping(e) = *r {
                    right = *e;
                } else {
                    right = *r;
                }
                
                // p      |          (!p & q)
                // left   operator   right
                if let Expr::Binary(mut leftr, operatorr, rightr) = right {
                    // !p      &           q
                    // leftr   operatorr   rightr
                    
                    if let Expr::Unary(_, rightrl) = *leftr {
                        // !   p
                        // _   rightrl
                        leftr = rightrl;
                        unary = true;
                    }

                    if left != leftr {
                        return;
                    } else if (operator != Token::And && operator != Token::Or) || (operatorr != Token::Or && operatorr != Token::And) {
                        return;
                    } else if operator == operatorr {
                        return;
                    }

                    if unary {
                        self.simplified = Expr::binary(*left, operator, *rightr);
                    } else {
                        self.simplified = *left;
                    }
                }
            }
            _ => println!("NOT BINARY"),
        }
    }
}
