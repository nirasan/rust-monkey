extern crate rust_monkey;

use rust_monkey::lexer::Lexer;
use rust_monkey::parser::Parser;
use rust_monkey::object::Object;
use rust_monkey::evaluator::eval;
use rust_monkey::environment::Environment;

#[test]
fn test_evaluator() {
    assert_eq!(evaluate("5"), Some(Object::Integer(5)));
    assert_eq!(evaluate("true"), Some(Object::Bool(true)));
    assert_eq!(evaluate("false"), Some(Object::Bool(false)));
    assert_eq!(evaluate("!true"), Some(Object::Bool(false)));
    assert_eq!(evaluate("!false"), Some(Object::Bool(true)));
    assert_eq!(evaluate("!5"), Some(Object::Bool(false)));
    assert_eq!(evaluate("!!true"), Some(Object::Bool(true)));
    assert_eq!(evaluate("!!false"), Some(Object::Bool(false)));
    assert_eq!(evaluate("-5"), Some(Object::Integer(-5)));
    assert_eq!(evaluate("1 + 1"), Some(Object::Integer(2)));
    assert_eq!(evaluate("2 - 1"), Some(Object::Integer(1)));
    assert_eq!(evaluate("1 * 2"), Some(Object::Integer(2)));
    assert_eq!(evaluate("4 / 2"), Some(Object::Integer(2)));
    assert_eq!(evaluate("20 + 2 * -10"), Some(Object::Integer(0)));
    assert_eq!(evaluate("(5 + 10 * 2 + 15 / 3) * 2 + -10"), Some(Object::Integer(50)));
    assert_eq!(evaluate("1 < 2"), Some(Object::Bool(true)));
    assert_eq!(evaluate("1 > 2"), Some(Object::Bool(false)));
    assert_eq!(evaluate("1 == 2"), Some(Object::Bool(false)));
    assert_eq!(evaluate("1 != 2"), Some(Object::Bool(true)));
    assert_eq!(evaluate("true == true"), Some(Object::Bool(true)));
    assert_eq!(evaluate("false == false"), Some(Object::Bool(true)));
    assert_eq!(evaluate("true != true"), Some(Object::Bool(false)));
    assert_eq!(evaluate("false != false"), Some(Object::Bool(false)));
    assert_eq!(evaluate("(1 < 2) == true"), Some(Object::Bool(true)));
    assert_eq!(evaluate("if (true) { 10 }"), Some(Object::Integer(10)));
    assert_eq!(evaluate("if (false) { 10 } else { 20 }"), Some(Object::Integer(20)));
    assert_eq!(evaluate("if (1) { 10 } else { 20 }"), Some(Object::Integer(10)));
    assert_eq!(evaluate("return 10;"), Some(Object::Integer(10)));
    assert_eq!(evaluate("return 10; 9"), Some(Object::Integer(10)));
    assert_eq!(evaluate("if (true) { if (true) { return 10; } return 1; }"), Some(Object::Integer(10)));
    assert_eq!(evaluate("1 + true"), Some(Object::Error("type mismatch: Integer(1), \"+\", Bool(true)".to_string())));
    assert_eq!(evaluate("if (10 > 1) { false + true; }"), Some(Object::Error("unknown operator: Bool(false), \"+\", Bool(true)".to_string())));
    assert_eq!(evaluate("let a = 5; a;"), Some(Object::Integer(5)));
    assert_eq!(evaluate("let a = 5 * 5; a;"), Some(Object::Integer(25)));
    assert_eq!(evaluate("let a = 5; let b = 6; a + b;"), Some(Object::Integer(11)));
    assert_eq!(evaluate("let add = fn(a, b) { a + b; }; add(1, 2);"), Some(Object::Integer(3)));
}

fn evaluate(input: &str) -> Option<Object> {
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