mod build_tools;
use std::fs;

fn main() {
    let filename = "test_input.mo";
    let input = fs::read_to_string(filename);
    let input_str = match input {
        Ok(string) => string,
        Err(error) => panic!("Error opening file {}: {}", filename, error),
    };
    let lexer = build_tools::lexer::Lexer::new(input_str);
    let parser = build_tools::parser::Parser::new(lexer);
    // let program = parser::parse_program();
    // let result = evaluate_ast(program);

    println!("Hey from the end")
}

// fn evaluate_ast(program: &ast::root_node) -> Object {
//     let env = Object::new_environment();
//     return evaluator::eval(program, env)
// }
