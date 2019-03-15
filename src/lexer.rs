use crate::token::Token;
use crate::token::Token::*;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    next_position: usize,
    char: char
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let chars = input.chars().collect::<Vec<char>>();
        let first = chars[0].clone();

        let lexer = Lexer{
            input: chars,
            position: 0,
            next_position: 1,
            char: first,
        };
        return lexer;
    }

    fn token(&mut self) -> Token {
        let token = match self.char {
            '=' => Assign,
            ';' => SemiColon,
            '(' => LParen,
            ')' => RParen,
            '{' => LBrace,
            '}' => RBrace,
            ',' => Comma,
            '+' => Plus,
            '0' => Eof,
            c => Illegal(c.to_string()),
        };
        self.next();
        return token;
    }

    fn next(&mut self) {
        self.position += 1;
        self.next_position += 1;
        if self.position < self.input.len() {
            self.char = self.input[self.position].clone();
        } else {
            self.char = '0';
        }
    }
}

#[test]
fn test_next() {
    let input = "=+(){},;".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.token(), Assign);
    assert_eq!(lexer.token(), Plus);
    assert_eq!(lexer.token(), LParen);
    assert_eq!(lexer.token(), RParen);
    assert_eq!(lexer.token(), LBrace);
    assert_eq!(lexer.token(), RBrace);
    assert_eq!(lexer.token(), Comma);
    assert_eq!(lexer.token(), SemiColon);
    assert_eq!(lexer.token(), Eof);
}