use crate::build_tools::ast::{Expression, Identifier};
use crate::build_tools::token::*;
use crate::build_tools::lexer::{Lexer};

use std::collections::HashMap;

static LOWEST: usize = 1;
static EQUALS: usize = 2;
static LOGICAL: usize = 3;
static LESS_GREATER: usize = 4;
static SUM: usize = 5;
static PRODUCT: usize = 6;
static MODULO: usize = 7;
static PREFIX: usize = 8;
static CALL: usize = 9;
static INDEX: usize = 10;

struct Precedences;

impl Precedences {
    pub fn all() -> HashMap<TokenType, usize> {
        let mut precendences: HashMap<TokenType, usize> = HashMap::new();

        precendences.insert(TokenType::EQUAL_EQUAL, EQUALS);
        precendences.insert(TokenType::BANG_EQUAL, EQUALS);
        precendences.insert(TokenType::LESS, LESS_GREATER);
        precendences.insert(TokenType::GREATER, LESS_GREATER);
        precendences.insert(TokenType::LESS_EQUAL, LESS_GREATER);
        precendences.insert(TokenType::GREATER_EQUAL, LESS_GREATER);
        precendences.insert(TokenType::PLUS, SUM);
        precendences.insert(TokenType::MINUS, SUM);
        precendences.insert(TokenType::SLASH, PRODUCT);
        precendences.insert(TokenType::STAR, PRODUCT);
        precendences.insert(TokenType::MOD, MODULO);
        precendences.insert(TokenType::AND, LOGICAL);
        precendences.insert(TokenType::OR, LOGICAL);
        precendences.insert(TokenType::LEFT_PAREN, CALL);
        precendences.insert(TokenType::LEFT_BRACKET, INDEX);

        precendences
    }
}

type PrefixParseFunc = fn() -> Box<dyn Expression>;
type InfixParseFunc = fn(expr: dyn Expression) -> dyn Expression;
type PostfixParseFunc = fn() -> Box<dyn Expression>;

struct Parser<'a> {
    lexer: &'a Lexer,
    errors: Vec<String>,

    current_token: Token,
    peek_token: Token,
    prev_token: Token,

    prefix_parse_funcs: HashMap<TokenType, PrefixParseFunc>,
    infix_parse_funcs: HashMap<TokenType, InfixParseFunc>,
    postfix_parse_funcs: HashMap<TokenType, PostfixParseFunc>,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a Lexer) -> &'a Parser {
        let mut parser = &Parser {
            lexer,
            errors: vec![],
            current_token: Token { line: 0, literal: String::from(""), token_type: TokenType::NONE },
            peek_token: Token { line: 0, literal: String::from(""), token_type: TokenType::NONE },
            prev_token: Token { line: 0, literal: String::from(""), token_type: TokenType::NONE },
            prefix_parse_funcs: HashMap::new(),
            infix_parse_funcs: HashMap::new(),
            postfix_parse_funcs: HashMap::new(),
        };

        parser.prefix_parse_funcs = HashMap::new();

        parser.register_prefix(TokenType::IDENTIFIER, parser.parse_identifier());
        // parser.register_prefix(INTEGER, parser.parse_integer_literal);
        // parser.register_prefix(BANG, parser.parse_prefix_expression);
        // parser.register_prefix(MINUS, parser.parse_prefix_expression);
        // parser.register_prefix(TRUE, parser.parse_boolean);
        // parser.register_prefix(FALSE, parser.parse_boolean);
        // parser.register_prefix(LEFT_PAREN, parser.parse_grouped_expression);
        // parser.register_prefix(IF, parser.parse_if_expression);
        // parser.register_prefix(FUNCTION, parser.parse_function_literal);
        // parser.register_prefix(STRING, parser.parse_string_literal);
        // parser.register_prefix(LEFT_BRACKET, parser.parse_array_literal);
        // parser.register_prefix(LEFT_BRACE, parser.parse_hash_literal);

        parser
    }

    fn register_prefix(&mut self, token_type: TokenType, func: PrefixParseFunc) {
        self.prefix_parse_funcs[&token_type] = func;
    }

    fn parse_identifier(&self) -> impl Fn() -> Box<dyn Expression> {
        let contains_key = self.postfix_parse_funcs.contains_key(&self.peek_token.token_type);
        
        if contains_key {
            let postfix = self.postfix_parse_funcs[&self.peek_token.token_type];

            self.next_token();

            return || postfix();
        }
        
        return || Box::new(Identifier {
            token: self.current_token,
            value: self.current_token.literal,
        })
    }

    fn next_token(&mut self) {
        self.prev_token = self.current_token;
        self.current_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }
}