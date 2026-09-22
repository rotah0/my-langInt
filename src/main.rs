mod ast;
mod eval;     
mod lexer;
mod parser;

use eval::eval;
use lexer::tokenize;
use parser::Parser;

fn main() {
    let input = "2 + 3 * 4";
    let tokens = tokenize(input).expect("ошибка лексера");
    let mut p = Parser::new(tokens);
    let expr = match p.parse() {
        Ok(e) => e,
        Err(e) => {
            println!("Ошибка парсера: {}", e);
            return;
        }
    };
    println!("Дерево: {:#?}", expr);

    match eval(&expr) {
        Ok(result) => println!("Результат: {}", result),
        Err(e)     => println!("Ошибка вычисления: {}", e),
    }
}