#[cfg(test)]

mod tests {
    #[test]
    fn simple() {
        crate::_test("p");
    }

    #[test]
    fn binary() {
        crate::_test("p & q");
    }

    #[test]
    fn wtf() {
        crate::_test("t |true |1 |i |0 |p |i |l |false |1");
    }
}
