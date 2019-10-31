pub struct Lexer {
    input: Vec<char>,
    current_char: char,
    position: u32, 
    read_position: u32, 
    line: u32,
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
        if self.read_position >= self.input.len() as u32 {
            self.current_char = '0';
        } else {
            self.current_char = self.input[self.read_position as usize];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}