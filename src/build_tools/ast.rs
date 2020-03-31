use crate::build_tools::token::*;

/// Node - nodes in our ast will provide a token_literal and string methods for debugging
trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

/// Statement - must provide statement_node, token_literal, and string methods. Statements do not produce values.
pub trait Statement {
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
        "".to_owned()
    }
    /// string returns a buffer containing the programs Statements as strings.
    fn string(&self) -> String {
        let mut statements_string: String = "".to_owned();

        for s in &self.statements {
            statements_string += &s.string();
        }

        statements_string
    }
}

/// ZeroValueExpression - is used for initializations
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

/// ZeroValueStatement is used for initializations
pub struct ZeroValueStatement {}
impl Statement for ZeroValueStatement {
    fn token_literal(&self) -> String {
        "zero value".to_owned()
    }
    fn string(&self) -> String {
        "zero value".to_owned()
    }
    fn statement_node(&self) {}
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

/// Boolean - holds the token and it's value (a boolean)
pub struct Boolean {
    /// The prefix token (! or -)
    pub token: Token,
    /// String (either "!" or "-")
    pub value: bool,
}

impl Expression for Boolean {
    /// token_literal returns the Boolean's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    /// string - returns a string representation of the Boolean and satisfies our Node interface
    fn string(&self) -> String {
        self.token.literal.clone()
    }
    fn expression_node(&self) {}
}

/// IfExpression - holds the token, the condition expression and the consequence & alternative
/// block statements. Structure: if (<condition>) <consequence> else <alternative>
pub struct IfExpression {
    pub token: Token, // The If token
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: BlockStatement,
}

impl Expression for IfExpression {
    /// token_literal returns the IfExpression's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    /// string - returns a string representation of the IfExpression and satisfies our Node interface
    fn string(&self) -> String {
        let mut buf = "if".to_owned();
        // TODO: match on condition and get it's Expression
        // buf += self.condition.string();
        buf += " ";
        // buf += self.consequence.string();
        // TODO: something besides nil
        // if self.alternative != nil {
        //     buf += " else ";
        //     buf += self.alternative.string();
        // }
        buf
    }

    fn expression_node(&self) {}
}

/// BlockStatement - holds the token "{", and a slice of statements
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>,
}

impl Statement for BlockStatement {
    /// token_literal returns the BlockStatement's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    /// string - returns a string representation of the BlockStatement and satisfies our Node interface
    fn string(&self) -> String {
        // TODO: loop over self.statements and call .string() on each
        "BlockStatement".to_owned()
    }
    fn statement_node(&self) {}
}

impl Expression for BlockStatement {
    /// token_literal returns the BlockStatement's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    /// string - returns a string representation of the BlockStatement and satisfies our Node interface
    fn string(&self) -> String {
        // TODO: loop over self.statements and call .string() on each
        "BlockStatement".to_owned()
    }
    fn expression_node(&self) {}
}

/// LetStatement - Name holds the identifier of the binding and Value for the expression that produces the value.
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    /// token_literal returns the LetStatement's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    /// string - returns a string representation of the LetStatement and satisfies our Node interface
    fn string(&self) -> String {
        // TODO: actually implement this
        "LetStatement".to_owned()
    }
    fn statement_node(&self) {}
}

/// ConstStatement - Name holds the identifier of the binding and value for the expression that produces the value.
pub struct ConstStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Statement for ConstStatement {
    /// token_literal returns the ConstStatement's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    /// string - returns a string representation of the ConstStatement and satisfies our Node interface
    fn string(&self) -> String {
        // TODO: actually implement this
        "ConstStatement".to_owned()
    }
    fn statement_node(&self) {}
}

/// ReturnStatement - pretty self explanatory, holds RETURN token and return value
pub struct ReturnStatement {
    pub token: Token,
    /// The 'return' token
    pub return_value: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    /// token_literal returns the ReturnStatement's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    /// string - returns a string representation of the ReturnStatement and satisfies our Node interface
    fn string(&self) -> String {
        // TODO: actually implement this
        "ReturnStatement".to_owned()
    }
    fn statement_node(&self) {}
}

/// ExpressionStatement - holds the first token of the expression and the expression
pub struct ExpressionStatement {
    pub token: Token,
    /// The first token of the expression
    pub expression: Box<dyn Expression>,
}

impl Statement for ExpressionStatement {
    /// token_literal returns the ExpressionStatement's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    /// string - returns a string representation of the ExpressionStatement and satisfies our Node interface
    fn string(&self) -> String {
        // TODO: actually implement this
        "ExpressionStatement".to_owned()
    }
    fn statement_node(&self) {}
}

// FunctionLiteral - holds the token, the function params (a vec of Identifier), and
// the function Body (BlockStatement). Structure: func <parameters> <block statement>
pub struct FunctionLiteral {
    pub token: Token, // The 'func' token
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub name: String
}

impl Expression for FunctionLiteral {
    /// token_literal returns the FunctionLiteral's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    /// string - returns a string representation of the FunctionLiteral and satisfies our Node interface
    fn string(&self) -> String {
        // TODO: actually implement them
        "FunctionLiteral".to_owned()
    }
    fn expression_node(&self) {}
}

/// StringLiteral holds the token and it's value (string)
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl Expression for StringLiteral {
    /// token_literal returns the StringLiteral's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    /// string - returns a string representation of the StringLiteral and satisfies our Node interface
    fn string(&self) -> String {
        self.token.literal.clone()
    }
    fn expression_node(&self) {}
}

/// ArrayLiteral holds the token: '[' and an array of expressions (Elements)
pub struct ArrayLiteral {
    pub token: Token,/// the '[' token
    pub elements: Vec<Box<dyn Expression>>,
}

impl Expression for ArrayLiteral {
    /// token_literal returns the ArrayLiteral's literal and satisfies the Node interface.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    /// string - returns a string representation of the ArrayLiteral and satisfies our Node interface
    fn string(&self) -> String {
        // TODO: actually implement them
        "ArrayLiteral".to_owned()
    }
    fn expression_node(&self) {}
}