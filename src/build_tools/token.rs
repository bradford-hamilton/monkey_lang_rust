use std::collections::HashMap;

static ILLEGAL: &str = "ILLEGAL";
static EOF: &str = "EOF";

static IDENTIFIER: &str = "IDENTIFIER";
static INTEGER: &str = "INTEGER";
static STRING: &str = "STRING";

static EQUAL: &str = "=";
static PLUS: &str = "+";
static PLUS_PLUS: &str = "++";
static MINUS: &str = "-";
static MINUS_MINUS: &str = "--";
static STAR: &str = "*";
static SLASH: &str = "/";
static MOD: &str = "%";
static BANG: &str = "!";
static EQUAL_EQUAL: &str = "==";
static LESS: &str = "<";
static LESS_EQUAL: &str = "<=";
static GREATER: &str = ">";
static GREATER_EQUAL: &str = ">=";
static BANG_EQUAL: &str = "!=";
static AND: &str = "&&";
static OR: &str = "||";

static COMMA: &str = ",";
static COLON: &str = ":";
static SEMICOLON: &str = ";";
static LEFT_PAREN: &str = "(";
static RIGHT_PAREN: &str = ")";
static LEFT_BRACE: &str = "{";
static RIGHT_BRACE: &str = "}";
static LEFT_BRACKET: &str = "[";
static RIGHT_BRACKET: &str = "]";

static FUNCTION: &str = "FUNCTION";
static LET: &str = "LET";
static CONST: &str = "CONST";
static TRUE: &str = "TRUE";
static FALSE: &str = "FALSE";
static IF: &str = "IF";
static ELSE: &str = "ELSE";
static RETURN: &str = "RETURN";

type TokenType = String;

struct Token {
    token_type: TokenType,
    literal: String,
    line: usize,
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
