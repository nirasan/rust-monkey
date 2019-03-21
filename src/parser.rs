use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast;
use crate::ast::LetStatement;

use std::mem;

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Parser {
        let cur_token = lexer.token();
        let peek_token = lexer.token();
        Parser{
            lexer,
            cur_token,
            peek_token
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.token();
    }

    fn cur_token_is(&self, token: Token) -> bool {
        mem::discriminant(&(self.cur_token)) == mem::discriminant(&token)
    }

    fn peek_token_is(&self, token: Token) -> bool {
        mem::discriminant(&(self.peek_token)) == mem::discriminant(&token)
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token_is(token.clone()) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program{
            statements: vec![]
        };
        while self.cur_token != Token::Eof {
            let statement = self.parse_statement();
            if let Some(statement) = statement {
                program.statements.push(statement);
            }
            self.next_token();
        }
        return program;
    }

    fn parse_statement(&mut self) -> Option<Box<ast::Statement>> {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            _ => None
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<ast::Statement>> {
        let let_token = self.cur_token.clone();

        if !self.expect_peek(Token::Ident(String::new())) {
            return None;
        }

        let identifier = ast::Identifier{token: self.cur_token.clone(), value: self.cur_token.to_string()};

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        while self.cur_token_is(Token::SemiColon) {
            self.next_token();
        }

        Some(Box::new(LetStatement{
            token: let_token,
            name: identifier,
            value: Box::new(ast::DummyExpression{})
        }))
    }
}

#[test]
fn test_new() {
    let input = r#"let five = 5;"#.to_string();
    let mut lexer = Lexer::new(input);

    let parser = Parser::new(lexer);

    assert_eq!(parser.cur_token, Token::Let);
    assert_eq!(parser.peek_token, Token::Ident("five".to_string()));
}

#[test]
fn test_let() {
    let input = r#"let five = 5;"#.to_string();
    let mut lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    println!("{:?}", program);
}
