use crate::ast::{Expr, Operator};

pub fn eval(expr: Expr) -> Result<f64, String> {
    match expr {
        Expr::Call { args, target } => {
            let arg_results = args.into_iter().map(eval).collect::<Result<Vec<_>, _>>()?;

            let res = match target.as_str() {
                "Pi" => std::f64::consts::PI,
                "Sin" => arg_results[0].sin(), // how to index a vec or return Err in a not verbose way here?
                "Cos" => arg_results[0].cos(),
                "Tan" => arg_results[0].tan(),
                "Sum" => sum(&arg_results),
                _ => return Err(format!("unknown function: {}", target)),
            };

            Ok(res)
        }

        Expr::Num { val } => Ok(val),

        Expr::BinOp {
            op,
            a: a_node,
            b: b_node,
        } => {
            let a = eval(*a_node)?;
            let b = eval(*b_node)?;
            let res = match op {
                Operator::Plus => a + b,
                Operator::Minus => a - b,
                Operator::Multiply => a * b,
                Operator::Divide => a / b,
            };
            Ok(res)
        }
    }
}

fn sum(args: &[f64]) -> f64 {
    let mut sum: f64 = 0.0;
    for arg in args {
        sum += arg;
    }
    sum
}
