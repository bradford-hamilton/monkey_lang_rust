use crate::build_tools::token::{Token, TokenType, EQUAL, EQUAL_EQUAL, INTEGER, ILLEGAL, look_up_identifier};

pub struct Lexer {
    input: Vec<char>,
    current_char: char,
    position: usize,
    read_position: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input.chars().collect(),
            current_char: 'a',
            position: 0,
            read_position: 0,
            line: 0,
        };

        l.read_char();

        l
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

    fn skip_multiline_comment(&mut self) {
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

    fn next_token(&mut self) -> Token {
        let mut t = Token {
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

                    let literal = ch.to_string() + &self.current_char.to_string();

                    t = Token {
                        token_type: EQUAL_EQUAL.to_string(),
                        literal,
                        line: self.line,
                    }
                } else {
                    t = new_token(EQUAL.to_string(), self.line, self.current_char);
                }
            }
            // case '+':
            //     if l.peek() == '+' {
            //         ch := l.char
            //         l.readChar()
            //         t = token.Token{
            //             Type:    token.PlusPlus,
            //             Literal: string(ch) + string(l.char),
            //             Line:    l.line,
            //         }
            //     } else {
            //         t = newToken(token.Plus, l.line, l.char)
            //     }
            // case '-':
            //     if l.peek() == '-' {
            //         ch := l.char
            //         l.readChar()
            //         t = token.Token{
            //             Type:    token.MinusMinus,
            //             Literal: string(ch) + string(l.char),
            //             Line:    l.line,
            //         }
            //     } else {
            //         t = newToken(token.Minus, l.line, l.char)
            //     }
            // case '!':
            //     if l.peek() == '=' {
            //         ch := l.char
            //         l.readChar()
            //         literal := string(ch) + string(l.char)
            //         t = token.Token{
            //             Type:    token.BangEqual,
            //             Literal: literal,
            //             Line:    l.line,
            //         }
            //     } else {
            //         t = newToken(token.Bang, l.line, l.char)
            //     }
            // case '*':
            //     t = newToken(token.Star, l.line, l.char)
            // case '/':
            //     if l.peek() == '/' {
            //         l.skipSingleLineComment()
            //         return l.NextToken()
            //     }
            //     if l.peek() == '*' {
            //         l.skipMultiLineComment()
            //         return l.NextToken()
            //     }
            //     t = newToken(token.Slash, l.line, l.char)
            // case '%':
            //     t = newToken(token.Mod, l.line, l.char)
            // case '<':
            //     if l.peek() == '=' {
            //         ch := l.char
            //         l.readChar()
            //         t = newToken(token.LessEqual, l.line, ch, l.char)
            //     } else {
            //         t = newToken(token.Less, l.line, l.char)
            //     }
            // case '>':
            //     if l.peek() == '=' {
            //         ch := l.char
            //         l.readChar()
            //         t = newToken(token.GreaterEqual, l.line, ch, l.char)
            //     } else {
            //         t = newToken(token.Greater, l.line, l.char)
            //     }
            // case '&':
            //     if l.peek() == '&' {
            //         ch := l.char
            //         l.readChar()
            //         t = newToken(token.And, l.line, ch, l.char)
            //     }
            // case '|':
            //     if l.peek() == '|' {
            //         ch := l.char
            //         l.readChar()
            //         t = newToken(token.Or, l.line, ch, l.char)
            //     }
            // case ',':
            //     t = newToken(token.Comma, l.line, l.char)
            // case ':':
            //     t = newToken(token.Colon, l.line, l.char)
            // case ';':
            //     t = newToken(token.Semicolon, l.line, l.char)
            // case '(':
            //     t = newToken(token.LeftParen, l.line, l.char)
            // case ')':
            //     t = newToken(token.RightParen, l.line, l.char)
            // case '{':
            //     t = newToken(token.LeftBrace, l.line, l.char)
            // case '}':
            //     t = newToken(token.RightBrace, l.line, l.char)
            // case '[':
            //     t = newToken(token.LeftBracket, l.line, l.char)
            // case ']':
            //     t = newToken(token.RightBracket, l.line, l.char)
            // case '"':
            //     t.Type = token.String
            //     t.Literal = l.readString()
            //     t.Line = l.line
            // case 0:
            //     t.Literal = ""
            //     t.Type = token.EOF
            //     t.Line = l.line
            _ => {
                if is_letter(self.current_char) {
                    t.literal = self.read_identifier();
                    t.token_type = look_up_identifier(t.literal);
                    t.line = self.line;
                } else if is_integer(self.current_char) {
                    t.literal = self.read_integer();
                    t.token_type = INTEGER.to_string();
                    t.line = self.line;
                } else {
                    t = new_token(ILLEGAL.to_string(), self.line, self.current_char)
                }
            }
        }

        self.read_char();

        // TODO: figure out moved variable "t"
        return t;
    }
}

fn new_token(token_type: TokenType, line: usize, literal: char) -> Token {
    Token {
        token_type,
        literal: literal.to_string(),
        line,
    }
}

fn is_letter(character: char) -> bool {
    'a' <= character && character <= 'z' || 'A' <= character && character <= 'Z' || character == '_'
}

fn is_integer(character: char) -> bool {
    '0' <= character && character <= '9'
}
