use crate::build_tools::token::*;
use crate::build_tools::ast::{Expression};

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

        precendences.insert(EQUAL_EQUAL.to_string(), EQUALS);
        precendences.insert(BANG_EQUAL.to_string(), EQUALS);
        precendences.insert(LESS.to_string(), LESS_GREATER);
        precendences.insert(GREATER.to_string(), LESS_GREATER);
        precendences.insert(LESS_EQUAL.to_string(), LESS_GREATER);
        precendences.insert(LESS_GREATER.to_string(), LESS_GREATER);
        precendences.insert(PLUS.to_string(), SUM);
        precendences.insert(MINUS.to_string(), SUM);
        precendences.insert(SLASH.to_string(), PRODUCT);
        precendences.insert(STAR.to_string(), PRODUCT);
        precendences.insert(MOD.to_string(), MODULO);
        precendences.insert(AND.to_string(), LOGICAL);
        precendences.insert(OR.to_string(), LOGICAL);
        precendences.insert(LEFT_PAREN.to_string(), CALL);
        precendences.insert(LEFT_BRACKET.to_string(), INDEX);

        precendences
    }
}

type PrefixParseFunc = fn() -> dyn Expression;
type InfixParseFunc = fn(expr: dyn Expression) -> dyn Expression;
type PostfixParseFunc = fn() -> dyn Expression;
