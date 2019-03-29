extern crate rust_monkey;

use rust_monkey::environment::Environment;
use rust_monkey::evaluator::eval;
use rust_monkey::lexer::Lexer;
use rust_monkey::object::Object;
use rust_monkey::parser::Parser;

use core::borrow::Borrow;
use std::rc::Rc;

#[test]
fn test_evaluator() {
    assert_eq_int(evaluate("5"), 5);
    assert_eq_bool(evaluate("true"), true);

    assert_eq_bool(evaluate("false"), false);
    assert_eq_bool(evaluate("!true"), false);
    assert_eq_bool(evaluate("!false"), true);
    assert_eq_bool(evaluate("!5"), false);
    assert_eq_bool(evaluate("!!true"), true);
    assert_eq_bool(evaluate("!!false"), false);

    assert_eq_int(evaluate("-5"), -5);
    assert_eq_int(evaluate("1 + 1"), 2);
    assert_eq_int(evaluate("2 - 1"), 1);
    assert_eq_int(evaluate("1 * 2"), 2);
    assert_eq_int(evaluate("4 / 2"), 2);
    assert_eq_int(evaluate("20 + 2 * -10"), 0);
    assert_eq_int(evaluate("(5 + 10 * 2 + 15 / 3) * 2 + -10"), 50);

    assert_eq_bool(evaluate("1 < 2"), true);
    assert_eq_bool(evaluate("1 > 2"), false);
    assert_eq_bool(evaluate("1 == 2"), false);
    assert_eq_bool(evaluate("1 != 2"), true);
    assert_eq_bool(evaluate("true == true"), true);
    assert_eq_bool(evaluate("false == false"), true);
    assert_eq_bool(evaluate("true != true"), false);
    assert_eq_bool(evaluate("false != false"), false);
    assert_eq_bool(evaluate("(1 < 2) == true"), true);

    assert_eq_int(evaluate("if (true) { 10 }"), 10);
    assert_eq_int(evaluate("if (false) { 10 } else { 20 }"), 20);
    assert_eq_int(evaluate("if (1) { 10 } else { 20 }"), 10);
    assert_eq_int(evaluate("return 10;"), 10);
    assert_eq_int(evaluate("return 10; 9"), 10);
    assert_eq_int(
        evaluate("if (true) { if (true) { return 10; } return 1; }"),
        10,
    );
    assert_eq_error(
        evaluate("1 + true"),
        "type mismatch: Integer(1), \"+\", Bool(true)",
    );
    assert_eq_error(
        evaluate("if (10 > 1) { false + true; }"),
        "unknown operator: Bool(false), \"+\", Bool(true)",
    );
    assert_eq_int(evaluate("let a = 5; a;"), 5);
    assert_eq_int(evaluate("let a = 5 * 5; a;"), 25);
    assert_eq_int(evaluate("let a = 5; let b = 6; a + b;"), 11);
    assert_eq_int(evaluate("let add = fn(a, b) { a + b; }; add(1, 2);"), 3);
}

fn evaluate(input: &str) -> Option<Rc<Object>> {
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if parser.errors().len() > 0 {
        println!("PARSER ERROR: {:?}", parser.errors());
        return None;
    }
    let program = program.expect("failed to parse program");
    println!("PROGRAM: {:?}", program);
    let mut env = Environment::new();
    let object = eval(&program, &mut env);
    println!("OBJECT: {:?}", object);
    return object;
}

fn assert_eq_int(result: Option<Rc<Object>>, i: i64) {
    assert_eq!(result, Some(Rc::new(Object::Integer(i))));
}

fn assert_eq_bool(result: Option<Rc<Object>>, b: bool) {
    assert_eq!(result, Some(Rc::new(Object::Bool(b))));
}

fn assert_eq_error(result: Option<Rc<Object>>, s: &str) {
    assert_eq!(result, Some(Rc::new(Object::Error(s.to_string()))));
}
