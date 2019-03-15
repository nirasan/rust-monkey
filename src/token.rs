use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(String),
    Eof,

    Ident(String),
    Int(String),

    Assign,
    Plus,

    Comma,
    SemiColon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
}

impl FromStr for Token {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "=" => Ok(Token::Assign),
            "+" => Ok(Token::Plus),
            "," => Ok(Token::Comma),
            ";" => Ok(Token::SemiColon),
            "(" => Ok(Token::LParen),
            ")" => Ok(Token::RParen),
            "{" => Ok(Token::LBrace),
            "}" => Ok(Token::RBrace),
            "fn" => Ok(Token::Function),
            "let" => Ok(Token::Let),
            _ => Err(()),
        }
    }
}