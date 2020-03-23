use crate::build_tools::ast::{
    BlockStatement, Boolean, Expression, Identifier, IfExpression, IntegerLiteral,
    PrefixExpression, Statement, ZeroValueExpression, ZeroValueStatement, LetStatement
};
use crate::build_tools::lexer::Lexer;
use crate::build_tools::token::*;

use std::collections::HashMap;

/// Operator precedence constants
static LOWEST: usize = 1;
/// =
static EQUALS: usize = 2;
/// && and ||
static LOGICAL: usize = 3;
/// > or <
static LESS_GREATER: usize = 4;
/// +
static SUM: usize = 5;
/// *
static PRODUCT: usize = 6;
/// %
static MODULO: usize = 7;
/// -x or !x
static PREFIX: usize = 8;
/// myFunction(x)
static CALL: usize = 9;
/// array[index], hash[key]
static INDEX: usize = 10;

struct Precedences;

/// Operator precedence table
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

type PrefixParseFunc = fn(parser: &mut Parser) -> Box<dyn Expression>;
type InfixParseFunc = fn(parser: &mut Parser, expr: Box<dyn Expression>) -> Box<dyn Expression>;
type PostfixParseFunc = fn(parser: &mut Parser) -> Box<dyn Expression>;

/// Parser holds a Lexer, its errors, the current_token, peek_token (next token), and
/// prev_token (used for ++ and --), as well as the prefix/infix/postfix functions
pub struct Parser {
    lexer: Lexer,
    errors: Vec<String>,

    current_token: Token,
    peek_token: Token,
    prev_token: Token,

    prefix_parse_funcs: HashMap<TokenType, PrefixParseFunc>,
    infix_parse_funcs: HashMap<TokenType, InfixParseFunc>,
    postfix_parse_funcs: HashMap<TokenType, PostfixParseFunc>,
}

impl Parser {
    /// New takes a Lexer, creates a Parser with that Lexer, sets the
    /// current and peek tokens, and returns the Parser.
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            errors: vec![],
            current_token: Token {
                line: 0,
                literal: "".to_owned(),
                token_type: TokenType::NONE,
            },
            peek_token: Token {
                line: 0,
                literal: "".to_owned(),
                token_type: TokenType::NONE,
            },
            prev_token: Token {
                line: 0,
                literal: "".to_owned(),
                token_type: TokenType::NONE,
            },
            prefix_parse_funcs: HashMap::new(),
            infix_parse_funcs: HashMap::new(),
            postfix_parse_funcs: HashMap::new(),
        };

        parser.register_prefix(TokenType::IDENTIFIER, parse_identifier);
        parser.register_prefix(TokenType::INTEGER, parse_integer_literal);
        parser.register_prefix(TokenType::BANG, parse_prefix_expression);
        parser.register_prefix(TokenType::MINUS, parse_prefix_expression);
        parser.register_prefix(TokenType::TRUE, parse_boolean);
        parser.register_prefix(TokenType::FALSE, parse_boolean);
        parser.register_prefix(TokenType::LEFT_PAREN, parse_grouped_expression);
        parser.register_prefix(TokenType::IF, parse_if_expression);
        // parser.register_prefix(FUNCTION, parser.parse_function_literal);
        // parser.register_prefix(STRING, parser.parse_string_literal);
        // parser.register_prefix(LEFT_BRACKET, parser.parse_array_literal);
        // parser.register_prefix(LEFT_BRACE, parser.parse_hash_literal);

        parser
    }

    fn register_prefix(&mut self, token_type: TokenType, func: PrefixParseFunc) {
        self.prefix_parse_funcs.insert(token_type, func);
    }

    fn next_token(&mut self) {
        self.prev_token = self.current_token.clone();
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.clone().next_token();
    }

    fn parse_expr(&mut self, precedence: usize) -> Option<Box<dyn Expression>> {
        let prefix = match self.prefix_parse_funcs.get(&self.current_token.token_type) {
            Some(&func) => func,
            _ => {
                self.no_prefix_parse_func_error(self.current_token.clone());
                return None;
            }
        };

        let mut left_expr = prefix(self);

        while !self.peek_token_type_is(TokenType::SEMICOLON)
            && precedence < self.peek_token_precedence()
        {
            let infix = match self.infix_parse_funcs.get(&self.peek_token.token_type) {
                Some(&func) => func,
                _ => {
                    return Some(left_expr);
                }
            };
            self.next_token();
            left_expr = infix(self, left_expr);
        }

        Some(left_expr)
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let block = BlockStatement {
            token: self.current_token,
            statements: vec![],
        };

        self.next_token();

        while !self.current_token_type_is(TokenType::RIGHT_BRACE)
            && !self.current_token_type_is(TokenType::EOF)
        {
            let stmt = match self.parse_statement() {
                Some(stmt) => {
                    block.statements.push(stmt);
                }
                _ => {
                    return BlockStatement {
                        token: Token {
                            line: 0,
                            literal: "".to_owned(),
                            token_type: TokenType::NONE,
                        },
                        statements: vec![],
                    }
                }
            };

            self.next_token();
        }

        block
    }

    fn parse_statement(&self) -> Option<Box<dyn Statement>> {
        let ret = match self.current_token.token_type {
            TokenType::LET => parse_let_statement(&mut self),
            TokenType::CONST => parse_const_statement(&mut self),
            TokenType::RETURN => parse_return_statement(&mut self),
            _ => parse_expr_stat(&mut self),
        };

        Some(ret)
    }

    fn peek_token_type_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn peek_token_precedence(&self) -> usize {
        match Precedences::all().get(&self.peek_token.token_type) {
            Some(precedence) => return *precedence,
            _ => return LOWEST,
        };
    }

    fn expect_peek_type(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_type_is(token_type) {
            self.next_token();
            return true;
        }

        self.peek_error(token_type);

        return false;
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "Line {}: Expected token to be {}, but found, {}",
            self.current_token.line, token_type, self.peek_token.literal,
        );
        self.errors.push(msg);
    }

    fn no_prefix_parse_func_error(&mut self, token: Token) {
        let msg = format!(
            "Line {}: No prefix parse function for {} found",
            self.current_token.line, token.literal
        );
        self.errors.push(msg);
    }

    fn current_token_type_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }
}

