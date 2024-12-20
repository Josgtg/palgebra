use crate::grammar::Expr;
use crate::token::Token;

    
pub fn simplify(expression: Expr) -> Expr {
    let simplified = absorbtion(expression);
    simplified
}

fn absorbtion(expression: Expr) -> Expr {
    let original = expression.clone();  // Must avoid cloning the value in future implementations
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
                    return original;
                } else if (operator != Token::And && operator != Token::Or) || (operatorr != Token::Or && operatorr != Token::And) {
                    return original;
                } else if operator == operatorr {
                    return original;
                }

                if unary {
                    return Expr::binary(*left, operator, *rightr);
                } else {
                    return *left;
                }
            }
            return original;
        }
        Expr::Grouping(e) => Expr::grouping(absorbtion(*e)),
        _ => original
    }
}
