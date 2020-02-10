mod ast;
mod interpreter;
mod lex;
mod parse;
mod test;

use interpreter::eval;
use parse::parse;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    match run(buffer.as_str()) {
        Ok(res) => println!("{}", res),
        Err(msg) => println!("Error: {}", msg),
    }

    Ok(()) // TODO exit code based on success
}

fn run(code: &str) -> Result<f64, String> {
    eval(parse(code)?)
}
