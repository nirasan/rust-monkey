use crate::ast::Node;
use crate::environment::Environment;
use crate::object;
use crate::object::{Object, HashPair};
use core::borrow::Borrow;
use std::rc::Rc;
use std::collections::HashMap;

pub fn eval(node: &Box<Node>, env: &mut Environment) -> Option<Rc<Object>> {
    let n = node.borrow();
    match n {
        Node::Program {
            statements: statements,
        } => eval_program(statements, env),
        Node::Statement { node: node } => eval(node, env),
        Node::Expression { node: node } => eval(node, env),
        Node::ExpressionStatement {
            token: _,
            expression: expression,
        } => eval(expression, env),
        Node::IntegerLiteral {
            token: _,
            value: value,
        } => Some(Rc::new(Object::Integer(*value))),
        Node::StringLiteral {
            token: _,
            value: value,
        } => Some(Rc::new(Object::StringValue(value.to_owned()))),
        Node::Boolean {
            token: _,
            value: value,
        } => Some(Rc::new(native_bool_to_bool_object(*value))),
        Node::PrefixExpression {
            token: _,
            operator: operator,
            right: right,
        } => eval_prefix_expression(operator, right, env),
        Node::InfixExpression {
            token: _,
            left: left,
            operator: operator,
            right: right,
        } => eval_infix_expression(left, operator, right, env),
        Node::BlockStatement {
            token: _,
            statements: statements,
        } => eval_block_statements(statements, env),
        Node::IfExpression {
            token: _,
            condition: condition,
            consequence: consequence,
            alternative: alternative,
        } => eval_if_expression(condition, consequence, alternative, env),
        Node::ReturnStatement {
            token: _,
            return_value: return_value,
        } => eval_return_statement(return_value, env),
        Node::LetStatement {
            token: _,
            name: name,
            value: value,
        } => eval_let_statement(name, value, env),
        Node::Identifier {
            token: _,
            value: value,
        } => eval_identifier(value, env),
        Node::FunctionLiteral {
            token: _,
            parameters: parameters,
            body: body,
        } => eval_function_literal(parameters, body, env),
        Node::CallExpression {
            token: _,
            function: function,
            arguments: arguments,
        } => eval_call_expression(function, arguments, env),
        Node::ArrayLiteral {
            token: _,
            elements,
        } => eval_array_literal(elements, env),
        Node::HashLiteral {
            token: _,
            elements,
        } => eval_hash_literal(elements, env),
        Node::IndexExpression {
            token,
            left,
            index,
        } => eval_index_expression(left, index, env),
    }
}

fn eval_program(nodes: &Vec<Box<Node>>, env: &mut Environment) -> Option<Rc<Object>> {
    let mut result = None;

    for node in nodes.iter() {
        let r = eval(node, env)?;
        if let Object::ReturnValue(v) = r.borrow() {
            return Some(v.clone());
        } else if let Object::Error(_) = r.borrow() {
            return Some(r);
        }
        result = Some(r);
    }

    return result;
}

fn eval_block_statements(nodes: &Vec<Box<Node>>, env: &mut Environment) -> Option<Rc<Object>> {
    let mut result = None;

    for node in nodes.iter() {
        let r = eval(node, env)?;

        if r.is_same(&Object::ReturnValue(Rc::new(Object::Null)))
            || r.is_same(&Object::Error(String::new()))
        {
            return Some(r);
        }

        result = Some(r);
    }

    return result;
}

fn native_bool_to_bool_object(b: bool) -> Object {
    if b {
        object::TRUE
    } else {
        object::FALSE
    }
}

fn eval_prefix_expression(
    operator: &str,
    right: &Box<Node>,
    env: &mut Environment,
) -> Option<Rc<Object>> {
    let right = eval(right, env)?;
    if right.is_error() {
        return Some(right);
    }
    match operator {
        "!" => Some(eval_bang_operator_expression(right)),
        "-" => Some(eval_minus_prefix_operator_expression(right)),
        _ => Some(Rc::new(Object::Error(format!(
            "unknown operator: {}, {:?}",
            operator, right
        )))),
    }
}

