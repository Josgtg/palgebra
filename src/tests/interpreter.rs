#[cfg(test)]

mod tests {
    use crate::scanner::scan;
    use crate::parser::parse;
    use crate::interpreter::interpret;
    use crate::tests::ast_printer::print_ast;

    fn test(proposition: &str, goal: bool) {
        println!("{}", proposition);
        let tokens = scan(proposition, 1).unwrap();
        println!("{:?}", tokens);
        let expr = parse(tokens, 1);
        println!("{}", print_ast(expr.clone().unwrap()));
        let res = interpret(expr.unwrap());

        assert_eq!(res, goal);
    }

    #[test]
    fn v() {
        test("true", true);
    }

    #[test]
    fn f() {
        test("false", false);
    }

    #[test]
    fn and() {
        test("true & true", true);
        test("true & false", false);
    }

    #[test]
    fn or() {
        test("true | false", true);
        test("false | false", false);
    }

    #[test]
    fn long() {
        test("((0 | 1) ~ ((true & false) > true) ~ true) | false", true);
    }
}