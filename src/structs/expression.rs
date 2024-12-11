use std::fmt::Display;

use crate::grammar::Expr;
use crate::tests::ast_printer;

pub struct Expression {
    expression: Expr,
}

impl Expression {
    pub fn new(expression: Expr) -> Self {
        Expression {
            expression
        }
    }

    pub fn get(&self) -> &Expr {
        &self.expression
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&ast_printer::print_ast(self.get()))
    }
}