fn parse_identifier(parser: &mut Parser) -> Box<dyn Expression> {
    let contains_key = parser
        .postfix_parse_funcs
        .contains_key(&parser.peek_token.token_type);

    if contains_key {
        let postfix = parser.postfix_parse_funcs[&parser.peek_token.token_type];
        parser.next_token();
        return postfix(parser);
    }

    Box::new(Identifier {
        token: parser.current_token.clone(),
        value: parser.current_token.literal.clone(),
    })
}

fn parse_integer_literal(parser: &mut Parser) -> Box<dyn Expression> {
    Box::new(IntegerLiteral {
        token: parser.current_token.clone(),
        value: parser.current_token.literal.parse::<usize>().unwrap(),
    })
}

fn parse_prefix_expression(parser: &mut Parser) -> Box<dyn Expression> {
    let mut expr = PrefixExpression {
        token: parser.current_token.clone(),
        operator: parser.current_token.literal.clone(),
        right: Box::new(ZeroValueExpression {}),
    };

    parser.next_token();
    expr.right = match parser.parse_expr(PREFIX) {
        Some(expr) => expr,
        _ => {
            let msg = format!(
                "Line {}: Failed to parse expression {}.",
                parser.current_token.line, parser.current_token.literal
            );
            parser.errors.push(msg);
            Box::new(ZeroValueExpression {})
        }
    };

    Box::new(expr)
}

fn parse_boolean(parser: &mut Parser) -> Box<dyn Expression> {
    Box::new(Boolean {
        token: parser.current_token.clone(),
        value: parser.current_token_type_is(TokenType::TRUE),
    })
}

fn parse_grouped_expression(parser: &mut Parser) -> Box<dyn Expression> {
    parser.next_token();

    let expr = match parser.parse_expr(LOWEST) {
        Some(expr) => expr,
        _ => {
            let msg = format!(
                "Line {}: Failed to parse expression {}.",
                parser.current_token.line, parser.current_token.literal
            );
            parser.errors.push(msg);
            Box::new(ZeroValueExpression {})
        }
    };

    if !parser.expect_peek_type(TokenType::RIGHT_PAREN) {
        return Box::new(ZeroValueExpression {});
    }

    expr
}

fn parse_if_expression(parser: &mut Parser) -> Box<dyn Expression> {
    let expr = IfExpression {
        token: parser.current_token,
        condition: Box::new(ZeroValueExpression {}),
        consequence: BlockStatement {
            token: parser.current_token,
            statements: vec![],
        },
        alternative: BlockStatement {
            token: parser.current_token,
            statements: vec![],
        },
    };

    if !parser.expect_peek_type(TokenType::LEFT_PAREN) {
        return Box::new(ZeroValueExpression {});
    }

    parser.next_token();
    expr.condition = match parser.parse_expr(LOWEST) {
        Some(cond) => cond,
        _ => {
            let msg = format!(
                "Line {}: Failed to parse expression {}.",
                parser.current_token.line, parser.current_token.literal
            );
            parser.errors.push(msg);
            Box::new(ZeroValueExpression {})
        }
    };

    if !parser.expect_peek_type(TokenType::RIGHT_PAREN) {
        return Box::new(ZeroValueExpression {});
    }

    if !parser.expect_peek_type(TokenType::LEFT_BRACE) {
        return Box::new(ZeroValueExpression {});
    }

    expr.consequence = parser.parse_block_statement();

    if parser.peek_token_type_is(TokenType::ELSE) {
        parser.next_token();

        if !parser.expect_peek_type(TokenType::LEFT_BRACE) {
            return Box::new(ZeroValueExpression {});
        }

        expr.alternative = parser.parse_block_statement();
    }

    Box::new(expr)
}

fn parse_let_statement(parser: &mut Parser) -> Box<dyn Statement> {
    let stmt = LetStatement{token: parser.current_token};

    if !parser.expect_peek_type(TokenType::IDENTIFIER) {
        return LetStatement{};
    }

    stmt.name = Identifier{
        token: parser.current_token,
        value: parser.current_token.literal,
    };

    if !parser.expect_peek_type(TokenType::EQUAL) {
        return LetStatement{};
    }

    parser.next_token();
    stmt.value = parser.parse_expr(LOWEST);

    // TODO: Handle function literal piece here
    // if fl, ok := stmt.Value.(*ast.FunctionLiteral); ok {
	// 	fl.Name = stmt.Name.Value
    // }
    
    if parser.peek_token_type_is(TokenType::SEMICOLON) {
        parser.next_token();
    }

    Box::new(stmt)
}

fn parse_const_statement(parser: &mut Parser) -> Box<dyn Statement> {}

fn parse_return_statement(parser: &mut Parser) -> Box<dyn Statement> {}

fn parse_expr_stat(parser: &mut Parser) -> Box<dyn Statement> {}
