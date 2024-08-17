#[cfg(test)]

mod tests {
    use super::*;
    use crate::parse;
    use crate::ast_printer::print;
    use crate::grammar::Expr;

    fn debug(expr: Box<Expr>) {
        println!("{:?}", print(expr));
    }

    fn assert_ok(proposition: &str) {
        println!("{}", proposition);
        let expr = parse(proposition);
        if let Err(_) = expr {
            panic!();
        }
        debug(expr.unwrap());
    }

    fn assert_err(proposition: &str) {
        println!("{}", proposition);
        if let Ok(expr) = parse(proposition) {
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
}