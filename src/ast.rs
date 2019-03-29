use crate::token::Token;
use core::borrow::Borrow;

#[derive(Debug, Clone)]
pub enum Node {
    Statement {
        node: Box<Node>,
    },
    Expression {
        node: Box<Node>,
    },

    Program {
        statements: Vec<Box<Node>>,
    },

    // Statement
    LetStatement {
        token: Token,
        name: Box<Node>,
        value: Box<Node>,
    }, // name is Identifier, value is Expression
    ReturnStatement {
        token: Token,
        return_value: Box<Node>,
    },
    ExpressionStatement {
        token: Token,
        expression: Box<Node>,
    },
    BlockStatement {
        token: Token,
        statements: Vec<Box<Node>>,
    },

    // Expression
    Identifier {
        token: Token,
        value: String,
    },
    IntegerLiteral {
        token: Token,
        value: i64,
    },
    PrefixExpression {
        token: Token,
        operator: String,
        right: Box<Node>,
    },
    InfixExpression {
        token: Token,
        left: Box<Node>,
        operator: String,
        right: Box<Node>,
    },
    Boolean {
        token: Token,
        value: bool,
    },
    IfExpression {
        token: Token,
        condition: Box<Node>,
        consequence: Box<Node>,
        alternative: Option<Box<Node>>,
    },
    FunctionLiteral {
        token: Token,
        parameters: Vec<Box<Node>>,
        body: Box<Node>,
    },
    CallExpression {
        token: Token,
        function: Box<Node>,
        arguments: Vec<Box<Node>>,
    },
}

impl Node {
    pub fn new_statement(node: Box<Node>) -> Option<Box<Node>> {
        if Node::is_statement(node.borrow()) {
            return Some(node);
        }
        let node = match node.borrow() {
            &Node::LetStatement {
                token: _,
                name: _,
                value: _,
            } => Some(node),
            &Node::ReturnStatement {
                token: _,
                return_value: _,
            } => Some(node),
            &Node::ExpressionStatement {
                token: _,
                expression: _,
            } => Some(node),
            &Node::BlockStatement {
                token: _,
                statements: _,
            } => Some(node),
            _ => None,
        };
        return node.and_then(|node| Some(Box::new(Node::Statement { node })));
    }

    pub fn new_expression(node: Box<Node>) -> Option<Box<Node>> {
        eprintln!("[new_expression 1] node is {:?}", node);
        if Node::is_expression(node.borrow()) {
            return Some(node);
        }
        let node = match node.borrow() {
            &Node::Identifier { token: _, value: _ } => Some(node),
            &Node::IntegerLiteral { token: _, value: _ } => Some(node),
            &Node::PrefixExpression {
                token: _,
                operator: _,
                right: _,
            } => Some(node),
            &Node::InfixExpression {
                token: _,
                left: _,
                operator: _,
                right: _,
            } => Some(node),
            &Node::Boolean { token: _, value: _ } => Some(node),
            &Node::IfExpression {
                token: _,
                condition: _,
                consequence: _,
                alternative: _,
            } => Some(node),
            &Node::FunctionLiteral {
                token: _,
                parameters: _,
                body: _,
            } => Some(node),
            &Node::CallExpression {
                token: _,
                function: _,
                arguments: _,
            } => Some(node),
            &Node::Expression { node: _ } => Some(node),
            _ => None,
        };
        eprintln!("[new_expression 2] node is {:?}", node);
        return node.and_then(|node| Some(Box::new(Node::Expression { node })));
    }

    pub fn new_program(statements: Vec<Box<Node>>) -> Option<Box<Node>> {
        for s in statements.iter() {
            if !Node::is_statement(s.borrow()) {
                return None;
            }
        }

        return Some(Box::new(Node::Program { statements }));
    }

    pub fn new_let_statement(token: Token, name: Box<Node>, value: Box<Node>) -> Option<Box<Node>> {
        if !Node::is_identifier(name.borrow()) {
            return None;
        }

        if !Node::is_expression(value.borrow()) {
            return None;
        }

        return Some(Box::new(Node::LetStatement { token, name, value }));
    }

    pub fn new_return_statement(token: Token, return_value: Box<Node>) -> Option<Box<Node>> {
        if !Node::is_expression(return_value.borrow()) {
            return None;
        }

        return Some(Box::new(Node::ReturnStatement {
            token,
            return_value,
        }));
    }

