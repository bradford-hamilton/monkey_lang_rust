use crate::build_tools::token::{
    look_up_identifier, Token, TokenType, AND, BANG, BANG_EQUAL, COLON, COMMA, EOF, EQUAL,
    EQUAL_EQUAL, GREATER, GREATER_EQUAL, ILLEGAL, INTEGER, LEFT_BRACE, LEFT_BRACKET, LEFT_PAREN,
    LESS, LESS_EQUAL, MINUS, MINUS_MINUS, MOD, OR, PLUS, PLUS_PLUS, RIGHT_BRACE, RIGHT_BRACKET,
    RIGHT_PAREN, SEMICOLON, SLASH, STAR, STRING,
};

pub struct Lexer {
    input: Vec<char>,
    current_char: char,
    position: usize,
    read_position: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            current_char: 'a',
            position: 0,
            read_position: 0,
            line: 0,
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_string(&mut self) -> String {
        let position: usize = self.position + 1;

        loop {
            self.read_char();
            if self.current_char == '"' || self.current_char == '\0' {
                break;
            }
        }

        let string: String = self.input[position..self.position].iter().collect();

        string
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while is_letter(self.current_char) {
            self.read_char();
        }

        let string: String = self.input[position..self.position].iter().collect();

        string
    }

    fn read_integer(&mut self) -> String {
        let position = self.position;

        while is_integer(self.current_char) {
            self.read_char();
        }

        let string: String = self.input[position..self.position].iter().collect();

        string
    }

    fn skip_whitespace(&mut self) {
        while self.current_char == ' '
            || self.current_char == '\t'
            || self.current_char == '\n'
            || self.current_char == '\r'
        {
            if self.current_char == '\n' {
                self.line += 1;
            }

            self.read_char();
        }
    }

    fn skip_single_line_comment(&mut self) {
        while self.current_char != '\n' && self.current_char != '\0' {
            self.read_char();
        }

        self.skip_whitespace();
    }

    fn skip_multi_line_comment(&mut self) {
        let mut end_found = false;

        while !end_found {
            if self.current_char == '\0' {
                end_found = true;
            }

            if self.current_char == '*' && self.peek() == '/' {
                end_found = true;
                self.read_char();
            }

            self.read_char();
        }

        self.skip_whitespace();
    }

    fn peek(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        }

