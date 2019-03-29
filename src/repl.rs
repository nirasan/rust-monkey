use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::Token::Eof;

pub fn start() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut code = String::new();
        io::stdin().read_line(&mut code).ok().expect("failed to read line");

        if code == "exit" {
            break;
        }

        let mut lexer = Lexer::new(code);
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
