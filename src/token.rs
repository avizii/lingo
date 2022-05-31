type TokenType = &'static str;

struct Token {
    token_type: TokenType,
    literal: String,
}

// signifies a token/character we don't know about
const ILLEGAL: TokenType = "ILLEGAL";
// stands for "end of file", which tells our parser later on that it can stop
const EOF: TokenType = "EOF";

// identifiers
const IDENT: TokenType = "IDENT";
// literals
const INT: TokenType = "INT";

// operators
const ASSIGN: TokenType = "=";
const PLUS: TokenType = "+";

// delimiters
const COMMA: TokenType = ",";
const SEMICOLON: TokenType = ";";
const LPAREN: TokenType = "(";
const RPAREN: TokenType = ")";
const LBRACE: TokenType = "{";
const RBRACE: TokenType = "}";

// keywords
const FUNCTION: TokenType = "FUNCTION";
const LET: TokenType = "LET";

#[cfg(test)]
mod tests {
    use super::*;

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

        let lex = Lexer::new(input);

        for tt in tests {
            let token: Token = lex.next_token();
            if token.token_type != tt.0 {
                todo!()
            }
            if token.literal.as_str() != tt.1 {
                todo!()
            }
        }
    }
}
