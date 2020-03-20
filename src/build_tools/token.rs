use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENTIFIER,
    INTEGER,
    STRING,

    EQUAL,
    PLUS,
    PLUS_PLUS,
    MINUS,
    MINUS_MINUS,
    STAR,
    SLASH,
    MOD,
    BANG,
    EQUAL_EQUAL,
    LESS,
    LESS_EQUAL,
    GREATER,
    GREATER_EQUAL,
    BANG_EQUAL,
    AND,
    OR,

    COMMA,
    COLON,
    SEMICOLON,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    LEFT_BRACKET,
    RIGHT_BRACKET,

    FUNCTION,
    LET,
    CONST,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,

    NONE,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
}

struct Keywords;

impl Keywords {
    pub fn all() -> HashMap<String, TokenType> {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();

        keywords.insert(String::from("func"), TokenType::FUNCTION);
        keywords.insert(String::from("let"), TokenType::LET);
        keywords.insert(String::from("const"), TokenType::CONST);
        keywords.insert(String::from("true"), TokenType::TRUE);
        keywords.insert(String::from("false"), TokenType::FALSE);
        keywords.insert(String::from("if"), TokenType::IF);
        keywords.insert(String::from("else"), TokenType::ELSE);
        keywords.insert(String::from("return"), TokenType::RETURN);

        keywords
    }
}

pub fn look_up_identifier(identifier: &String) -> TokenType {
    if Keywords::all().contains_key(identifier) {
        return Keywords::all()[identifier];
    }

    return TokenType::IDENTIFIER;
}
