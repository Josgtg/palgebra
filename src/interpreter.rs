use std::collections::HashMap;

use crate::grammar::Expr;
use crate::token::Token;


pub fn interpret(values: HashMap<Token, bool>, proposition: Box<Expr>) -> bool {
    let interpreter = Interpreter::new(values);
    interpreter.interpret(proposition)
}

struct Interpreter {
    values: HashMap<Token, bool>,
}

impl Interpreter {

    fn new(values: HashMap<Token, bool>) -> Self {
        Interpreter { values }
    }

    fn interpret(&self, proposition: Box<Expr>) -> bool {
        match *proposition {
            Expr::Grouping(expr) => self.interpret(expr),
            Expr::Binary(left, op, right) => self.binary(left, op, right),
            Expr::Unary(op, right) => self.unary(op, right),
            Expr::Literal(t) => self.literal(t),
            // Should never get here
            _ => false
        }
    }

    fn literal(&self, t: Token) -> bool {
        if t == Token::True { true }
        else if t == Token::False { false }
        else { *self.values.get(&t).unwrap() }
    }

    fn binary(&self, left: Box<Expr>, op: Token, right: Box<Expr>) -> bool {
        match op {
            Token::And => self.interpret(left) && self.interpret(right),
            Token::Or => self.interpret(left) || self.interpret(right),
            Token::IfOnlyIf => self.if_only_if(self.interpret(left), self.interpret(right)),
            Token::IfThen => self.if_then(self.interpret(left), self.interpret(right)),
            // Should never get here
            _ => false
        }
    }

    fn unary(&self, op: Token, right: Box<Expr>) -> bool {
        match op {
            Token::Not => !self.interpret(right),
            // Should never get here
            _ => false
        }
    }

    fn if_only_if(&self, l: bool, r: bool) -> bool {
        if l {
            if r { true }
            else { false }
        }
        else { 
            if r { false }
            else { true }
        }   
    }

    fn if_then(&self, l: bool, r: bool) -> bool {
        if l {
            if r { true }
            else { false }
        }
        else { true }
    }
}