fn eval_bang_operator_expression(right: Rc<Object>) -> Rc<Object> {
    Rc::new(match right.borrow() {
        Object::Bool(b) => native_bool_to_bool_object(!*b),
        Object::Null => object::TRUE,
        _ => object::FALSE,
    })
}

fn eval_minus_prefix_operator_expression(right: Rc<Object>) -> Rc<Object> {
    if let Object::Integer(i) = right.borrow() {
        Rc::new(Object::Integer(-*i))
    } else {
        Rc::new(Object::Error(format!("unknown object: {:?}", right)))
    }
}

fn eval_infix_expression(
    left: &Box<Node>,
    operator: &str,
    right: &Box<Node>,
    env: &mut Environment,
) -> Option<Rc<Object>> {
    let left = eval(left, env)?;
    if left.is_error() {
        return Some(left);
    }

    let right = eval(right, env)?;
    if right.is_error() {
        return Some(right);
    }

    if left.is_integer() && right.is_integer() {
        if let Object::Integer(n) = left.borrow() {
            if let Object::Integer(m) = right.borrow() {
                return Some(eval_integer_infix_expression(operator, *n, *m));
            }
        }
    }

    if left.is_string() && right.is_string() {
        if let Object::StringValue(l) = left.borrow() {
            if let Object::StringValue(r) = right.borrow() {
                return Some(eval_string_infix_expression(operator, l, r));
            }
        }
    }

    if !left.is_same(&right) {
        return Some(Rc::new(Object::Error(format!(
            "type mismatch: {:?}, {:?}, {:?}",
            left, operator, right
        ))));
    }

    Some(Rc::new(match operator {
        "==" => native_bool_to_bool_object(left == right),
        "!=" => native_bool_to_bool_object(left != right),
        _ => Object::Error(format!(
            "unknown operator: {:?}, {:?}, {:?}",
            left, operator, right
        )),
    }))
}

fn eval_integer_infix_expression(operator: &str, left: i64, right: i64) -> Rc<Object> {
    Rc::new(match operator {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        "<" => native_bool_to_bool_object(left < right),
        ">" => native_bool_to_bool_object(left > right),
        "==" => native_bool_to_bool_object(left == right),
        "!=" => native_bool_to_bool_object(left != right),
        _ => Object::Error(format!(
            "unknown operator: {:?}, {:?}, {:?}",
            left, operator, right
        )),
    })
}

fn eval_string_infix_expression(operator: &str, left: &str, right: &str) -> Rc<Object> {
    if operator != "+" {
        return Rc::new(Object::Error(format!(
            "unknown operator: {:?}, {:?}, {:?}",
            left, operator, right
        )));
    }
    return Rc::new(Object::StringValue(left.to_owned() + right));
}

fn eval_if_expression(
    condition: &Box<Node>,
    consequence: &Box<Node>,
    alternative: &Option<Box<Node>>,
    env: &mut Environment,
) -> Option<Rc<Object>> {
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
        return Some(Rc::new(Object::Null));
    }
}

fn is_truthy(object: Rc<Object>) -> bool {
    match object.borrow() {
        Object::Null => false,
        Object::Bool(true) => true,
        Object::Bool(false) => false,
        _ => true,
    }
}

fn eval_return_statement(return_value: &Box<Node>, env: &mut Environment) -> Option<Rc<Object>> {
    let val = eval(return_value, env)?;
    if val.is_error() {
        return Some(val);
    }
    return Some(Rc::new(Object::ReturnValue(val)));
}

fn eval_let_statement(
    name: &Box<Node>,
    value: &Box<Node>,
    env: &mut Environment,
) -> Option<Rc<Object>> {
    let val = eval(value, env)?;
    if val.is_error() {
        return Some(val);
    }

    if let Node::Identifier { token: _, value: v } = name.borrow() {
        let val = env.set(v.to_owned(), val);
        return val;
    } else {
        return Some(Rc::new(Object::Error(format!(
            "invalid identifier: {:?}",
            name
        ))));
    }
}

