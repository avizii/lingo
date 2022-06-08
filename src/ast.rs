use crate::token::Token;
use std::any::Any;

/// AST node. contains two different types of nodes: expression and statement
pub trait Node {
    /// return the literal value of the token it's associated with
    /// this method will be used only for debugging and testing
    fn token_literal(&self) -> &str;

    /// converting a trait into a concrete type
    /// refer to:
    /// * (downcast-trait-object)[https://bennetthardwick.com/rust/downcast-trait-object/]
    /// * (downcast in rust)[https://ysantos.com/blog/downcast-rust]
    fn as_any(&self) -> &dyn Any;

    /// print AST nodes for debugging and to compare them with other AST nodes
    fn format(&self) -> String;
}

/// statement don't produce a value
/// including `let`
pub trait Statement: Node {
    fn statement_node(&self);
}

/// expression produces a value
/// including `function literals`
pub trait Expression: Node {
    fn expression_node(&self);
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

    pub fn format(&self) -> String {
        let mut out = String::new();
        for stat in &self.statements {
            out.push_str(&stat.format());
        }
        out
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn format(&self) -> String {
        self.value.clone()
    }
}

/// the identifier in a let statement doesn't produce a value, but in order to keep things simple,
/// we perform the `Identifier` to implements the `Expression`. because `Identifier` in other parts
/// of a Lingo program does produce values
impl Expression for Identifier {
    fn expression_node(&self) {}
}

pub struct IntegerLiteral {
    pub token: Token,
    pub value: usize,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn format(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

/// struct of usage is the following:
/// ```
/// <prefix operator><expression>;
/// ```
pub struct PrefixExpression {
    pub token: Token,
    /// contain either '-' or '!'
    pub operator: String,
    /// contain the expression to the right of the operator
    pub right: Box<dyn Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn format(&self) -> String {
        format!("({}{})", self.operator, self.right.format())
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}

pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn format(&self) -> String {
        format!(
            "({} {} {})",
            self.left.format(),
            self.operator,
            self.right.format()
        )
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}
}

pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Node for Boolean {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn format(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for Boolean {
    fn expression_node(&self) {}
}

pub struct IfExpression {
    pub token: Token,
    pub condition: Box<dyn Expression>,
}

/// let-statement form is as following:
/// ```
/// let <identifier> = <expression>;
/// ```
pub struct LetStatement {
    pub token: Token,
    /// hold the identifier of the binding
    pub name: Identifier,
    /// the expression that produces the value
    pub value: Option<Box<dyn Expression>>, // TODO
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn format(&self) -> String {
        let value_format: String = match &self.value {
            None => String::new(),
            Some(expression) => expression.format(),
        };
        format!(
            "{} {} = {};",
            self.token_literal(),
            self.name.format(),
            value_format
        )
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

/// return-statement's form is as following:
/// ```
/// return <expression>;
/// ```
pub struct ReturnStatement {
    /// initial token
    pub token: Token,
    /// contain the expression that is to be returned
    pub return_value: Option<Box<dyn Expression>>, // TODO
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn format(&self) -> String {
        let value_format: String = match &self.return_value {
            None => String::new(),
            Some(expression) => expression.format(),
        };
        format!("{} {};", self.token_literal(), value_format)
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<dyn Expression>>, // TODO
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn format(&self) -> String {
        match &self.expression {
            None => String::new(),
            Some(expression) => expression.format(),
        }
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::ast::{Identifier, LetStatement, Program};
    use crate::token::{Token, IDENT, LET};

    #[test]
    fn test_node_format() {
        let program = Program {
            statements: vec![Box::new(LetStatement {
                token: Token {
                    token_type: LET,
                    literal: "let".to_string(),
                },
                name: Identifier {
                    token: Token {
                        token_type: IDENT,
                        literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                },
                value: Some(Box::new(Identifier {
                    token: Token {
                        token_type: IDENT,
                        literal: "anotherVar".to_string(),
                    },
                    value: "anotherVar".to_string(),
                })),
            })],
        };

        assert_eq!(program.format(), "let myVar = anotherVar;");
    }
}
