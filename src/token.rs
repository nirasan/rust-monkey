#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(String),
    Eof,

    Ident(String),
    Int(i64),

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