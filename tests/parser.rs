extern crate rust_monkey;

use rust_monkey::lexer::Lexer;
use rust_monkey::parser::Parser;

#[test]
fn test1() {
    parse("let five = 5;");
    parse("let ten 10;");
    parse("return 10;");
    parse("foobar;");
    parse("!5;");
    parse("-15;");
    parse("5 + 5;");
    parse("5 - 5;");
    parse("1 + 2 * 3;");
    parse("5 > 4 == 3 < 2");
    parse("true != false");
    parse("(1 + 2) * 3;");
}

fn parse(input: &str) {
    println!("=====\nINPUT:\n{}\n", input);

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);

    let program = p.parse_program();

    println!("PROGRAM:\n{:?}\n", program);
    println!("ERROR:\n{:?}\n", p.errors());
}