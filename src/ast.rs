use crate::token::Token;

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
pub struct ReturnStatement {
    token: Token,
    return_value: Box<Expression>
}

impl ReturnStatement {
    pub fn new(token: Token, return_value: Box<Expression>) -> ReturnStatement {
        ReturnStatement{token, return_value}
    }
}

impl Node for ReturnStatement {}

impl Statement for ReturnStatement {}

#[derive(Debug)]
pub struct ExpressionStatement {
    token: Token,
    expression: Box<Expression>
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Box<Expression>) -> ExpressionStatement {
        ExpressionStatement{ token, expression }
    }
}

impl Node for ExpressionStatement {}

impl Statement for ExpressionStatement {}

#[derive(Debug)]
pub struct IntegerLiteral {
    token: Token,
    value: i64
}

impl IntegerLiteral {
    pub fn new(token: Token, value: i64) -> IntegerLiteral {
        IntegerLiteral{ token, value }
    }
}

impl Node for IntegerLiteral {}

impl Expression for IntegerLiteral {}


#[derive(Debug)]
pub struct PrefixExpression {
    token: Token,
    operator: String,
    right: Box<Expression>,
}

impl PrefixExpression {
    pub fn new(token: Token, operator: String, right: Box<Expression>) -> PrefixExpression {
        PrefixExpression{ token, operator, right }
    }
}

impl Node for PrefixExpression {}

impl Expression for PrefixExpression {}


#[derive(Debug)]
pub struct InfixExpression {
    token: Token,
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>
}

impl InfixExpression {
    pub fn new(token: Token, left: Box<Expression>, operator: String, right: Box<Expression>) -> InfixExpression {
        InfixExpression{ token, left, operator, right }
    }
}

impl Node for InfixExpression {}

impl Expression for InfixExpression {}


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