use crate::ast::Node;
use crate::object;
use crate::object::Object;
use crate::environment::Environment;
use core::borrow::Borrow;

pub fn eval(node: &Box<Node>, env: &mut Environment) -> Option<Object> {
    let n = node.borrow();
    println!("NODE: {:?}", n);
    match n {
        Node::Program {statements: statements} => eval_program(statements, env),
        Node::Statement {node: node} => eval(node, env),
        Node::Expression {node: node} => eval(node, env),
        Node::ExpressionStatement {token: _, expression: expression} => eval(expression, env),
        Node::IntegerLiteral {token: _, value: value} => Some(Object::Integer(*value)),
        Node::Boolean {token: _, value: value} => Some(native_bool_to_bool_object(*value)),
        Node::PrefixExpression { token: _, operator: operator, right: right} => eval_prefix_expression(operator, right, env),
        Node::InfixExpression { token: _, left: left, operator: operator, right: right} => eval_infix_expression(left, operator, right, env),
        Node::BlockStatement {token: _, statements: statements} => eval_block_statements(statements, env),
        Node::IfExpression { token: _, condition: condition, consequence: consequence, alternative: alternative} => eval_if_expression(condition, consequence, alternative, env),
        Node::ReturnStatement { token: _, return_value: return_value } => eval_return_statement(return_value, env),
        Node::LetStatement { token: _, name: name, value: value } => eval_let_statement(name, value, env),
        Node::Identifier { token: _, value: value} => eval_identifier(value, env),
        Node::FunctionLiteral { token: _, parameters: parameters, body: body } => eval_function_literal(parameters, body, env),
        Node::CallExpression { token: _, function: function, arguments: arguments } => eval_call_expression(function, arguments, env),
    }
}

fn eval_program(nodes: &Vec<Box<Node>>, env: &mut Environment) -> Option<Object> {
    let mut result = None;

    for node in nodes.iter() {
        let r = eval(node, env)?;
        if let Object::ReturnValue(v) = r {
            return Some(*v);
        } else if let Object::Error(_) = r {
            return Some(r);
        }
        result = Some(r);
    }

    return result;
}

fn eval_block_statements(nodes: &Vec<Box<Node>>, env: &mut Environment) -> Option<Object> {
    let mut result = None;

    for node in nodes.iter() {
        let r = eval(node, env)?;

        if r.is_same(&Object::ReturnValue(Box::new(Object::Null))) || r.is_same(&Object::Error(String::new())) {
            return Some(r);
        }

        result = Some(r);
    }

    return result;
}

fn native_bool_to_bool_object(b: bool) -> Object {
    if b { object::TRUE } else { object::FALSE }
}

fn eval_prefix_expression(operator: &str, right: &Box<Node>, env: &mut Environment) -> Option<Object> {
    let right = eval(right, env)?;
    if right.is_error() {
        return Some(right);
    }
    match operator {
        "!" => Some(eval_bang_operator_expression(right)),
        "-" => Some(eval_minus_prefix_operator_expression(right)),
        _ => Some(Object::Error(format!("unknown operator: {}, {:?}", operator, right)))
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
        Object::Error(format!("unknown object: {:?}", right))
    }
}

fn eval_infix_expression(left: &Box<Node>, operator: &str, right: &Box<Node>, env: &mut Environment) -> Option<Object> {
    let left = eval(left, env)?;
    if left.is_error() {
        return Some(left);
    }

    let right = eval(right, env)?;
    if right.is_error() {
        return Some(right);
    }

    if left.is_integer() && right.is_integer() {
        if let Object::Integer(n) = left {
            if let Object::Integer(m) = right {
                return Some(eval_integer_infix_expression(operator, n, m));
            }
        }
    }

    if !left.is_same(&right) {
        return Some(Object::Error(format!("type mismatch: {:?}, {:?}, {:?}", left, operator, right)));
    }

    Some(match operator {
        "==" => native_bool_to_bool_object(left == right),
        "!=" => native_bool_to_bool_object(left != right),
        _ => Object::Error(format!("unknown operator: {:?}, {:?}, {:?}", left, operator, right))
    })
}

