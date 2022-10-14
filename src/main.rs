pub mod lexer;
pub mod parser;
use std::env;

fn main() {
    let input = env::args().nth(1).unwrap_or_else(|| panic!("No input provided"));
    let output = parser::parse(input.as_str());
    println!("{} = {}", input, output.eval());
}
