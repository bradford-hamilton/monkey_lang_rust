use monkey_lang_rust::Lexer;
use std::fs;

fn main() {
    let filename = "test_input.mo";
    let input = fs::read_to_string(filename);
    let input = match input {
        Ok(string) => string,
        Err(error) => panic!("Error opening file {}: {}", filename, error),
    };
    let lexer = Lexer::new(input);

    println!("Hey from the end")
}
