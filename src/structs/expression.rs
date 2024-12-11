use std::fmt::Display;

use crate::grammar::Expr;
use crate::tests::ast_printer;

pub struct Expression {
    expression: Box<Expr>,
}

impl Expression {}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&ast_printer::print_ast(&self.expression))
    }
}
