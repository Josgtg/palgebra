#[cfg(test)]

mod tests {
    use crate::scanner::scan;
    use crate::parser::parse;
    use crate::interpreter::interpret;
    use crate::utils::replace_literals;

    #[test]
    fn simple() {
        crate::test("p");
    }

    #[test]
    fn binary() {
        crate::test("p & q");
    }

    #[test]
    fn wtf() {
        crate::test("t |true |1 |i |0 |p |i |l |false |1");
    }
}