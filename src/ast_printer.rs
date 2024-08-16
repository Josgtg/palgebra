use crate::token::Token;
use crate::grammar::Expr;

pub fn print(expr: Box<Expr>) -> String {
    as_str(expr)
}

fn parenthezise(name: char, exprs: [Option<Box<Expr>>; 2]) -> String {
    let mut s = String::new();
    s.push('(');
    s.push(name);
    for e in exprs {
        if let Some(expr) = e {
            s.push(' ');
            s.push_str(&as_str(expr));
        } else {
            break;
        }
    }
    s.push(')');
    s
}

fn as_str(expr: Box<Expr>) -> String {
    match *expr {
        Expr::Literal(value) => literal(value),
        Expr::Grouping(expr) => grouping(expr),
        Expr::Binary(left, op, right) => binary(left, op, right),
        Expr::Unary(op, right) => unary(op, right),
        Expr::Operation(op) => literal(op),
        Expr::InvalidToken | Expr::Null => String::new()
    }
}

fn literal(value: Token) -> String {
    value.as_char().to_string()
}

fn grouping(expr: Box<Expr>) -> String {
    parenthezise('*', [Some(expr), None])
}

fn binary(left: Box<Expr>, op: Token, right: Box<Expr>) -> String {
    parenthezise(op.as_char(), [Some(left), Some(right)])
}

fn unary(op: Token, right: Box<Expr>) -> String {
    parenthezise(op.as_char(), [Some(right), None])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let expr = Expr::Binary(
            Box::new(Expr::Unary(
                Token::Not,
                Box::new(Expr::Literal(
                    Token::Sentence('p')
                ))
            )),
            Token::And,
            Box::new(Expr::Grouping(
                    Box::new(Expr::Literal(
                        Token::Sentence('q')
                    ))
            ))
        );
        println!("{}", print(Box::new(expr)));
    }

    #[test]
    fn second() {
        let expr = Expr::Binary(
            Box::new(Expr::Binary(
                Box::new(Expr::Binary(
                    Box::new(Expr::Unary(
                        Token::Not,
                        Box::new(Expr::Literal(
                            Token::Sentence('p')
                        )))),
                    Token::Or,
                    Box::new(Expr::Literal(Token::Sentence('p')))
                    )),
                    Token::IfOnlyIf,
                    Box::new(Expr::Literal(
                        Token::Sentence('q')
                ))
            )),
            Token::And,
            Box::new(Expr::Grouping(
                    Box::new(Expr::Literal(
                        Token::Sentence('q')
                    ))
            ))
        );
        println!("{}", print(Box::new(expr)));
    }

    #[test]
    fn third() {
        let expr = Expr::Binary(
            Box::new(Expr::Binary(
                Box::new(Expr::Binary(
                    Box::new(Expr::Unary(
                        Token::Not,
                        Box::new(Expr::Literal(
                            Token::Sentence('p')
                        )))),
                    Token::Or,
                    Box::new(Expr::Literal(Token::Sentence('p')))
                    )),
                    Token::And,
                    Box::new(Expr::Literal(
                        Token::Sentence('q')
                ))
            )),
            Token::IfThen,
            Box::new(Expr::Grouping(
                Box::new(Expr::Binary(
                    Box::new(Expr::Binary(
                        Box::new(Expr::Unary(
                            Token::Not,
                            Box::new(Expr::Literal(
                                Token::Sentence('p')
                            )))),
                        Token::Or,
                        Box::new(Expr::Literal(Token::Sentence('p')))
                        )),
                        Token::Or,
                        Box::new(Expr::Literal(
                            Token::Sentence('q')
                    ))
                )),
            ))
        );
        println!("{}", print(Box::new(expr)));
    }
}