use crate::ast::{Expr, Operator};

pub fn eval(expr: &Expr) -> Result<f64, String> {
  return match expr {
    Expr::Call { args, target } => {
      let mut arg_results = vec![];
      for arg in args {
        arg_results.push(eval(arg)?);
      }

      let res = match target.as_str() {
        "Pi" => std::f64::consts::PI,
        "Sin" => arg_results[0].sin(), // how to index a vec or return Err in a not verbose way here?
        "Cos" => arg_results[0].cos(),
        "Tan" => arg_results[0].tan(),
        "Sum" => sum(&arg_results),
        _ => return Err(format!("unknown function: {}", target)),
      };

      return Ok(res);
    }

    Expr::Num { val } => Ok(*val),

    Expr::BinOp {
      op,
      a: a_node,
      b: b_node,
    } => {
      let a = eval(a_node)?;
      let b = eval(b_node)?;
      let res = match op {
        Operator::Plus => a + b,
        Operator::Minus => a - b,
        Operator::Multiply => a * b,
        Operator::Divide => a / b,
      };
      return Ok(res);
    }
  };
}

fn sum(args: &Vec<f64>) -> f64 {
  let mut sum: f64 = 0.0;
  for arg in args {
    sum += arg;
  }
  return sum;
}
