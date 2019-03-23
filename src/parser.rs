use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast;
use crate::ast::{LetStatement, ReturnStatement, DummyExpression, ExpressionStatement};

use std::mem;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let cur_token = lexer.token();
        let peek_token = lexer.token();
        Parser{
            lexer,
            cur_token,
            peek_token,
            errors: vec![],
        }
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
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
            self.peek_error(token);
            false
        }
    }

    fn peek_error(&mut self, token: Token) {
        self.errors.push(format!("expected next token to be {:?}, got {:?} insted", token, self.peek_token))
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program::new();
        while self.cur_token != Token::Eof {
            println!("cur_token is {:?}", self.cur_token);
            let statement = self.parse_statement();
            if let Some(statement) = statement {
                program.push(statement);
            }
            self.next_token();
        }
        return program;
    }

    fn parse_statement(&mut self) -> Option<Box<ast::Statement>> {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<ast::Statement>> {
        let let_token = self.cur_token.clone();

        if !self.expect_peek(Token::Ident(String::new())) {
            return None;
        }

        let identifier = ast::Identifier::new(self.cur_token.clone(), self.cur_token.to_string());

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        // TODO: skip until semicolon
        while !self.cur_token_is(Token::SemiColon) {
            self.next_token();
        }

        return Some(Box::new(LetStatement::new(
            let_token,
            identifier,
            Box::new(ast::DummyExpression{}),
        )));
    }

    fn parse_return_statement(&mut self) ->Option<Box<ast::Statement>> {
        let token = self.cur_token.clone();

        self.next_token();

        while self.cur_token_is(Token::SemiColon) {
            self.next_token();
        }

        return Some(Box::new(ReturnStatement::new(
            token, Box::new(DummyExpression{})
        )));
    }

    fn parse_expression_statement(&mut self) -> Option<Box<ast::Statement>> {
        let token = self.cur_token.clone();

        let expression = self.parse_expression(Precedence::LOWEST);
        let expression = if let Some(e) = expression {
            e
        } else {
            Box::new(ast::DummyExpression{})
        };

        if self.peek_token_is(Token::SemiColon) {
            self.next_token();
        }

        return Some(Box::new(ExpressionStatement::new(token, expression)));
    }

    fn parse_expression(&mut self, p: Precedence) -> Option<Box<ast::Expression>> {
        match &self.cur_token {
            Token::Ident(_) => self.parse_identifier(),
            Token::Int(_) => self.parse_integer_literal(),
            Token::Bang | Token::Minus => self.parse_prefix_expression(),
            _ => {
                self.errors.push(format!("no prefix parse function for {:?} found", self.cur_token));
                None
            }
        }
    }

    fn parse_identifier(&mut self) -> Option<Box<ast::Expression>> {
        Some(Box::new(
            ast::Identifier::new(self.cur_token.clone(), self.cur_token.to_string())
        ))
    }

    fn parse_integer_literal(&mut self) -> Option<Box<ast::Expression>> {
        let token = self.cur_token.clone();

        let value = match &self.cur_token {
            Token::Int(s) => s.parse::<i64>().ok(),
            _ => None,
        };

        if value.is_none() {
            return None;
        }

        return Some(Box::new(
            ast::IntegerLiteral::new(token, value.unwrap())
        ));
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<ast::Expression>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.to_string();

        self.next_token();

        let right = self.parse_expression(Precedence::PREFIX);

        if right.is_none() {
            return None;
        }

        return Some(Box::new(ast::PrefixExpression::new(
            token,
            operator,
            right.unwrap()
        )));
    }
}

enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL
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

    println!("PROGRAM: {:?}", program);
    println!("ERRORS: {:?}", parser.errors);
}