    pub fn new_expression_statement(token: Token, expression: Box<Node>) -> Option<Box<Node>> {
        if !Node::is_expression(expression.borrow()) {
            eprintln!("{:?} is not expression", expression);
            return None;
        }

        return Some(Box::new(Node::ExpressionStatement { token, expression }));
    }

    pub fn new_block_statement(token: Token, statements: Vec<Box<Node>>) -> Option<Box<Node>> {
        for s in statements.iter() {
            if !Node::is_statement(s.borrow()) {
                return None;
            }
        }

        return Some(Box::new(Node::BlockStatement { token, statements }));
    }

    pub fn new_identifier(token: Token, value: String) -> Box<Node> {
        Box::new(Node::Identifier { token, value })
    }

    pub fn new_integer_literal(token: Token, value: i64) -> Box<Node> {
        Box::new(Node::IntegerLiteral { token, value })
    }

    pub fn new_prefix_expression(
        token: Token,
        operator: String,
        right: Box<Node>,
    ) -> Option<Box<Node>> {
        if !Node::is_expression(right.borrow()) {
            return None;
        }

        Some(Box::new(Node::PrefixExpression {
            token,
            operator,
            right,
        }))
    }

    pub fn new_infix_expression(
        token: Token,
        left: Box<Node>,
        operator: String,
        right: Box<Node>,
    ) -> Option<Box<Node>> {
        if !Node::is_expression(left.borrow()) {
            eprintln!("left is not expression. {:?}", left);
            return None;
        }

        if !Node::is_expression(right.borrow()) {
            eprintln!("right is not expression. {:?}", right);
            return None;
        }

        Some(Box::new(Node::InfixExpression {
            token,
            left,
            operator,
            right,
        }))
    }

    pub fn new_boolean(token: Token, value: bool) -> Box<Node> {
        Box::new(Node::Boolean { token, value })
    }

    pub fn new_if_expression(
        token: Token,
        condition: Box<Node>,
        consequence: Box<Node>,
        alternative: Option<Box<Node>>,
    ) -> Option<Box<Node>> {
        if !Node::is_expression(condition.borrow()) {
            return None;
        }

        if !Node::is_block_statement(consequence.borrow()) {
            return None;
        }

        if alternative.is_some()
            && !Node::is_block_statement(alternative.as_ref().unwrap().borrow())
        {
            return None;
        }

        Some(Box::new(Node::IfExpression {
            token,
            condition,
            consequence,
            alternative,
        }))
    }

    pub fn new_function_literal(
        token: Token,
        parameters: Vec<Box<Node>>,
        body: Box<Node>,
    ) -> Option<Box<Node>> {
        for p in parameters.iter() {
            if !Node::is_identifier(p.borrow()) {
                return None;
            }
        }

        if !Node::is_block_statement(body.borrow()) {
            return None;
        }

        Some(Box::new(Node::FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    pub fn new_call_expression(
        token: Token,
        function: Box<Node>,
        arguments: Vec<Box<Node>>,
    ) -> Option<Box<Node>> {
        if !Node::is_expression(function.borrow()) {
            return None;
        }

        for a in arguments.iter() {
            if !Node::is_expression(a.borrow()) {
                return None;
            }
        }

        Some(Box::new(Node::CallExpression {
            token,
            function,
            arguments,
        }))
    }

    fn is_statement(node: &Node) -> bool {
        match node {
            &Node::Statement { node: _ } => true,
            _ => false,
        }
    }

    fn is_expression(node: &Node) -> bool {
        match node {
            &Node::Expression { node: _ } => true,
            _ => false,
        }
    }

    fn is_identifier(node: &Node) -> bool {
        match node {
            &Node::Identifier { token: _, value: _ } => true,
            _ => false,
        }
    }

    fn is_block_statement(node: &Node) -> bool {
        match node {
            &Node::BlockStatement {
                token: _,
                statements: _,
            } => true,
            _ => false,
        }
    }
}

#[test]
fn test_new_let_statement() {
    let i = Node::new_identifier(Token::Ident("".to_string()), "".to_string());
    let ii = Node::new_identifier(Token::Ident("".to_string()), "".to_string());
    let ii = Node::new_expression(ii);
    let l = Node::new_let_statement(Token::Let, i, ii.unwrap());
    println!("{:?}", l);
}

#[test]
fn test_is_identifier() {
    let i = Node::new_identifier(Token::Ident("".to_string()), "".to_string());
    println!("{:?}", Node::is_identifier(i.borrow()));
}
