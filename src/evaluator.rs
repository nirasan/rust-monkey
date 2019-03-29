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
        Node::PrefixExpression { token: _, operator: operator, right: right} => eval_prefix_expression(operator, right),
        Node::InfixExpression { token: _, left: left, operator: operator, right: right} => eval_infix_expression(left, operator, right),
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

fn eval_prefix_expression(operator: &str, right: &Box<Node>) -> Option<Object> {
    let right = eval(right)?;
    match operator {
        "!" => Some(eval_bang_operator_expression(right)),
        "-" => Some(eval_minus_prefix_operator_expression(right)),
        _ => Some(Object::Null)
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        Object::Bool(b) => native_bool_to_bool_object(!b),
        Object::Null => object::TRUE,
        _ => object::FALSE
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    if let Object::Integer(i) = right {
        Object::Integer(-i)
    } else {
        Object::Null
    }
}

fn eval_infix_expression(left: &Box<Node>, operator: &str, right: &Box<Node>) -> Option<Object> {
    let left = eval(left)?;
    let right = eval(right)?;

    if left.is_integer() && right.is_integer() {
        if let Object::Integer(n) = left {
            if let Object::Integer(m) = right {
                return Some(eval_integer_infix_expression(operator, n, m));
            }
        }
    }

    Some(match operator {
        "==" => native_bool_to_bool_object(left == right),
        "!=" => native_bool_to_bool_object(left != right),
        _ => Object::Null
    })
}

fn eval_integer_infix_expression(op: &str, left: i64, right: i64) -> Object {
    match op {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        "<" => native_bool_to_bool_object(left < right),
        ">" => native_bool_to_bool_object(left > right),
        "==" => native_bool_to_bool_object(left == right),
        "!=" => native_bool_to_bool_object(left != right),
        _ => Object::Null
    }
}
