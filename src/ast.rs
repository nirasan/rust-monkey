use crate::token::Token;
use crate::token::Token::*;

pub trait Node: std::fmt::Debug {}

pub trait Statement: Node {}

pub trait Expression: Node {}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<Statement>>
}

impl Node for Program {}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<Expression>
}

impl Node for LetStatement {}

impl Statement for LetStatement {}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {}

impl Expression for Identifier {}

#[derive(Debug)]
pub struct DummyExpression {}

impl Node for DummyExpression {}

impl Expression for DummyExpression {}

#[test]
fn test_new() {
    let l = LetStatement{
        token: Eq,
        name: Identifier{token: Eq, value: "five".to_string()},
        value: Box::new(Identifier{token: Eq, value: "ten".to_string()})
    };
    println!("{:?}", l);
    let p = Program{statements: vec![Box::new(l)]};
    println!("{:?}", p);
}