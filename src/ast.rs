#[derive(Debug)]
pub enum Expr {
    BinOp {
        op: Operator,
        a: Box<Expr>,
        b: Box<Expr>,
    },
    Call {
        target: String,
        args: Vec<Expr>,
    },
    Num {
        val: f64,
    },
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}
