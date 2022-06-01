use std::ascii;

type TokenType = &'static str;

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: u8) -> Self {
        Self {
            token_type,
            literal: ascii::escape_default(ch).to_string(),
        }
    }
}

// signifies a token/character we don't know about
pub const ILLEGAL: TokenType = "ILLEGAL";
// stands for "end of file", which tells our parser later on that it can stop
pub const EOF: TokenType = "EOF";

// identifiers
pub const IDENT: TokenType = "IDENT";
// literals
pub const INT: TokenType = "INT";

// operators
pub const ASSIGN: TokenType = "=";
pub const PLUS: TokenType = "+";

// delimiters
pub const COMMA: TokenType = ",";
pub const SEMICOLON: TokenType = ";";
pub const LPAREN: TokenType = "(";
pub const RPAREN: TokenType = ")";
pub const LBRACE: TokenType = "{";
pub const RBRACE: TokenType = "}";

// keywords
pub const FUNCTION: TokenType = "FUNCTION";
pub const LET: TokenType = "LET";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = vec![
            (ASSIGN, "="),
            (PLUS, "+"),
            (LPAREN, "("),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RBRACE, "}"),
            (COMMA, ","),
            (SEMICOLON, ";"),
            (EOF, ""),
        ];

        let mut lex = Lexer::new(input.to_string());

        for (i, (expected_type, expected_literal)) in tests.into_iter().enumerate() {
            let token: Token = lex.next_token();
            if token.token_type != expected_type {
                eprintln!(
                    "tests[{}] - token_type wrong. expected={}, got={}",
                    i, expected_type, token.token_type
                );
            }
            if token.literal.as_str() != expected_literal {
                eprintln!(
                    "tests[{}] - literal wrong. expected={}, got={}",
                    i, expected_literal, token.literal
                );
            }
        }
    }

    let shader = r#"
    let five = 5;
    let ten = 10;

    let add = fn(x, y) {
        x + y;
    }

    let result = add(five, ten);
    "#;
}
