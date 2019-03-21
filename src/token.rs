use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Illegal(s) => write!(f, "Illegal({})", s),
            Token::Ident(s) => write!(f, "\"{}\"", s),
            Token::Int(s) => write!(f, "{}", s),
            Token::Eof => write!(f, "EOF"),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::LT => write!(f, "<"),
            Token::GT => write!(f, ">"),
            Token::Comma => write!(f, ","),
            Token::SemiColon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Function => write!(f, "function"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
        }
    }
}

#[test]
fn test_display() {
    assert_eq!(Token::Plus.to_string(), "+");
    assert_eq!(Token::LBrace.to_string(), "{");
    assert_eq!(Token::Ident("hello".to_string()).to_string(), "\"hello\"");
}