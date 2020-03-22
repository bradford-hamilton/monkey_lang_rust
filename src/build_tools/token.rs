use std::collections::HashMap;
use std::fmt;

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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            ILLEGAL => "ILLEGAL",
            EOF => "EOF",
            IDENTIFIER => "IDENTIFIER",
            INTEGER => "INTEGER",
            STRING => "STRING",
            EQUAL => "EQUAL",
            PLUS => "PLUS",
            PLUS_PLUS => "PLUS_PLUS",
            MINUS => "MINUS",
            MINUS_MINUS => "MINUS_MINUS",
            STAR => "STAR",
            SLASH => "SLASH",
            MOD => "MOD",
            BANG => "BANG",
            EQUAL_EQUAL => "EQUAL_EQUAL",
            LESS => "LESS",
            LESS_EQUAL => "LESS_EQUAL",
            GREATER => "GREATER",
            GREATER_EQUAL => "GREATER_EQUAL",
            BANG_EQUAL => "BANG_EQUAL",
            AND => "AND",
            OR => "OR",
            COMMA => "COMMA",
            COLON => "COLON",
            SEMICOLON => "SEMICOLON",
            LEFT_PAREN => "LEFT_PAREN",
            RIGHT_PAREN => "RIGHT_PAREN",
            LEFT_BRACE => "LEFT_BRACE",
            RIGHT_BRACE => "RIGHT_BRACE",
            LEFT_BRACKET => "LEFT_BRACKET",
            RIGHT_BRACKET => "RIGHT_BRACKET",
            FUNCTION => "FUNCTION",
            LET => "LET",
            CONST => "CONST",
            TRUE => "TRUE",
            FALSE => "FALSE",
            IF => "IF",
            ELSE => "ELSE",
            RETURN => "RETURN",
            NONE => "NONE",
        };
        write!(f, "{}", printable)
    }
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
