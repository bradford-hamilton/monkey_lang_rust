use crate::build_tools::token::*;

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

    pub fn next_token(mut self) -> Token {
        let mut token = Token {
            token_type: TokenType::NONE,
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
                        token_type: TokenType::EQUAL_EQUAL,
                        literal: ch.to_string() + &self.current_char.to_string(),
                        line: self.line,
                    };
                } else {
                    token = new_token(TokenType::EQUAL, self.line, self.current_char.to_string());
                }
            }
            '+' => {
                if self.peek() == '+' {
                    let ch = self.current_char;

                    self.read_char();

                    token = Token {
                        token_type: TokenType::PLUS_PLUS,
                        literal: ch.to_string() + &self.current_char.to_string(),
                        line: self.line,
                    }
                } else {
                    token = new_token(TokenType::PLUS, self.line, self.current_char.to_string());
                }
            }
            '-' => {
                if self.peek() == '-' {
                    let ch = self.current_char;

                    self.read_char();

                    token = Token {
                        token_type: TokenType::MINUS_MINUS,
                        literal: ch.to_string() + &self.current_char.to_string(),
                        line: self.line,
                    }
                } else {
                    token = new_token(TokenType::MINUS, self.line, self.current_char.to_string())
                }
            }
            '!' => {
                if self.peek() == '=' {
                    let ch = self.current_char;

                    self.read_char();

                    token = Token {
                        token_type: TokenType::BANG_EQUAL,
                        literal: ch.to_string() + &self.current_char.to_string(),
                        line: self.line,
                    }
                } else {
                    token = new_token(TokenType::BANG, self.line, self.current_char.to_string());
                }
            }
            '*' => {
                token = new_token(TokenType::STAR, self.line, self.current_char.to_string());
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

                token = new_token(TokenType::SLASH, self.line, self.current_char.to_string());
            }
            '%' => {
                token = new_token(TokenType::MOD, self.line, self.current_char.to_string());
            }
            '<' => {
                if self.peek() == '=' {
                    let ch = self.current_char;

                    self.read_char();

                    let literal = ch.to_string() + &self.current_char.to_string();

                    token = new_token(TokenType::LESS_EQUAL, self.line, literal);
                } else {
                    token = new_token(TokenType::LESS, self.line, self.current_char.to_string());
                }
            }
            '>' => {
                if self.peek() == '=' {
                    let ch = self.current_char;

                    self.read_char();

                    let literal = ch.to_string() + &self.current_char.to_string();

                    token = new_token(TokenType::GREATER_EQUAL, self.line, literal);
                } else {
                    token = new_token(
                        TokenType::GREATER,
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

                    token = new_token(TokenType::AND, self.line, literal);
                }
            }
            '|' => {
                if self.peek() == '|' {
                    let ch = self.current_char;

                    self.read_char();

                    let literal = ch.to_string() + &self.current_char.to_string();

                    token = new_token(TokenType::OR, self.line, literal);
                }
            }
            ',' => {
                token = new_token(TokenType::COMMA, self.line, self.current_char.to_string());
            }
            ':' => {
                token = new_token(TokenType::COLON, self.line, self.current_char.to_string());
            }
            ';' => {
                token = new_token(
                    TokenType::SEMICOLON,
                    self.line,
                    self.current_char.to_string(),
                );
            }
            '(' => {
                token = new_token(
                    TokenType::LEFT_PAREN,
                    self.line,
                    self.current_char.to_string(),
                );
            }
            ')' => {
                token = new_token(
                    TokenType::RIGHT_PAREN,
                    self.line,
                    self.current_char.to_string(),
                );
            }
            '{' => {
                token = new_token(
                    TokenType::LEFT_BRACE,
                    self.line,
                    self.current_char.to_string(),
                );
            }
            '}' => {
                token = new_token(
                    TokenType::RIGHT_BRACE,
                    self.line,
                    self.current_char.to_string(),
                );
            }
            '[' => {
                token = new_token(
                    TokenType::LEFT_BRACKET,
                    self.line,
                    self.current_char.to_string(),
                );
            }
            ']' => {
                token = new_token(
                    TokenType::RIGHT_BRACKET,
                    self.line,
                    self.current_char.to_string(),
                );
            }
            '"' => {
                token.token_type = TokenType::STRING;
                token.literal = self.read_string();
                token.line = self.line;
            }
            '\0' => {
                token.literal = String::from("");
                token.token_type = TokenType::EOF;
                token.line = self.line;
            }
            _ => {
                if is_letter(self.current_char) {
                    token.literal = self.read_identifier();
                    token.token_type = look_up_identifier(&token.literal);
                    token.line = self.line;
                } else if is_integer(self.current_char) {
                    token.literal = self.read_integer();
                    token.token_type = TokenType::INTEGER;
                    token.line = self.line;
                } else {
                    token = new_token(
                        TokenType::ILLEGAL,
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
