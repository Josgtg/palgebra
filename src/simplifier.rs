use crate::grammar::Expr;
use crate::token::Token;
    
pub fn simplify(expression: Expr) -> Expr {
    rule_recursion(expression)    
}

// FIXME: Does not simplify (q | p) & p, for example, as the program doesn't recognize it's the same as p & (p | q), which is the form it checks.
// FIXME: Does not simplify (p) | ((p) & q) as the program confuses with the agrupations.

fn rule_recursion(expression: Expr) -> Expr {
    let simplified: Expr = match expression {
        Expr::Binary(left,op , right) => Expr::binary(rule_recursion(*left), op, rule_recursion(*right)),
        Expr::Unary(op, right) => Expr::unary(op, rule_recursion(*right)),
        Expr::Grouping(expr) => Expr::grouping(rule_recursion(*expr)),
        _default => _default
    };
    apply_rule(simplified)
}

fn apply_rule(expression: Expr) -> Expr {
    let mut simplified: Expr = expression;
    let mut absorption_applied = true;
    let mut conditional_applied = true;
    let mut biconditional_applied = true;
    while absorption_applied || conditional_applied || biconditional_applied {
        (simplified, absorption_applied) = absorption(simplified);
        (simplified, conditional_applied) = conditional(simplified);
        (simplified, biconditional_applied) = biconditional(simplified);
    }
    simplified
}

fn absorption(expression: Expr) -> (Expr, bool) {
    let original = expression.clone();  // Must avoid cloning the value in future implementations
    match expression {
        // p | (!p & q) => p | q
        Expr::Binary(l, operator, r) => {
            let left: Expr;
            let right: Expr;
            let mut unary = false;
            if let Expr::Grouping(e) = *l {
                left = *e;
            } else {
                left = *l;
            }
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

                if left != *leftr && left != *rightr {
                    return (original, false);
                } else if (operator != Token::And && operator != Token::Or) || (operatorr != Token::Or && operatorr != Token::And) {
                    return (original, false);
                } else if operator == operatorr {
                    return (original, false);
                }

                if unary {
                    return (Expr::binary(left, operator, *rightr), true);
                } else {
                    return (left, true);
                }
            }
            return (original, false);
        }
        Expr::Grouping(e) => {
            let (expression, applied) = absorption(*e);
            return (Expr::grouping(expression), applied);
        }
        _ => (original, false)
    }
}

fn conditional(expression: Expr) -> (Expr, bool) {
    let mut rule_applied = false;
    if let Expr::Binary(left, operator, right) = expression {
        return if operator == Token::IfThen {
            rule_applied = true;
            (Expr::binary(Expr::unary(Token::Not, *left), Token::Or, *right), rule_applied)
        } else {
            (Expr::binary(*left, operator, *right), rule_applied)
        }
    } else if let Expr::Grouping(e) = expression {
        let (expr, rule_applied) = conditional(*e);
        return (Expr::grouping(expr), rule_applied);
    }
    (expression, rule_applied)
}

fn biconditional(expression: Expr) -> (Expr, bool) {
    if let Expr::Binary(left, operator, right) = expression {
        if operator == Token::IfOnlyIf {
            return (
                Expr::binary(
                    Expr::grouping(Expr::binary(
                        Expr::unary(Token::Not, *left.clone()),
                        Token::Or,
                        *right.clone()
                    )),
                    Token::And,
                    Expr::grouping(Expr::binary(
                        Expr::unary(Token::Not, *right),
                        Token::Or,
                        *left
                    )),
                ),
                true
            );
        } else {
            return (Expr::binary(*left, operator, *right), false);
        }
    } else if let Expr::Grouping(e) = expression {
        let (expr, applied) = conditional(*e);
        return (Expr::grouping(expr), applied);
    }
    (expression, false)
}
