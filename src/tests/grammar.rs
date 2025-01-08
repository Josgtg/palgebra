#[cfg(test)]

mod tests {
    use crate::grammar::Expr;
    use crate::scanner::scan;
    use crate::parser::parse;

    fn generate_expression(proposition: &str) -> Expr {
        parse(scan(proposition, 0).unwrap(), 0).unwrap()
    }

    #[test]
    fn same_base() {
        let left = generate_expression("p");
        let right = generate_expression("p");
        println!("{} == {}", &left.unparenthesized(), &right);
        println!("{:?} == {:?}", &left.unparenthesized(), &right);
        assert!(left.is_same(&right))
    }

    #[test]
    fn same_parenthesized() {
        let left = generate_expression("(((((p)))))");
        let right = generate_expression("p");
        println!("{} == {}", &left.unparenthesized(), &right);
        println!("{:?} == {:?}", &left.unparenthesized(), &right);
        assert!(left.is_same(&right))
    }

    #[test]
    fn same_binary_basic() {
        let left = generate_expression("p & q");
        let right = generate_expression("q & p");
        println!("{} == {}", &left.unparenthesized(), &right);
        println!("{:?} == {:?}", &left.unparenthesized(), &right);
        assert!(left.is_same(&right))
    }

    #[test]
    fn same_binary_parenthesized() {
        let left = generate_expression("(((((p & (q))))))");
        let right = generate_expression("p & q");
        println!("{} == {}", &left.unparenthesized(), &right);
        println!("{:?} == {:?}", &left.unparenthesized(), &right);
        assert!(left.is_same(&right))
    }

    #[test]
    fn same_binary_chain() {
        // FIXME: Does not evaluate as the same (It's hard 😢, implementation is pending)
        let _left = generate_expression("p & (s & q)");
        let _right = generate_expression("p & q & s");
        let left = generate_expression("(p & (p & q & r & s))");
        let right = generate_expression("p & (p & q & r & s)");
        println!("{} == {}", &left.unparenthesized(), &right);
        println!("{:?} == {:?}", &left.unparenthesized(), &right);
        assert!(left.is_same(&right))
    }

    #[test]
    fn same_complex() {
        let left = generate_expression("(p & (((q) | (!s & r))))");
        let right = generate_expression("p & (q | (!s & r))");
        println!("{} == {}", &left.unparenthesized(), &right);
        println!("{:?} == {:?}", &left.unparenthesized(), &right);
        assert!(left.is_same(&right))
    }
}