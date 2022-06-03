use crate::token::Token;
use std::any::Any;

/// AST node. contains two different types of nodes: expression and statement
trait Node {}

/// statement don't produce a value
/// including `let`
pub trait Statement: Node {
    fn statement_node(&self);

    /// return the literal value of the token it's associated with
    /// this method will be used only for debugging and testing
    fn token_literal(&self) -> &str;

    /// converting a trait into a concrete type
    /// refer to:
    /// * (downcast-trait-object)[https://bennetthardwick.com/rust/downcast-trait-object/]
    /// * (downcast in rust)[https://ysantos.com/blog/downcast-rust]
    fn as_any(&self) -> &dyn Any;
}

/// expression produces a value
/// including `function literals`
pub trait Expression: Node {
    fn expression_node(&self);

    /// return the literal value of the token it's associated with
    /// this method will be used only for debugging and testing
    fn token_literal(&self) -> &str;

    fn as_any(&self) -> &dyn Any;
}

/// the root node of every AST out parser produces
/// every valid Lingo program is a series of statements
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    fn token_literal(&self) -> &str {
        if !self.statements.is_empty() {
            match self.statements.get(0) {
                None => "",
                Some(statement) => statement.token_literal(),
            }
        } else {
            ""
        }
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {}

/// the identifier in a let statement doesn't produce a value, but in order to keep things simple,
/// we perform the `Identifier` to implements the `Expression`. because `Identifier` in other parts
/// of a Lingo program does produce values
impl Expression for Identifier {
    fn expression_node(&self) {}

    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct LetStatement {
    pub token: Token,
    /// hold the identifier of the binding
    pub name: Identifier,
    /// the expression that produces the value
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
