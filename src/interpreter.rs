use crate::ast::{Expr, Operator};

pub fn eval(expr: &Expr) -> f64 {
  return match expr {
    Expr::Call { args, target } =>
    // TODO actually define fns. for now lets just sum all args i guess
    {
      match target.as_str() {
        "Pi" => std::f64::consts::PI,
        "Sin" => eval(&args[0]).sin(),
        "Cos" => eval(&args[0]).cos(),
        "Tan" => eval(&args[0]).tan(),
        "Sum" => sum(&args),
        _ => panic!("unknown function: {}", target),
      }
    }

    Expr::Num { val } => *val,

    Expr::BinOp { op, a, b } => match op {
      Operator::Plus => eval(a) + eval(b),
      Operator::Minus => eval(a) - eval(b),
      Operator::Multiply => eval(a) * eval(b),
      Operator::Divide => eval(a) / eval(b),
    },
  };
}

fn sum(args: &Vec<Expr>) -> f64 {
  let mut sum: f64 = 0.0;
  for arg in args {
    sum += eval(arg);
  }
  return sum;
}
