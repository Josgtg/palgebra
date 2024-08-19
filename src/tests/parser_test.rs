#[cfg(test)]

mod tests {
    use crate::scanner::scan;
    use crate::parser::parse;
    use crate::tests::ast_printer::print_ast;
    use crate::grammar::Expr;

    fn debug(expr: Box<Expr>) {
        println!("{:?}", print_ast(expr));
    }

    fn assert_ok(proposition: &str) {
        println!("{}", proposition);

        let (tokens, _)= scan(proposition);

        let expr = parse(tokens);

        if let Err(_) = expr {
            panic!();
        }

        debug(expr.unwrap());
    }

    fn assert_err(proposition: &str) {
        println!("{}", proposition);

        let (tokens, _) = scan(proposition);

        if let Ok(expr) = parse(tokens) {
            debug(expr);
            panic!();
        }
    }

    #[test]
    fn lone_sentence() {
        assert_ok("p");
    }

    #[test]
    fn binary() {
        assert_ok("p & q");
    }

    #[test]
    fn binary_missing_left() {
        assert_err(" & q")
    }

    #[test]
    fn binary_missing_right() {
        assert_err("q &")
    }

    #[test]
    fn just_letters() {
        assert_err("pappasj & askj")
    }

    #[test]
    fn close_letters() {
        assert_err("pp | q & !qq | (pp)")
    }

    #[test]
    fn trinary() {
        assert_ok("p & q | s");
    }

    #[test]
    fn four() {
        assert_ok("p & q | s ~ t");
    }

    #[test]
    fn five() {
        assert_ok("p & q | s ~ t > m");
    }

    #[test]
    fn little_groups() {
        assert_ok("p & (q | s)");
    }

    #[test]
    fn groups() {
        assert_ok("p & (q | s ~ t > m) | (p & q | (s > u))");
    }

    #[test]
    fn negated_groups() {
        assert_ok("p & !(q | s ~ t > m) | !(p & q | !(s > u))");
    }

    #[test]
    fn negation() {
        assert_ok("!p & !!!j | !(p ~ !s)");
    }

    #[test]
    fn lots_parenthesis() {
        assert_ok("(p & (s) > ((l)) | ((((!q)))))");
    }

    #[test]
    fn not_matching_parenthesis_at_finish() {
        assert_err("(p & s  | !(s)");
    }

    #[test]
    fn not_matching_parenthesis_at_middle() {
        assert_err("(p & s  | !(s | (t))");
    }

    #[test]
    fn not_matching_parenthesis_at_first() {
        assert_err("p & s  | !(s) | (t))");
    }

    #[test]
    fn only_open() {
        assert_err("(((((p & q");
    }

    #[test]
    fn only_closing() {
        assert_err("p & q)))))");
    }

    #[test]
    fn nonsense_parenthesis() {
        assert_err("p &)(()) s  ))| !)s) | (t!!");
    }

    #[test]
    fn missing_left_side_in_group() {
        assert_err("(p | s) > (~ (p & q))");
    }

    #[test] 
    fn empty_groups() {
        assert_err("p & (()) s & q");
    }
    
    #[test]
    fn complicated_correct() {
        assert_ok("(p > ((!y & !s) | !(k ~ a)) > (o ~ (!p | p))) ~ l")
    }

    #[test]
    fn complicated_incorrect() {
        assert_err("(p > ((! & !s) | !(k ~ a)) > (~ (!p | p))) ~ l")
    }

    #[test]
    fn incoherent_negation() {
        assert_err("t !!! q && q")
    }

    #[test]
    fn correct_with_unvalid() {
        assert_err("p & q 99 a | o")
    }

    #[test]
    fn incorrect_with_unvalid() {
        assert_err("p & 9 s || a")
    }
}
