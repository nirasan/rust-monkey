#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod token;
pub mod lexer;
pub mod repl;
pub mod ast;
pub mod parser;