        return self.input[self.read_position];
    }

    fn next_token(mut self) -> Token {
        let mut token = Token {
            token_type: String::from(""),
            literal: String::from(""),
            line: 0,
        };

        self.skip_whitespace();

        match self.current_char {
            '=' => {
                if self.peek() == '=' {
                    let ch = self.current_char;

                    self.read_char();

                    token = Token {
                        token_type: EQUAL_EQUAL.to_string(),
                        literal: ch.to_string() + &self.current_char.to_string(),
                        line: self.line,
                    };
                } else {
                    token = new_token(EQUAL.to_string(), self.line, self.current_char.to_string());
                }
            }
            '+' => {
                if self.peek() == '+' {
                    let ch = self.current_char;

                    self.read_char();

                    token = Token {
                        token_type: PLUS_PLUS.to_string(),
                        literal: ch.to_string() + &self.current_char.to_string(),
                        line: self.line,
                    }
                } else {
                    token = new_token(PLUS.to_string(), self.line, self.current_char.to_string());
                }
            }
            '-' => {
                if self.peek() == '-' {
                    let ch = self.current_char;

                    self.read_char();

                    token = Token {
                        token_type: MINUS_MINUS.to_string(),
                        literal: ch.to_string() + &self.current_char.to_string(),
                        line: self.line,
                    }
                } else {
                    token = new_token(MINUS.to_string(), self.line, self.current_char.to_string())
                }
            }
            '!' => {
                if self.peek() == '=' {
                    let ch = self.current_char;

                    self.read_char();

                    token = Token {
                        token_type: BANG_EQUAL.to_string(),
                        literal: ch.to_string() + &self.current_char.to_string(),
                        line: self.line,
                    }
                } else {
                    token = new_token(BANG.to_string(), self.line, self.current_char.to_string());
                }
            }
            '*' => {
                token = new_token(STAR.to_string(), self.line, self.current_char.to_string());
            }
            '/' => {
                if self.peek() == '/' {
                    self.skip_single_line_comment();
                    return self.next_token();
                }

                if self.peek() == '*' {
                    self.skip_multi_line_comment();
                    return self.next_token();
                }

                token = new_token(SLASH.to_string(), self.line, self.current_char.to_string());
            }
            '%' => {
                token = new_token(MOD.to_string(), self.line, self.current_char.to_string());
            }
            '<' => {
                if self.peek() == '=' {
                    let ch = self.current_char;

                    self.read_char();

                    let literal = ch.to_string() + &self.current_char.to_string();

                    token = new_token(LESS_EQUAL.to_string(), self.line, literal);
                } else {
                    token = new_token(LESS.to_string(), self.line, self.current_char.to_string());
                }
            }
            '>' => {
                if self.peek() == '=' {
                    let ch = self.current_char;

                    self.read_char();

                    let literal = ch.to_string() + &self.current_char.to_string();

                    token = new_token(GREATER_EQUAL.to_string(), self.line, literal);
                } else {
                    token = new_token(
                        GREATER.to_string(),
                        self.line,
                        self.current_char.to_string(),
                    );
                }
            }
            '&' => {
                if self.peek() == '&' {
                    let ch = self.current_char;

                    self.read_char();

                    let literal = ch.to_string() + &self.current_char.to_string();

                    token = new_token(AND.to_string(), self.line, literal);
                }
            }
            '|' => {
                if self.peek() == '|' {
                    let ch = self.current_char;

                    self.read_char();

                    let literal = ch.to_string() + &self.current_char.to_string();

                    token = new_token(OR.to_string(), self.line, literal);
                }
            }
            ',' => {
                token = new_token(COMMA.to_string(), self.line, self.current_char.to_string());
            }
            ':' => {
                token = new_token(COLON.to_string(), self.line, self.current_char.to_string());
            }
            ';' => {
                token = new_token(
                    SEMICOLON.to_string(),
                    self.line,
                    self.current_char.to_string(),
                );
            }
            '(' => {
                token = new_token(
                    LEFT_PAREN.to_string(),
                    self.line,
                    self.current_char.to_string(),
                );
            }
            ')' => {
                token = new_token(
                    RIGHT_PAREN.to_string(),
                    self.line,
                    self.current_char.to_string(),
                );
            }
            '{' => {
                token = new_token(
                    LEFT_BRACE.to_string(),
                    self.line,
                    self.current_char.to_string(),
                );
            }
            '}' => {
                token = new_token(
                    RIGHT_BRACE.to_string(),
                    self.line,
                    self.current_char.to_string(),
                );
            }
            '[' => {
                token = new_token(
                    LEFT_BRACKET.to_string(),
                    self.line,
                    self.current_char.to_string(),
                );
            }
            ']' => {
                token = new_token(
                    RIGHT_BRACKET.to_string(),
                    self.line,
                    self.current_char.to_string(),
                );
            }
            '"' => {
                token.token_type = STRING.to_string();
                token.literal = self.read_string();
                token.line = self.line;
            }
            '\0' => {
                token.literal = String::from("");
                token.token_type = EOF.to_string();
                token.line = self.line;
            }
            _ => {
                if is_letter(self.current_char) {
                    token.literal = self.read_identifier();
                    token.token_type = look_up_identifier(&token.literal);
                    token.line = self.line;
                } else if is_integer(self.current_char) {
                    token.literal = self.read_integer();
                    token.token_type = INTEGER.to_string();
                    token.line = self.line;
                } else {
                    token = new_token(
                        ILLEGAL.to_string(),
                        self.line,
                        self.current_char.to_string(),
                    )
                }
            }
        }

        self.read_char();

        token
    }
}

fn new_token(token_type: TokenType, line: usize, literal: String) -> Token {
    Token {
        token_type,
        literal,
        line,
    }
}

fn is_letter(character: char) -> bool {
    'a' <= character && character <= 'z' || 'A' <= character && character <= 'Z' || character == '_'
}

fn is_integer(character: char) -> bool {
    '0' <= character && character <= '9'
}
