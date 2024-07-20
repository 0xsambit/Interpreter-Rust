mod ast;
mod interpreter;
mod lexer;
mod parser;
mod tests;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::io::{self, Write};

fn main() {
    let mut interpreter = Interpreter::new();
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }

        let lexer = Lexer::new(input.clone());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        interpreter.interpret(program);
    }
}
