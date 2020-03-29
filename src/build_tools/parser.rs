use crate::build_tools::ast;
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

type PrefixParseFunc = fn(parser: &mut Parser) -> Box<dyn ast::Expression>;
type InfixParseFunc =
    fn(parser: &mut Parser, expr: Box<dyn ast::Expression>) -> Box<dyn ast::Expression>;
type PostfixParseFunc = fn(parser: &mut Parser) -> Box<dyn ast::Expression>;

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
        parser.register_prefix(TokenType::BANG, parse_prefix_expr);
        parser.register_prefix(TokenType::MINUS, parse_prefix_expr);
        parser.register_prefix(TokenType::TRUE, parse_boolean);
        parser.register_prefix(TokenType::FALSE, parse_boolean);
        parser.register_prefix(TokenType::LEFT_PAREN, parse_grouped_expr);
        parser.register_prefix(TokenType::IF, parse_if_expr);
        parser.register_prefix(TokenType::FUNCTION, parse_function_literal);
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

    fn parse_expr(&mut self, precedence: usize) -> Option<Box<dyn ast::Expression>> {
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

    fn parse_block_stmt(&mut self) -> ast::BlockStatement {
        let mut block = ast::BlockStatement {
            token: self.current_token.clone(),
            statements: vec![],
        };

        self.next_token();

        while !self.current_token_type_is(TokenType::RIGHT_BRACE)
            && !self.current_token_type_is(TokenType::EOF)
        {
            let stmt = match self.parse_stmt() {
                Some(stmt) => {
                    block.statements.push(stmt);
                }
                _ => {
                    return ast::BlockStatement {
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

    fn parse_stmt(&mut self) -> Option<Box<dyn ast::Statement>> {
        let ret = match self.current_token.token_type {
            TokenType::LET => parse_let_stmt(self),
            TokenType::CONST => parse_const_stmt(self),
            TokenType::RETURN => parse_return_stmt(self),
            _ => parse_expr_stmt(self),
        };

        Some(ret)
    }

    fn parse_function_params(&mut self) -> Vec<ast::Identifier> {
        let mut identifiers: Vec<ast::Identifier> = vec![];

        if self.peek_token_type_is(TokenType::RIGHT_PAREN) {
            self.next_token();
            return identifiers;
        }

        self.next_token();

        identifiers.push(ast::Identifier{
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        });

        while self.peek_token_type_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();

            identifiers.push(ast::Identifier{
                token: self.current_token.clone(),
                value: self.current_token.literal.clone(),
            })
        }

        if !self.expect_peek_type(TokenType::RIGHT_PAREN) {
            return vec![];
        }

        identifiers
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

fn parse_identifier(parser: &mut Parser) -> Box<dyn ast::Expression> {
    let contains_key = parser
        .postfix_parse_funcs
        .contains_key(&parser.peek_token.token_type);

    if contains_key {
        let postfix = parser.postfix_parse_funcs[&parser.peek_token.token_type];
        parser.next_token();
        return postfix(parser);
    }

    Box::new(ast::Identifier {
        token: parser.current_token.clone(),
        value: parser.current_token.literal.clone(),
    })
}

fn parse_integer_literal(parser: &mut Parser) -> Box<dyn ast::Expression> {
    Box::new(ast::IntegerLiteral {
        token: parser.current_token.clone(),
        value: parser.current_token.literal.parse::<usize>().unwrap(),
    })
}

fn parse_prefix_expr(parser: &mut Parser) -> Box<dyn ast::Expression> {
    let mut expr = ast::PrefixExpression {
        token: parser.current_token.clone(),
        operator: parser.current_token.literal.clone(),
        right: Box::new(ast::ZeroValueExpression {}),
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
            Box::new(ast::ZeroValueExpression {})
        }
    };

    Box::new(expr)
}

fn parse_boolean(parser: &mut Parser) -> Box<dyn ast::Expression> {
    Box::new(ast::Boolean {
        token: parser.current_token.clone(),
        value: parser.current_token_type_is(TokenType::TRUE),
    })
}

fn parse_grouped_expr(parser: &mut Parser) -> Box<dyn ast::Expression> {
    parser.next_token();

    let expr = match parser.parse_expr(LOWEST) {
        Some(expr) => expr,
        _ => {
            let msg = format!(
                "Line {}: Failed to parse expression {}.",
                parser.current_token.line, parser.current_token.literal
            );
            parser.errors.push(msg);
            Box::new(ast::ZeroValueExpression {})
        }
    };

    if !parser.expect_peek_type(TokenType::RIGHT_PAREN) {
        return Box::new(ast::ZeroValueExpression {});
    }

    expr
}

fn parse_if_expr(parser: &mut Parser) -> Box<dyn ast::Expression> {
    let mut expr = ast::IfExpression {
        token: parser.current_token.clone(),
        condition: Box::new(ast::ZeroValueExpression {}),
        consequence: ast::BlockStatement {
            token: parser.current_token.clone(),
            statements: vec![],
        },
        alternative: ast::BlockStatement {
            token: parser.current_token.clone(),
            statements: vec![],
        },
    };

    if !parser.expect_peek_type(TokenType::LEFT_PAREN) {
        return Box::new(ast::ZeroValueExpression {});
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
            Box::new(ast::ZeroValueExpression {})
        }
    };

    if !parser.expect_peek_type(TokenType::RIGHT_PAREN) {
        return Box::new(ast::ZeroValueExpression {});
    }

    if !parser.expect_peek_type(TokenType::LEFT_BRACE) {
        return Box::new(ast::ZeroValueExpression {});
    }

    expr.consequence = parser.parse_block_stmt();

    if parser.peek_token_type_is(TokenType::ELSE) {
        parser.next_token();

        if !parser.expect_peek_type(TokenType::LEFT_BRACE) {
            return Box::new(ast::ZeroValueExpression {});
        }

        expr.alternative = parser.parse_block_stmt();
    }

    Box::new(expr)
}

fn parse_let_stmt(parser: &mut Parser) -> Box<dyn ast::Statement> {
    let zero_value_token: Token = Token {
        token_type: TokenType::NONE,
        literal: "".to_owned(),
        line: 0,
    };
    let zero_value_identifier: ast::Identifier = ast::Identifier {
        token: zero_value_token,
        value: "".to_owned(),
    };
    let mut stmt = ast::LetStatement {
        token: parser.current_token.clone(),
        name: zero_value_identifier,
        value: Box::new(ast::ZeroValueExpression {}),
    };

    if !parser.expect_peek_type(TokenType::IDENTIFIER) {
        return Box::new(ast::ZeroValueStatement {});
    }

    stmt.name = ast::Identifier {
        token: parser.current_token.clone(),
        value: parser.current_token.literal.clone(),
    };

    if !parser.expect_peek_type(TokenType::EQUAL) {
        return Box::new(ast::ZeroValueStatement {});
    }

    parser.next_token();

    stmt.value = match parser.parse_expr(LOWEST) {
        Some(expr) => expr,
        _ => {
            let msg = format!(
                "Line {}: Failed to parse expression {}.",
                parser.current_token.line, parser.current_token.literal
            );
            parser.errors.push(msg);
            Box::new(ast::ZeroValueExpression {})
        }
    };

    // TODO: Handle function literal piece here
    // if fl, ok := stmt.Value.(*ast.FunctionLiteral); ok {
    // 	fl.Name = stmt.Name.Value
    // }

    if parser.peek_token_type_is(TokenType::SEMICOLON) {
        parser.next_token();
    }

    Box::new(stmt)
}

fn parse_const_stmt(parser: &mut Parser) -> Box<dyn ast::Statement> {
    let zero_value_token: Token = Token {
        token_type: TokenType::NONE,
        literal: "".to_owned(),
        line: 0,
    };
    let zero_value_identifier: ast::Identifier = ast::Identifier {
        token: zero_value_token,
        value: "".to_owned(),
    };
    let mut stmt = ast::ConstStatement {
        token: parser.current_token.clone(),
        name: zero_value_identifier,
        value: Box::new(ast::ZeroValueExpression {}),
    };

    if !parser.expect_peek_type(TokenType::IDENTIFIER) {
        return Box::new(ast::ZeroValueStatement {});
    }

    stmt.name = ast::Identifier {
        token: parser.current_token.clone(),
        value: parser.current_token.literal.clone(),
    };

    if !parser.expect_peek_type(TokenType::EQUAL) {
        return Box::new(ast::ZeroValueStatement {});
    }

    parser.next_token();

    stmt.value = match parser.parse_expr(LOWEST) {
        Some(expr) => expr,
        _ => {
            let msg = format!(
                "Line {}: Failed to parse expression {}.",
                parser.current_token.line, parser.current_token.literal
            );
            parser.errors.push(msg);
            Box::new(ast::ZeroValueExpression {})
        }
    };

    // TODO: Handle function literal piece here
    // if fl, ok := stmt.Value.(*ast.FunctionLiteral); ok {
    // 	fl.Name = stmt.Name.Value
    // }

    if parser.peek_token_type_is(TokenType::SEMICOLON) {
        parser.next_token();
    }

    Box::new(stmt)
}

fn parse_return_stmt(parser: &mut Parser) -> Box<dyn ast::Statement> {
    let mut stmt = ast::ReturnStatement {
        token: parser.current_token.clone(),
        return_value: Box::new(ast::ZeroValueExpression {}),
    };

    parser.next_token();

    stmt.return_value = match parser.parse_expr(LOWEST) {
        Some(expr) => expr,
        _ => {
            let msg = format!(
                "Line {}: Failed to parse expression {}.",
                parser.current_token.line, parser.current_token.literal,
            );
            parser.errors.push(msg);
            Box::new(ast::ZeroValueExpression {})
        }
    };

    if parser.peek_token_type_is(TokenType::SEMICOLON) {
        parser.next_token();
    }

    Box::new(stmt)
}

fn parse_expr_stmt(parser: &mut Parser) -> Box<dyn ast::Statement> {
    let mut stmt = ast::ExpressionStatement {
        token: parser.current_token.clone(),
        expression: Box::new(ast::ZeroValueExpression {}),
    };

    stmt.expression = match parser.parse_expr(LOWEST) {
        Some(expr) => expr,
        _ => {
            let msg = format!(
                "Line {}: Failed to parse expression {}.",
                parser.current_token.line, parser.current_token.literal,
            );
            parser.errors.push(msg);
            Box::new(ast::ZeroValueExpression {})
        }
    };

    if parser.peek_token_type_is(TokenType::SEMICOLON) {
        parser.next_token();
    }

    Box::new(stmt)
}

fn parse_function_literal(parser: &mut Parser) -> Box<dyn ast::Expression> {
    let zero_value_token: Token = Token {
        token_type: TokenType::NONE,
        literal: "".to_owned(),
        line: 0,
    };
    let mut lit = ast::FunctionLiteral{
        token: parser.current_token.clone(),
        parameters: vec![],
        body: ast::BlockStatement{
            token: zero_value_token,
            statements: vec![],
        },
        name: "".to_owned(),
    };

    if !parser.expect_peek_type(TokenType::LEFT_PAREN) {
        return Box::new(ast::ZeroValueExpression {});
    }

    lit.parameters = parser.parse_function_params();

    if !parser.expect_peek_type(TokenType::LEFT_BRACE) {
        return Box::new(ast::ZeroValueExpression {});
    }

    lit.body = parser.parse_block_stmt();

    Box::new(lit)
}
