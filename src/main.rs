mod ast;
mod interpreter;
mod lex;
mod parse;
mod test;
use interpreter::eval;
use parse::parse;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: rformula \"Cos(Pi() * 2)\"")
    } else {
        match run(args[1].as_str()) {
            Ok(res) => println!("{}", res),
            Err(msg) => println!("Error: {}", msg),
        }
    }
}

fn run(code: &str) -> Result<f64, String> {
    eval(&parse(code)?)
}
