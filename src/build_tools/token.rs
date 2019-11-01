use std::collections::HashMap;

pub static ILLEGAL: &str = "ILLEGAL";
pub static EOF: &str = "EOF";

pub static IDENTIFIER: &str = "IDENTIFIER";
pub static INTEGER: &str = "INTEGER";
pub static STRING: &str = "STRING";

pub static EQUAL: &str = "=";
pub static PLUS: &str = "+";
pub static PLUS_PLUS: &str = "++";
pub static MINUS: &str = "-";
pub static MINUS_MINUS: &str = "--";
pub static STAR: &str = "*";
pub static SLASH: &str = "/";
pub static MOD: &str = "%";
pub static BANG: &str = "!";
pub static EQUAL_EQUAL: &str = "==";
pub static LESS: &str = "<";
pub static LESS_EQUAL: &str = "<=";
pub static GREATER: &str = ">";
pub static GREATER_EQUAL: &str = ">=";
pub static BANG_EQUAL: &str = "!=";
pub static AND: &str = "&&";
pub static OR: &str = "||";

pub static COMMA: &str = ",";
pub static COLON: &str = ":";
pub static SEMICOLON: &str = ";";
pub static LEFT_PAREN: &str = "(";
pub static RIGHT_PAREN: &str = ")";
pub static LEFT_BRACE: &str = "{";
pub static RIGHT_BRACE: &str = "}";
pub static LEFT_BRACKET: &str = "[";
pub static RIGHT_BRACKET: &str = "]";

pub static FUNCTION: &str = "FUNCTION";
pub static LET: &str = "LET";
pub static CONST: &str = "CONST";
pub static TRUE: &str = "TRUE";
pub static FALSE: &str = "FALSE";
pub static IF: &str = "IF";
pub static ELSE: &str = "ELSE";
pub static RETURN: &str = "RETURN";

pub type TokenType = String;

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
}

struct Keywords;

impl Keywords {
    pub fn all() -> HashMap<String, TokenType> {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();

        keywords.insert(String::from("func"), FUNCTION.to_string() as TokenType);
        keywords.insert(String::from("let"), LET.to_string() as TokenType);
        keywords.insert(String::from("const"), CONST.to_string() as TokenType);
        keywords.insert(String::from("true"), TRUE.to_string() as TokenType);
        keywords.insert(String::from("false"), FALSE.to_string() as TokenType);
        keywords.insert(String::from("if"), IF.to_string() as TokenType);
        keywords.insert(String::from("else"), ELSE.to_string() as TokenType);
        keywords.insert(String::from("return"), RETURN.to_string() as TokenType);

        keywords
    }
}

pub fn look_up_identifier(identifier: String) -> TokenType {
    if Keywords::all().contains_key(&identifier) {
        return Keywords::all()[&identifier].to_string();
    }

    return IDENTIFIER.to_string();
}
