#[cfg(test)]

mod tests {

    use crate::scanner::scan;
    use crate::token::Token;

    #[allow(warnings)]
    fn print_tokens(tokens: (Vec<Token>, Vec<char>, bool)) {
        println!("{:?}", tokens.1);
    }

    fn assert_ok(proposition: &str) {
        println!("{}", proposition);
        let tokens = scan(proposition, 1);
        if let Ok(tokens) = tokens {
            println!("{:?}", tokens);
        } else {
            panic!()
        }
    }

    fn assert_err(proposition: &str) {
        println!("{}", proposition);
        let tokens = scan(proposition, 1);
        if let Ok(tokens) = tokens {
            println!("{:?}", tokens);
            panic!();
        }
    }

    #[test]
    fn comment() {
        assert_ok("p & q | !s // ajshajsakshkkj.-..{.{");
    }

    #[test]
    fn comment_with_new_line() {
        assert_ok("p & q | !s // ajshajsakshkkj.-..{.{\np & false");
    }

    #[test]
    fn true_kw() {
        assert_ok("true & q | !s");
    }

    #[test]
    fn false_kw() {
        assert_ok("p & false | !s");
    }

    #[test]
    fn new_line() {
        assert_ok("p & !false\nq > s")
    }

    #[test]
    fn err_at_new_line() {
        assert_err("p & !false\nq > s}")
    }

    #[test]
    fn numbers() {
        assert_ok("false & !1\n0 > 1")
    }

    #[test]
    fn err_indexes() {
        assert_err("}.- }.- }.\n-{  {.");
    }
}
