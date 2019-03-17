use std::io::{self, Write};
use crate::lexer::Lexer;
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
        loop {
            let token = lexer.token();
            io::stdout().write(format!("{:?}\n", token).as_bytes()).ok().expect("failed to write token");
            if token == Eof {
                break;
            }
        }
    }
}
