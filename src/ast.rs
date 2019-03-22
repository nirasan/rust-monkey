use crate::token::Token;
use crate::token::Token::*;

pub trait Node: std::fmt::Debug {}

pub trait Statement: Node {}

pub trait Expression: Node {}

#[derive(Debug)]
pub struct Program {
    statements: Vec<Box<Statement>>
}

impl Program {
    pub fn new() -> Program {
        Program{
            statements: vec![]
        }
    }

    pub fn push(&mut self, statement: Box<Statement>) {
        self.statements.push(statement)
    }
}

impl Node for Program {}

#[derive(Debug)]
pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<Expression>
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Box<Expression>) -> LetStatement {
        LetStatement{ token, name, value }
    }
}

impl Node for LetStatement {}

impl Statement for LetStatement {}

#[derive(Debug)]
pub struct Identifier {
    token: Token,
    value: String,
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Identifier {
        Identifier{token, value}
    }
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