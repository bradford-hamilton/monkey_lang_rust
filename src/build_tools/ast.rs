use crate::build_tools::token::*;

/// Node - nodes in our ast will provide a token_literal and string methods for debugging
trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

/// Statement - must provide statement_node, token_literal, and string methods. Statements do not produce values.
trait Statement {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
    fn statement_node(&self);
}

/// Expression - must provide expression_node, token_literal, and string methods. Expressions produce values.
pub trait Expression {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
    fn expression_node(&self);
}

/// RootNode of every AST our parser produces.
struct RootNode {
    statements: Vec<Box<dyn Statement>>,
}

impl RootNode {
    /// token_literal returns the RootNode's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        }
        String::from("")
    }

    /// string returns a buffer containing the programs Statements as strings.
    fn string(&self) -> String {
        let mut statements_string: String = String::from("");

        for s in &self.statements {
            statements_string += &s.string();
        }

        statements_string
    }
}

/// ZeroValueExpression - this is like very poor rust, but I'd like to continue moving forward for now. This
/// will be useful in a lot of scenarios I'm running into where you cannot initialize a struct with only
/// some of the fields and so I'm adding this "zero value" expression for when needing to return something
pub struct ZeroValueExpression {}
impl Expression for ZeroValueExpression {
    fn token_literal(&self) -> String {
        "zero value".to_owned()
    }
    fn string(&self) -> String {
        "zero value".to_owned()
    }
    fn expression_node(&self) {}
}

/// Identifier - holds IDENTIFIER token and it's value (add, foobar, x, y, ...)
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Expression for Identifier {
    /// token_literal returns the Identifier's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    /// string - returns a string representation of the Identifier and satisfies our Node interface
    fn string(&self) -> String {
        self.value.clone()
    }

    fn expression_node(&self) {}
}

/// IntegerLiteral - holds the token and it's value (int64)
pub struct IntegerLiteral {
    pub token: Token,
    pub value: usize,
}

impl Expression for IntegerLiteral {
    /// token_literal returns the IntegerLiteral's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    /// string - returns a string representation of the IntegerLiteral and satisfies our Node interface
    fn string(&self) -> String {
        self.token.literal.clone()
    }

    fn expression_node(&self) {}
}

/// PrefixExpression - holds the token, a string version of the operator, and the expression to the right of it
pub struct PrefixExpression {
    /// The prefix token (! or -)
    pub token: Token,
    /// String (either "!" or "-")
    pub operator: String,
    /// The expression to the right of the operator
    pub right: Box<dyn Expression>,
}

impl Expression for PrefixExpression {
    /// token_literal returns the PrefixExpression's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    /// String - returns a string representation of the operator followed by it's expression to the right (-5) and satisfies our Node interface
    fn string(&self) -> String {
        let mut buf = "(".to_owned();
        buf += &self.operator[..];
        buf += &self.right.string();
        buf += ")";
        buf
    }

    fn expression_node(&self) {}
}
