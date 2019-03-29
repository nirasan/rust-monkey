use crate::ast::Node;
use crate::object;
use crate::object::Object;
use core::borrow::Borrow;

pub fn eval(node: &Box<Node>) -> Option<Object> {
    let n = node.borrow();
    println!("NODE: {:?}", n);
    match n {
        Node::Program {statements: statements} => eval_statements(statements),
        Node::Statement {node: node} => eval(node),
        Node::Expression {node: node} => eval(node),
        Node::ExpressionStatement {token: _, expression: expression} => eval(expression),
        Node::IntegerLiteral {token: _, value: value} => Some(Object::Integer(*value)),
        Node::Boolean {token: _, value: value} => Some(native_bool_to_bool_object(*value)),
        _ => None
    }
}

fn eval_statements(nodes: &Vec<Box<Node>>) -> Option<Object> {
    let mut result = None;

    for node in nodes.iter() {
        result = eval(node);
    }

    return result;
}

fn native_bool_to_bool_object(b: bool) -> Object {
    if b { object::TRUE } else { object::FALSE }
}