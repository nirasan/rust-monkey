use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{self, Write};

pub fn start() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .ok()
            .expect("failed to read line");

        if code == "exit" {
            break;
        }

        let lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        if parser.errors().len() > 0 {
            println!("ERROR: {:?}", parser.errors());
            continue;
        }

        let program = program.expect("failed to parse program");

        println!("PROGRAM: {:?}", program);
    }
}
