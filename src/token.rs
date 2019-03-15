use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(String),
    Eof,

    Ident(String),
    Int(String),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LT,
    GT,

    Comma,
    SemiColon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Eq,
    NotEq,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl FromStr for Token {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "=" => Ok(Token::Assign),
            "+" => Ok(Token::Plus),
            "-" => Ok(Token::Minus),
            "!" => Ok(Token::Bang),
            "*" => Ok(Token::Asterisk),
            "/" => Ok(Token::Slash),
            "<" => Ok(Token::LT),
            ">" => Ok(Token::GT),
            "," => Ok(Token::Comma),
            ";" => Ok(Token::SemiColon),
            "(" => Ok(Token::LParen),
            ")" => Ok(Token::RParen),
            "{" => Ok(Token::LBrace),
            "}" => Ok(Token::RBrace),
            "==" => Ok(Token::Eq),
            "!=" => Ok(Token::NotEq),
            "fn" => Ok(Token::Function),
            "let" => Ok(Token::Let),
            "true" => Ok(Token::True),
            "false" => Ok(Token::False),
            "if" => Ok(Token::If),
            "else" => Ok(Token::Else),
            "return" => Ok(Token::Return),
            _ => Err(()),
        }
    }
}