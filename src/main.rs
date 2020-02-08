mod ast;
mod interpreter;
mod lex;
mod parse;
mod test;
use interpreter::eval;
use parse::parse;

fn main() {
  println!("{:?}", eval(&parse("Sum(2 + 2, 2 + 2)").unwrap()));
}