fn eval_integer_infix_expression(operator: &str, left: i64, right: i64) -> Object {
    match operator {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        "<" => native_bool_to_bool_object(left < right),
        ">" => native_bool_to_bool_object(left > right),
        "==" => native_bool_to_bool_object(left == right),
        "!=" => native_bool_to_bool_object(left != right),
        _ => Object::Error(format!("unknown operator: {:?}, {:?}, {:?}", left, operator, right))
    }
}

fn eval_if_expression(condition: &Box<Node>, consequence: &Box<Node>, alternative: &Option<Box<Node>>, env: &mut Environment) -> Option<Object> {
    let condition = eval(condition, env)?;
    if condition.is_error() {
        return Some(condition);
    }

    if is_truthy(condition) {
        return eval(consequence, env);
    } else if alternative.is_some() {
        let alternative = alternative.as_ref().unwrap();
        return eval(alternative, env);
    } else {
        return Some(Object::Null);
    }
}

fn is_truthy(object: Object) -> bool {
    match object {
        Object::Null => false,
        Object::Bool(true) => true,
        Object::Bool(false) => false,
        _ => true,
    }
}

fn eval_return_statement(return_value: &Box<Node>, env: &mut Environment) -> Option<Object> {
    let val = eval(return_value, env)?;
    if val.is_error() {
        return Some(val);
    }
    return Some(Object::ReturnValue(Box::new(val)));
}

fn eval_let_statement(name: &Box<Node>, value: &Box<Node>, env: &mut Environment) -> Option<Object> {
    let val = eval(value, env)?;
    if val.is_error() {
        return Some(val);
    }

    match name.borrow() {
        Node::Identifier { token:_, value: v } => env.set(v.to_owned(), val),
        _ => Some(Object::Error(format!("invalid identifier: {:?}", name)))
    }
}

fn eval_identifier(value: &String, env: &mut Environment) -> Option<Object> {
    let object = env.get(value);
    if object.is_none() {
        return Some(Object::Error(format!("identifier not found: {:?}", value)));
    }
    return object;
}


fn eval_function_literal(parameters: &Vec<Box<Node>>, body: &Box<Node>, env: &mut Environment) -> Option<Object> {
    Some(Object::Function {
        parameters: parameters.clone(),
        body: body.clone(),
        environment: env.clone(),
    })
}

fn eval_call_expression(function: &Box<Node>, arguments: &Vec<Box<Node>>, env: &mut Environment) -> Option<Object> {
    let function = eval(function, env)?;
    if function.is_error() {
        return Some(function);
    }

    let arguments = eval_expression(arguments, env)?;
    if arguments.len() == 1 {
        if let Object::Error(e) = &arguments[0] {
            return Some(Object::Error(e.to_string()));
        }
    }

    return apply_function(function, arguments);
}

fn eval_expression(expressions: &Vec<Box<Node>>, env: &mut Environment) -> Option<Vec<Object>> {
    let mut results = vec![];

    for expression in expressions.iter() {
        let result = eval(expression, env)?;
        if result.is_error() {
            return None;
        }
        results.push(result);
    }

    return Some(results);
}

fn apply_function(function: Object, argument: Vec<Object>) -> Option<Object> {
    if let Object::Function { parameters: p, body: b, environment: e} = function {
        let mut extended_env = Environment::new_enclosed(e);
        for (i, v) in p.iter().enumerate() {
            let v = v.borrow();
            if let Node::Identifier { token: _, value: key } = v {
                let a = argument.get(i);
                if a.is_some() {
                    let a = a.unwrap();
                    extended_env.set(key.to_owned(), a.to_owned());
                }
            }
        }

        println!("[apply_function] extended_env is {:?}", extended_env);
        println!("[apply_function] body is {:?}", b);

        let result = eval(&b, &mut extended_env);

        if let Some(Object::ReturnValue(v)) = result {
            return Some(*v);
        }
        return result;
    }

    return None;
}