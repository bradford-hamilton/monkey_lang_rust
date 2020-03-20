use std::collections::HashMap;

/// Monkey's token types
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenType {
    /// Token/character we don't know about
    ILLEGAL,
    /// End of file
    EOF,

    /// Identifiers & literals
    IDENTIFIER, // add, foobar, x, y, ...
    INTEGER,
    STRING,

    /// Operators
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

    /// Delimiters
    COMMA,
    COLON,
    SEMICOLON,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    LEFT_BRACKET,
    RIGHT_BRACKET,

    /// Keywords
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

/// Token is a struct representing a Monkey token - holds a type and a literal
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

        keywords.insert("func".to_owned(), TokenType::FUNCTION);
        keywords.insert("let".to_owned(), TokenType::LET);
        keywords.insert("const".to_owned(), TokenType::CONST);
        keywords.insert("true".to_owned(), TokenType::TRUE);
        keywords.insert("false".to_owned(), TokenType::FALSE);
        keywords.insert("if".to_owned(), TokenType::IF);
        keywords.insert("else".to_owned(), TokenType::ELSE);
        keywords.insert("return".to_owned(), TokenType::RETURN);

        keywords
    }
}

/// look_up_identifier checks our keywords map for the scanned keyword. If it finds one, then
/// the keyword's type is returned. If not, the user defined IDENTIFIER is returned
pub fn look_up_identifier(identifier: &String) -> TokenType {
    if Keywords::all().contains_key(identifier) {
        return Keywords::all()[identifier];
    }
    TokenType::IDENTIFIER
}
