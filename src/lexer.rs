use crate::token::Token;
use crate::token::Token::*;

use std::iter::FromIterator;
use std::str::FromStr;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    char: Option<char>
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let chars = input.chars().collect::<Vec<char>>();
        let first = chars.get(0).and_then(|c| Some(*c));

        let lexer = Lexer{
            input: chars,
            position: 0,
            char: first,
        };
        return lexer;
    }

    fn token(&mut self) -> Token {
        self.skip_whitespace();
        let token = if let Some(c) = self.char {
            if Lexer::is_letter(c) {
                let ident = self.read_identifier();
                let reserved = Token::from_str(ident.as_str());
                if reserved.is_ok() {
                    reserved.unwrap()
                } else {
                    Ident(ident)
                }
            } else if Lexer::is_digit(c) {
                let number = self.read_number();
                Int(number)
            } else {
                let token = match c {
                    '=' => Assign,
                    ';' => SemiColon,
                    '(' => LParen,
                    ')' => RParen,
                    '{' => LBrace,
                    '}' => RBrace,
                    ',' => Comma,
                    '+' => Plus,
                    c => Illegal(c.to_string())
                };
                self.next();
                token
            }
        } else {
            Eof
        };
        return token;
    }

    fn next(&mut self) {
        self.position += 1;
        self.char = self.input.get(self.position).and_then(|c| Some(*c));
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while Lexer::is_letter(self.char.unwrap_or_default()) {
            self.next();
        }
        return String::from_iter(&self.input[start .. self.position]);
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        while Lexer::is_digit(self.char.unwrap_or_default()) {
            self.next();
        }
        return String::from_iter(&self.input[start .. self.position]);
    }

    fn is_letter(c: char) -> bool {
        return c.is_ascii_alphabetic() || c == '_';
    }

    fn is_digit(c: char) -> bool {
        return c.is_ascii_digit();
    }

    fn skip_whitespace(&mut self) {
        while self.char.unwrap_or_default().is_whitespace() {
            self.next();
        }
    }
}

#[test]
fn test_next_symbol() {
    let input = r#"=+(){},;"#.to_string();
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

#[test]
fn test_next_let() {
    let input = r#"let five = 5;"#.to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.token(), Let);
    assert_eq!(lexer.token(), Ident("five".to_string()));
    assert_eq!(lexer.token(), Assign);
    assert_eq!(lexer.token(), Int("5".to_string()));
    assert_eq!(lexer.token(), SemiColon);
    assert_eq!(lexer.token(), Eof);
}

#[test]
fn test_next_function() {
    let input = r#"let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
"#.to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.token(), Let);
    assert_eq!(lexer.token(), Ident("add".to_string()));
    assert_eq!(lexer.token(), Assign);

    assert_eq!(lexer.token(), Function);

    assert_eq!(lexer.token(), LParen);
    assert_eq!(lexer.token(), Ident("x".to_string()));
    assert_eq!(lexer.token(), Comma);
    assert_eq!(lexer.token(), Ident("y".to_string()));
    assert_eq!(lexer.token(), RParen);

    assert_eq!(lexer.token(), LBrace);
    assert_eq!(lexer.token(), Ident("x".to_string()));
    assert_eq!(lexer.token(), Plus);
    assert_eq!(lexer.token(), Ident("y".to_string()));
    assert_eq!(lexer.token(), SemiColon);
    assert_eq!(lexer.token(), RBrace);
    assert_eq!(lexer.token(), SemiColon);

    assert_eq!(lexer.token(), Let);
    assert_eq!(lexer.token(), Ident("result".to_string()));
    assert_eq!(lexer.token(), Assign);

    assert_eq!(lexer.token(), Ident("add".to_string()));
    assert_eq!(lexer.token(), LParen);
    assert_eq!(lexer.token(), Ident("five".to_string()));
    assert_eq!(lexer.token(), Comma);
    assert_eq!(lexer.token(), Ident("ten".to_string()));
    assert_eq!(lexer.token(), RParen);
    assert_eq!(lexer.token(), SemiColon);

    assert_eq!(lexer.token(), Eof);
}

#[test]
fn test_read_identifier() {
    let input = r#"let five = 5;"#.to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.read_identifier(), "let".to_string());
    lexer.next();
    assert_eq!(lexer.read_identifier(), "five".to_string());
}