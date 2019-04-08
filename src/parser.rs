use crate::ast;
use crate::lexer::Lexer;
use crate::token::Token;

use std::collections::HashMap;
use std::mem;

type PrefixParseFn = fn(&mut Parser) -> Option<Box<ast::Node>>;
type InfixParseFn = fn(&mut Parser, Box<ast::Node>) -> Option<Box<ast::Node>>;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<Token, PrefixParseFn>,
    infix_parse_fns: HashMap<Token, InfixParseFn>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let cur_token = lexer.token();
        let peek_token = lexer.token();
        let mut parser = Parser {
            lexer,
            cur_token,
            peek_token,
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        parser.register_prefix_parse_fn(Token::Ident(String::new()), Parser::parse_identifier);
        parser.register_prefix_parse_fn(Token::Int(String::new()), Parser::parse_integer_literal);
        parser.register_prefix_parse_fn(Token::Str(String::new()), Parser::parse_string_literal);
        parser.register_prefix_parse_fn(Token::Bang, Parser::parse_prefix_expression);
        parser.register_prefix_parse_fn(Token::Minus, Parser::parse_prefix_expression);
        parser.register_prefix_parse_fn(Token::True, Parser::parse_boolean);
        parser.register_prefix_parse_fn(Token::False, Parser::parse_boolean);
        parser.register_prefix_parse_fn(Token::LParen, Parser::parse_grouped_expression);
        parser.register_prefix_parse_fn(Token::If, Parser::parse_if_expression);
        parser.register_prefix_parse_fn(Token::Function, Parser::parse_function_literal);
        parser.register_prefix_parse_fn(Token::LBracket, Parser::parse_array_literal);
        parser.register_prefix_parse_fn(Token::LBrace, Parser::parse_hash_literal);

        parser.register_infix_parse_fn(Token::Plus, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(Token::Minus, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(Token::Slash, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(Token::Asterisk, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(Token::Eq, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(Token::NotEq, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(Token::LT, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(Token::GT, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(Token::LParen, Parser::parse_call_expression);
        parser.register_infix_parse_fn(Token::LBracket, Parser::parse_index_expression);

        return parser;
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
        self.errors.push(format!(
            "expected next token to be {:?}, got {:?} insted",
            token, self.peek_token
        ))
    }

    pub fn parse_program(&mut self) -> Option<Box<ast::Node>> {
        let mut statements = vec![];
        while self.cur_token != Token::Eof {
            println!("cur_token is {:?}", self.cur_token);
            let statement = self.parse_statement();
            if let Some(statement) = statement {
                statements.push(statement);
            }
            self.next_token();
        }
        return ast::Node::new_program(statements);
    }

    pub(self) fn parse_statement(&mut self) -> Option<Box<ast::Node>> {
        let statement = match self.cur_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };
        return statement.and_then(|s| ast::Node::new_statement(s));
    }

    pub(self) fn parse_let_statement(&mut self) -> Option<Box<ast::Node>> {
        let let_token = self.cur_token.clone();

        if !self.expect_peek(Token::Ident(String::new())) {
            return None;
        }

        let identifier =
            ast::Node::new_identifier(self.cur_token.clone(), self.cur_token.to_string());

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        self.next_token();

        let expression = self.parse_expression(Precedence::LOWEST);
        if expression.is_none() {
            return None;
        }
        let expression = expression.unwrap();

        if !self.expect_peek(Token::SemiColon) {
            return None;
        }

        return ast::Node::new_let_statement(let_token, identifier, expression);
    }

    pub(self) fn parse_return_statement(&mut self) -> Option<Box<ast::Node>> {
        let token = self.cur_token.clone();

        self.next_token();

        let expression = self.parse_expression(Precedence::LOWEST);
        if expression.is_none() {
            return None;
        }
        let expression = expression.unwrap();

        if !self.expect_peek(Token::SemiColon) {
            return None;
        }

        return ast::Node::new_return_statement(token, expression);
    }

    pub(self) fn parse_expression_statement(&mut self) -> Option<Box<ast::Node>> {
        let token = self.cur_token.clone();

        let expression = self.parse_expression(Precedence::LOWEST);

        if expression.is_none() {
            return None;
        }
        let expression = expression.unwrap();

        if self.peek_token_is(Token::SemiColon) {
            self.next_token();
        }

        return ast::Node::new_expression_statement(token, expression);
    }

    pub(self) fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<ast::Node>> {
        let prefix_parse_fn = self.get_prefix_parse_fn(self.cur_token.clone())?;

        let mut left = prefix_parse_fn(self)?;
        left = ast::Node::new_expression(left)?;

        while !self.peek_token_is(Token::SemiColon) && precedence < self.peek_precedence() {
            let infix_parse_fn = self.get_infix_parse_fn(self.peek_token.clone());

            if infix_parse_fn.is_none() {
                return Some(left);
            }
            let infix_parse_fn = infix_parse_fn.unwrap();

            self.next_token();

            left = infix_parse_fn(self, left)?;
            left = ast::Node::new_expression(left)?;
        }

        return Some(left);
    }

    pub(self) fn parse_identifier(&mut self) -> Option<Box<ast::Node>> {
        Some(ast::Node::new_identifier(
            self.cur_token.clone(),
            self.cur_token.to_string(),
        ))
    }

    pub(self) fn parse_integer_literal(&mut self) -> Option<Box<ast::Node>> {
        let token = self.cur_token.clone();

        let value = match &self.cur_token {
            Token::Int(s) => s.parse::<i64>().ok(),
            _ => None,
        };

        if value.is_none() {
            return None;
        }

        return Some(ast::Node::new_integer_literal(token, value.unwrap()));
    }

    pub(self) fn parse_string_literal(&mut self) -> Option<Box<ast::Node>> {
        Some(ast::Node::new_string_literal(
            self.cur_token.clone(),
            self.cur_token.to_string(),
        ))
    }

    pub(self) fn parse_array_literal(&mut self) -> Option<Box<ast::Node>> {
        let token = self.cur_token.clone();

        let elements = self.parse_expression_list(Token::RBracket)?;

        ast::Node::new_array_literal(
            token,
            elements,
        )
    }

    pub(self) fn parse_hash_literal(&mut self) -> Option<Box<ast::Node>> {
        let token = self.cur_token.clone();
        println!("[parse_hash_literal] start token {:?}", token);

        let mut elements = vec![];

        while !self.peek_token_is(Token::RBrace) {
            self.next_token();

            let expression = self.parse_expression(Precedence::LOWEST);
            let key = expression.and_then(|e| ast::Node::new_expression(e))?;

            if !self.expect_peek(Token::Colon) {
                return None;
            }

            self.next_token();

            let expression = self.parse_expression(Precedence::LOWEST);
            let val = expression.and_then(|e| ast::Node::new_expression(e))?;

            elements.push(key);
            elements.push(val);

            if !self.peek_token_is(Token::RBrace) && !self.expect_peek(Token::Comma) {
                return None;
            }
        }

        if !self.expect_peek(Token::RBrace) {
            return None;
        }

        return ast::Node::new_hash_literal(
            token, elements
        );
    }

    pub(self) fn parse_boolean(&mut self) -> Option<Box<ast::Node>> {
        Some(ast::Node::new_boolean(
            self.cur_token.clone(),
            self.cur_token_is(Token::True),
        ))
    }

    pub(self) fn parse_prefix_expression(&mut self) -> Option<Box<ast::Node>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.to_string();

        self.next_token();

        let right = self.parse_expression(Precedence::PREFIX);

        if right.is_none() {
            return None;
        }

        return ast::Node::new_prefix_expression(token, operator, right.unwrap());
    }

    pub(self) fn parse_infix_expression(&mut self, left: Box<ast::Node>) -> Option<Box<ast::Node>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.to_string();

        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence);

        if right.is_none() {
            return None;
        }

        return ast::Node::new_infix_expression(token, left, operator, right.unwrap());
    }

    pub(self) fn parse_grouped_expression(&mut self) -> Option<Box<ast::Node>> {
        self.next_token();

        let exp = self.parse_expression(Precedence::LOWEST);

        if !self.expect_peek(Token::RParen) {
            return None;
        }

        eprintln!("grouped expression is {:?}", exp);

        return exp;
    }

    pub(self) fn parse_if_expression(&mut self) -> Option<Box<ast::Node>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(Token::LParen) {
            return None;
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::LOWEST);

        if condition.is_none() {
            return None;
        }
        let condition = condition.unwrap();

        if !self.expect_peek(Token::RParen) {
            return None;
        }

        if !self.expect_peek(Token::LBrace) {
            return None;
        }

        let consequence = self.parse_block_statement();

        if consequence.is_none() {
            return None;
        }
        let consequence = consequence.unwrap();

        let mut alternative = None;
        if self.peek_token_is(Token::Else) {
            self.next_token();

            if !self.expect_peek(Token::LBrace) {
                return None;
            }

            alternative = self.parse_block_statement();
        }

        return ast::Node::new_if_expression(token, condition, consequence, alternative);
    }

    pub(self) fn parse_block_statement(&mut self) -> Option<Box<ast::Node>> {
        let token = self.cur_token.clone();
        let mut statements = vec![];

        self.next_token();

        while !self.cur_token_is(Token::RBrace) && !self.cur_token_is(Token::Eof) {
            let statement = self.parse_statement();
            if let Some(s) = statement {
                statements.push(s);
            }
            self.next_token();
        }

        return ast::Node::new_block_statement(token, statements);
    }

    pub(self) fn parse_function_literal(&mut self) -> Option<Box<ast::Node>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(Token::LParen) {
            return None;
        }

        let parameters = self.parse_function_parameters();

        if parameters.is_none() {
            return None;
        }
        let parameters = parameters.unwrap();

        if !self.expect_peek(Token::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        if body.is_none() {
            return None;
        }
        let body = body.unwrap();

        return ast::Node::new_function_literal(token, parameters, body);
    }

    pub(self) fn parse_function_parameters(&mut self) -> Option<Vec<Box<ast::Node>>> {
        let mut identifiers = vec![];

        if self.peek_token_is(Token::RParen) {
            self.next_token();
            return Some(identifiers);
        };

        self.next_token();

        identifiers.push(ast::Node::new_identifier(
            self.cur_token.clone(),
            self.cur_token.to_string(),
        ));

        while self.peek_token_is(Token::Comma) {
            self.next_token();
            self.next_token();
            identifiers.push(ast::Node::new_identifier(
                self.cur_token.clone(),
                self.cur_token.to_string(),
            ));
        }

        if !self.expect_peek(Token::RParen) {
            return None;
        }

        return Some(identifiers);
    }

    pub(self) fn parse_call_expression(
        &mut self,
        function: Box<ast::Node>,
    ) -> Option<Box<ast::Node>> {
        let token = self.cur_token.clone();
        let arguments = self.parse_expression_list(Token::RParen);
        if arguments.is_none() {
            return None;
        }
        return ast::Node::new_call_expression(token, function, arguments.unwrap());
    }

    pub(self) fn parse_expression_list(&mut self, end: Token) -> Option<Vec<Box<ast::Node>>> {
        let mut list = vec![];

        if self.peek_token_is(end.clone()) {
            self.next_token();
            return Some(list);
        }

        self.next_token();
        let expression = self.parse_expression(Precedence::LOWEST);
        let expression = expression.and_then(|e| ast::Node::new_expression(e));
        if expression.is_none() {
            return None;
        }
        list.push(expression.unwrap());

        while self.peek_token_is(Token::Comma) {
            self.next_token();
            self.next_token();
            let expression = self.parse_expression(Precedence::LOWEST);
            let expression = expression.and_then(|e| ast::Node::new_expression(e));
            if expression.is_none() {
                return None;
            }
            list.push(expression.unwrap());
        }

        if !self.expect_peek(end) {
            return None;
        }

        return Some(list);
    }

    pub(self) fn parse_index_expression(
        &mut self,
        left: Box<ast::Node>,
    ) -> Option<Box<ast::Node>> {
        let token = self.cur_token.clone();

        self.next_token();
        let index = self.parse_expression(Precedence::LOWEST)?;

        if !self.expect_peek(Token::RBracket) {
            return None;
        }

        return ast::Node::new_index_expression(token, left, index);
    }

    fn peek_precedence(&self) -> Precedence {
        self.precedence(&self.peek_token)
    }

    fn cur_precedence(&self) -> Precedence {
        self.precedence(&self.cur_token)
    }

    fn precedence(&self, token: &Token) -> Precedence {
        match token {
            Token::Eq => Precedence::EQUALS,
            Token::NotEq => Precedence::EQUALS,
            Token::LT => Precedence::LESSGREATER,
            Token::GT => Precedence::LESSGREATER,
            Token::Plus => Precedence::SUM,
            Token::Minus => Precedence::SUM,
            Token::Slash => Precedence::PRODUCT,
            Token::Asterisk => Precedence::PRODUCT,
            Token::LParen => Precedence::CALL,
            Token::LBracket => Precedence::INDEX,
            _ => Precedence::LOWEST,
        }
    }

    fn register_prefix_parse_fn(&mut self, token: Token, f: PrefixParseFn) {
        self.prefix_parse_fns.insert(token, f);
    }

    fn register_infix_parse_fn(&mut self, token: Token, f: InfixParseFn) {
        self.infix_parse_fns.insert(token, f);
    }

    fn get_prefix_parse_fn(&mut self, token: Token) -> Option<PrefixParseFn> {
        for (k, v) in self.prefix_parse_fns.iter() {
            if token.is_same(k) {
                return Some(*v);
            }
        }
        return None;
    }

    fn get_infix_parse_fn(&mut self, token: Token) -> Option<InfixParseFn> {
        for (k, v) in self.infix_parse_fns.iter() {
            if token.is_same(k) {
                return Some(*v);
            }
        }
        return None;
    }
}

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
    INDEX,
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
