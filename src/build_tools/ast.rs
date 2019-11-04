trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

trait Statement {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
    fn statement_node(&self);
}

pub trait Expression {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
    fn expression_node(&self);
}

struct RootNode {
    statements: Vec<Box<dyn Statement>>,
}

impl RootNode {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        }

        return String::from("");
    }

    fn string(&self) -> String {
        let mut statements_string: String = String::from("");

        for s in &self.statements {
            statements_string += &s.string();
        }

        statements_string
    }
}