fn eval_identifier(value: &String, env: &mut Environment) -> Option<Rc<Object>> {
    let mut object = env.get(value);

    if object.is_none() {

        object = find_builtin(value);

        if object.is_none() {
            return Some(Rc::new(Object::Error(format!(
                "identifier not found: {:?}",
                value
            ))));
        }
    }
    return object;
}

fn eval_function_literal(
    parameters: &Vec<Box<Node>>,
    body: &Box<Node>,
    env: &mut Environment,
) -> Option<Rc<Object>> {
    Some(Rc::new(Object::Function {
        parameters: parameters.clone(),
        body: body.clone(),
        environment: env.clone(),
    }))
}

fn eval_call_expression(
    function: &Box<Node>,
    arguments: &Vec<Box<Node>>,
    env: &mut Environment,
) -> Option<Rc<Object>> {
    let function = eval(function, env)?;

    if function.is_error() {
        return Some(function);
    }

    let arguments = eval_expression(arguments, env)?;
    if arguments.len() == 1 {
        if let Object::Error(e) = arguments[0].borrow() {
            return Some(Rc::new(Object::Error(e.to_string())));
        }
    }

    return apply_function(function, arguments);
}

fn eval_expression(expressions: &Vec<Box<Node>>, env: &mut Environment) -> Option<Vec<Rc<Object>>> {
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

fn apply_function(function: Rc<Object>, argument: Vec<Rc<Object>>) -> Option<Rc<Object>> {
    if let Object::Builtin(name) = function.borrow() {
        return do_builtin(name, argument);
    }

    if let Object::Function {
        parameters: p,
        body: b,
        environment: e,
    } = function.borrow()
    {
        let mut extended_env = Environment::new_enclosed(Rc::new(e.clone()));
        for (i, v) in p.iter().enumerate() {
            let v = v.borrow();
            if let Node::Identifier {
                token: _,
                value: key,
            } = v
            {
                let a = argument.get(i);
                if a.is_some() {
                    let a = a.unwrap();
                    extended_env.set(key.to_owned(), a.clone());
                }
            }
        }

        println!("[apply_function] extended_env is {:?}", extended_env);
        println!("[apply_function] body is {:?}", b);

        let result = eval(&b, &mut extended_env);

        if let Some(s) = result.borrow() {
            if let Object::ReturnValue(v) = s.borrow() {
                return Some(v.clone());
            }
        }

        return result;
    }

    return None;
}

fn eval_array_literal(elements: &Vec<Box<Node>>, env: &mut Environment) -> Option<Rc<Object>> {
    let elements = eval_expression(elements, env)?;
    if elements.len() == 1 && elements[0].is_error() {
        return Some(elements[0].clone());
    }
    return Some(Rc::new(Object::Array(elements)));
}

fn eval_hash_literal(elements: &Vec<Box<Node>>, env: &mut Environment) -> Option<Rc<Object>> {
    let mut hash_object_value = HashMap::<String, HashPair>::new();

    let mut iter = elements.chunks(2);

    for chunk in iter.next() {
        let (key, value) = (chunk[0].clone(), chunk[1].clone());
        let key = eval(&key, env)?;
        if key.is_error() {
            return Some(key);
        }
        let value = eval(&value, env)?;
        if value.is_error() {
            return Some(value);
        }
        let key = key.create_hash_key()?;
        hash_object_value.insert(key.to_owned(), HashPair{ key, value });
    }

    return Some(Rc::new(Object::Hash(hash_object_value)));
}

fn eval_index_expression(
    left: &Box<Node>,
    index: &Box<Node>,
    env: &mut Environment,
) -> Option<Rc<Object>> {
    let left = eval(left, env)?;
    if left.is_error() {
        return Some(left);
    }
    let index = eval(index, env)?;
    if index.is_error() {
        return Some(index);
    }
    if let Object::Array(elements) = left.borrow() {
        if let Object::Integer(i) = index.borrow() {
            let max = elements.len() - 1;
            let i = *i as usize;
            if i < 0 || max < i {
                return Some(Rc::new(Object::Error(format!(
                    "invalid index: {}",
                    i
                ))));
            }
            return Some(elements[i].clone());
        }
    } else if let Object::Hash(elements) = left.borrow() {
        let key = index.create_hash_key();
        if key.is_none() {
            Some(Rc::new(Object::Error(format!(
                "invalid index: {:?}", index))));
        }
        let key = key.unwrap();
        let value = elements.get(&key)?;
        return Some(value.value.clone());
    }
    return Some(Rc::new(Object::Error(format!(
        "index operator not supported: {:?}",
        left
    ))));
}

fn find_builtin(s: &str) -> Option<Rc<Object>> {
    match s {
        "len" | "first" | "last" | "rest" | "push" => Some(Rc::new(Object::Builtin(s.to_owned()))),
        _ => None,
    }
}

fn do_builtin(s: &str, args: Vec<Rc<Object>>) -> Option<Rc<Object>> {
    match s {
        "len" => builtin_len(args),
        "first" => builtin_first(args),
        "last" => builtin_last(args),
        "rest" => builtin_rest(args),
        "push" => builtin_push(args),
        _ => None,
    }
}

fn builtin_len(args: Vec<Rc<Object>>) -> Option<Rc<Object>> {
    if args.len() != 1 {
        return Some(Rc::new(Object::Error(format!("wrong number of arguments, got = {}, want = 1", args.len()))));
    }

    let o = args.get(0).unwrap();

    match o.borrow() {
        Object::StringValue(v) => Some(Rc::new(Object::Integer(v.len() as i64))),
        Object::Array(v) => Some(Rc::new(Object::Integer(v.len() as i64))),
        _ => Some(Rc::new(Object::Error("argument to len not supported".to_owned())))
    }
}

fn builtin_first(args: Vec<Rc<Object>>) -> Option<Rc<Object>> {
    if args.len() != 1 {
        return Some(Rc::new(Object::Error(format!("wrong number of arguments, got = {}, want = 1", args.len()))));
    }

    let o = args.get(0).unwrap();
    if let Object::Array(elements) = o.borrow() {
        return Some(elements[0].clone());
    } else {
        return Some(Rc::new(Object::Error(format!("argument to first must be Array, got = {:?}", o))));
    }
}


fn builtin_last(args: Vec<Rc<Object>>) -> Option<Rc<Object>> {
    if args.len() != 1 {
        return Some(Rc::new(Object::Error(format!("wrong number of arguments, got = {}, want = 1", args.len()))));
    }

    let o = args.get(0).unwrap();
    if let Object::Array(elements) = o.borrow() {
        return Some(elements[elements.len()-1].clone());
    } else {
        return Some(Rc::new(Object::Error(format!("argument to first must be Array, got = {:?}", o))));
    }
}

fn builtin_rest(args: Vec<Rc<Object>>) -> Option<Rc<Object>> {
    if args.len() != 1 {
        return Some(Rc::new(Object::Error(format!("wrong number of arguments, got = {}, want = 1", args.len()))));
    }

    let o = args.get(0).unwrap();
    if let Object::Array(elements) = o.borrow() {
        return Some(Rc::new(Object::Array(elements[1..].to_owned())));
    } else {
        return Some(Rc::new(Object::Error(format!("argument to first must be Array, got = {:?}", o))));
    }
}

fn builtin_push(args: Vec<Rc<Object>>) -> Option<Rc<Object>> {
    if args.len() != 2 {
        return Some(Rc::new(Object::Error(format!("wrong number of arguments, got = {}, want = 2", args.len()))));
    }

    let arr = args.get(0).unwrap();
    let elm = args.get(1).unwrap();

    if let Object::Array(elements) = arr.borrow() {
        let mut elements = elements.to_owned();
        elements.push(elm.to_owned());
        return Some(Rc::new(Object::Array(elements)));
    } else {
        return Some(Rc::new(Object::Error(format!("argument to first must be Array, got = {:?}", arr))));
    }
}