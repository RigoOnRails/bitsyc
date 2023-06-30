mod lexer;
use lexer::Lexer;

mod parser;
use parser::Parser;

fn main() {
    // TODO: Get input from file.
    let lexer = Lexer::new(String::from(""));
    let parser = Parser::new(lexer);
}
