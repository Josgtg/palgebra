#[cfg(test)]

use crate::grammar::Expr;
use crate::token::Token;
use crate::ast_printer::print;
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