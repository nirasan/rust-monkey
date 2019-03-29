extern crate rust_monkey;

use rust_monkey::lexer::Lexer;
use rust_monkey::parser::Parser;
use rust_monkey::object::Object;
use rust_monkey::evaluator::eval;

#[test]
fn test_evaluator() {
    assert_eq!(evaluate("5"), Some(Object::Integer(5)));
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
    let object = eval(&program);
    println!("OBJECT: {:?}", object);
    return object;
}