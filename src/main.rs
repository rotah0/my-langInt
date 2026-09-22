mod ast;
mod lexer;
mod parser;

use lexer::tokenize;
use parser::Parser;

fn main() {
    let input = "42";
    let tokens = tokenize(input).expect("ошибка лексера");
    let mut p = Parser::new(tokens);
    match p.parse() {
        Ok(expr) => println!("{:#?}", expr),
        Err(e)   => println!("Ошибка парсера: {}", e),
    }
}