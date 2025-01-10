use crate::grammar::{BinarySide, Expr};
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
    let mut first_rules: [bool;3];
    let mut second_rules: [bool;2];
    let mut expr_changed = true;
    while expr_changed {
        expr_changed = false;
        first_rules = [true;3];
        while first_rules.contains(&true) {
            (simplified, first_rules[0]) = conditional(simplified);
            (simplified, first_rules[1]) = biconditional(simplified);
            (simplified, first_rules[2]) = absorption(simplified);
            expr_changed = expr_changed || first_rules.contains(&true);
        }
        second_rules = [true;2];
        while second_rules.contains(&true) {
            (simplified, second_rules[0]) = idempotent(simplified);
            (simplified, second_rules[1]) = compliments(simplified);
            expr_changed = expr_changed || second_rules.contains(&true);
        }
    }
    simplified
}

// p & (p | q) == p
// q | (!q & p) == q | p
fn absorption(expression: Expr) -> (Expr, bool) {
    let mut rule_applied: bool = false;
    let opposite_op: Token;
    
    let (left, operator, right): (Expr, Token, Expr);
    if let Expr::Binary(l, o, r) = expression.unparenthesized() {
        opposite_op = if o == Token::And { Token::Or } else { Token::And };
        left = l.unparenthesized();
        operator = o;
        right = r.unparenthesized();
    } else {
        return (expression, rule_applied);
    }

    return if right.match_operator(&opposite_op) {
        if let Some((side, negated)) = check_absorption(&left, &right) {
            rule_applied = true;
            if negated {
                let other: Expr = match (side, right) {
                    (BinarySide::Left, Expr::Binary(.., right)) => *right,
                    (BinarySide::Right, Expr::Binary(left, ..)) => *left,
                    _ => panic!("should not have reached this part")
                };
                (Expr::binary(left, operator, other), rule_applied)
            } else {
                (left, rule_applied)
            }
        } else { (expression, rule_applied) }
    } else if left.match_operator(&opposite_op) {
        if let Some((side, negated)) = check_absorption(&right, &left) {
            rule_applied = true;
            if negated {
                let other: Expr = match (side, left) {
                    (BinarySide::Left, Expr::Binary(.., right)) => *right,
                    (BinarySide::Right, Expr::Binary(left, ..)) => *left,
                    _ => panic!("should not have reached this part")
                };
                (Expr::binary(right, operator, other), rule_applied)
            } else {
                (right, rule_applied)
            }
        } else { (expression, rule_applied) }
    } else {
        (expression, rule_applied)
    }
}

/// Checks if "simple" is inside "to_absorb" in a way that meets the absorption rule.
/// Returns those expressions if the rule is met,
/// the side of the binary operation in "to_absorb" that "simple" appears in and a bool of wether or not it's the normal or the negated rule.
fn check_absorption(simple: &Expr, to_absorb: &Expr) -> Option<(BinarySide, bool)> {
    // First we check for p & (p | q), which is the "normal" absorption rule
    let mut simple_in_to_abs = simple.in_binary(&to_absorb);
    return if simple_in_to_abs != BinarySide::None {
        Some((simple_in_to_abs, false))
    } else {
        // Then we check for p & (!p | q), which is the "negated" absorption rule
        let simple = Expr::unary(Token::Not, simple.clone());
        simple_in_to_abs = simple.in_binary(&to_absorb);
        if simple_in_to_abs != BinarySide::None {
                Some((simple_in_to_abs, true))
        } else {
            None 
        }
    }
}

// p > q == !p | q
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

// p ~ q == (!p | q) & (!q | p)
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

// Next functions must be applied after the other ones have finished

// FIXME: This function can be improved by checking all the combinations in an expression like (p & q) & (q & s) == p & q & s
#[allow(unused_mut)]
// p & p == p
// q | q == q
fn idempotent(expression: Expr) -> (Expr, bool) {
    if let Expr::Binary(left, op, right) = expression.unparenthesized() {
        if left.is_same(&right) {
            return (*left, true);
        } else {
            // Original structure is something like (p & q) & q
            // What I'm doing here is trying to simplify the right side (.. & q) & q if correct
            if let Expr::Binary(l, op2, r) = left.unparenthesized() {
                if op2 != op {
                    return (expression, false);
                }
                let (mut expr, mut applied): (Expr, bool);
                (expr, applied) = idempotent(Expr::binary(*r, op2, *right));
                if applied {
                    return (Expr::binary(*l, op, expr), true);
                }
            }
        }
    }
    (expression, false)
}

// p | !p = T
// q & !q = F
#[allow(warnings)]
pub fn compliments(expression: Expr) -> (Expr, bool) {
    // TODO:
    (expression, false)
}
