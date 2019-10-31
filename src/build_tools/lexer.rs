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
            self.current_char = '0';
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
            if self.current_char == '"' || self.current_char == '0' {
                break;
            }
        }

        let string: String = self.input[position..self.position].iter().collect();

        string
    }
}

// fn new_token(token_type: TokenType, line: usize, character: char) -> Token {
//     Token {
//         type: tokenType,
//         literal: string(char),
//         line: line,
//     }
// }
