#[cfg(test)]
mod tests {
    use crate::interpreter::eval;
    use crate::parse::parse;

    // https://stackoverflow.com/questions/30856285/assert-eq-with-floating-point-numbers-and-delta
    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }

    fn parse_and_eval(code: &str, expected: f64) {
        assert_delta!(eval(parse(code).unwrap()).unwrap(), expected, 1e-6);
    }

    #[test]
    fn bin_op() {
        parse_and_eval("2 + 2", 4.0);
        parse_and_eval("2 - 3", -1.0);
        parse_and_eval("2 * 3", 6.0);
        parse_and_eval("2 / 2", 1.0);
        parse_and_eval("(2 + 2)", 4.0);
        parse_and_eval("2 + 3 - 6", -1.0);
        parse_and_eval("((2 + 3) - 6)", -1.0);
        parse_and_eval("(2 + (3 - 6))", -1.0);
        parse_and_eval("(2 - (3 - 6))", 5.0);
    }

    #[test]
    fn fns() {
        parse_and_eval("Sum()", 0.0);
        parse_and_eval("Sum(1)", 1.0);
        parse_and_eval("Sum(1,2)", 3.0);
        parse_and_eval("Sum(1,2,3)", 6.0);

        parse_and_eval("Sin(0)", 0.0);
        parse_and_eval("Sin(Pi())", 0.0);
        parse_and_eval("Cos(0)", 1.0);
        parse_and_eval("Cos(Pi())", -1.0);
    }
}
