use crate::token::*;
use std::ascii;
use std::ops::Add;

/// the lexer only supports ASCII characters instead of the full Unicode range
/// in oder to keep things simple and concentrate on the essential parts of our interpreter.
///
/// the lexer only turn the input into tokens, not to tell us whether code makes sense, works or contains errors.
pub struct Lexer {
    input: String,
    /// current position in input (point to current char)
    position: usize,
    /// current reading position in input (after current char)
    read_position: usize,
    /// current char under examination
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lex = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lex.read_char();
        lex
    }

    /// give us the next character and advance our position in the input string
    pub fn read_char(&mut self) {
        // check whether we have reached the end of input
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            let pos_ch = self.input.as_bytes().get(self.read_position).unwrap();
            self.ch = pos_ch.to_owned();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        // check whether to advance our position in the input string after match a token
        // if match letter or digit, it should not advance the position because the position had already advanced when read entire literal.
        let mut char_advance = true;

        let token = match self.ch {
            b'=' => {
                // composed of two characters ==
                if self.peek_char() == b'=' {
                    let mut literal = String::new();
                    literal.push(self.ch as char);
                    self.read_char();
                    literal.push(self.ch as char);

                    Token {
                        token_type: EQ,
                        literal,
                    }
                } else {
                    Token::new(ASSIGN, self.ch)
                }
            }
            b'!' => {
                // composed of two characters !=
                if self.peek_char() == b'=' {
                    let mut literal = String::new();
                    literal.push(self.ch as char);
                    self.read_char();
                    literal.push(self.ch as char);

                    Token {
                        token_type: NOT_EQ,
                        literal,
                    }
                } else {
                    Token::new(BANG, self.ch)
                }
            }
            b'+' => Token::new(PLUS, self.ch),
            b'-' => Token::new(MINUS, self.ch),
            b'/' => Token::new(SLASH, self.ch),
            b'*' => Token::new(ASTERISK, self.ch),
            b'>' => Token::new(GT, self.ch),
            b'<' => Token::new(LT, self.ch),
            b';' => Token::new(SEMICOLON, self.ch),
            b',' => Token::new(COMMA, self.ch),
            b'(' => Token::new(LPAREN, self.ch),
            b')' => Token::new(RPAREN, self.ch),
            b'{' => Token::new(LBRACE, self.ch),
            b'}' => Token::new(RBRACE, self.ch),
            0 => Token {
                token_type: EOF,
                literal: "".to_string(),
            },
            _ => {
                if is_letter(self.ch) {
                    char_advance = false;

                    let literal = self.read_identifier();
                    let token_type = lookup_ident(literal);
                    Token {
                        token_type,
                        literal: literal.to_string(),
                    }
                } else if is_digit(self.ch) {
                    char_advance = false;

                    Token {
                        token_type: INT,
                        literal: self.read_number().to_string(),
                    }
                } else {
                    Token::new(ILLEGAL, self.ch)
                }
            }
        };
        if char_advance {
            self.read_char();
        }
        token
    }

    /// reads in an identifier and advances our lexer's positions
    /// until it encounters a non-letter-character
    fn read_identifier(&mut self) -> &str {
        let pos = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        &self.input[pos..self.position]
    }

    fn read_number(&mut self) -> &str {
        let pos = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        &self.input[pos..self.position]
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_char()
        }
    }

    /// similar to `read_char()` method, except that it doesn't increment `position` and `read_position`
    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            let pos_char = &self.input[self.read_position..self.read_position + 1]
                .as_bytes()
                .get(0)
                .unwrap()
                .to_owned();
            *pos_char
        }
    }
}

/// check whether the given argument is a letter
/// wh treat `_` as a letter and allow it in identifiers and keywords
fn is_letter(ch: u8) -> bool {
    (b'a' <= ch && ch <= b'z') || (b'A' <= ch && ch <= b'Z') || ch == b'_'
}

fn is_whitespace(ch: u8) -> bool {
    (ch == b' ') || (ch == b'\t') || (ch == b'\n') || (ch == b'\r')
}

fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::token::{IDENT, LET};

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let lex = Lexer::new(input.to_string());

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

        walk_through_input_token(lex, tests);

        let input = r#"
            let five = 5;
            let ten = 10;
                    
            let add = fn(x, y) {
                x + y;
            };
                    
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            
            10 == 10;
            10 != 9;
            "#;
        let lex = Lexer::new(input.to_string());

        let tests = vec![
            (LET, "let"),
            (IDENT, "five"),
            (ASSIGN, "="),
            (INT, "5"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "ten"),
            (ASSIGN, "="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "add"),
            (ASSIGN, "="),
            (FUNCTION, "fn"),
            (LPAREN, "("),
            (IDENT, "x"),
            (COMMA, ","),
            (IDENT, "y"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (IDENT, "x"),
            (PLUS, "+"),
            (IDENT, "y"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "result"),
            (ASSIGN, "="),
            (IDENT, "add"),
            (LPAREN, "("),
            (IDENT, "five"),
            (COMMA, ","),
            (IDENT, "ten"),
            (RPAREN, ")"),
            (SEMICOLON, ";"),
            (BANG, "!"),
            (MINUS, "-"),
            (SLASH, "/"),
            (ASTERISK, "*"),
            (INT, "5"),
            (SEMICOLON, ";"),
            (INT, "5"),
            (LT, "<"),
            (INT, "10"),
            (GT, ">"),
            (INT, "5"),
            (SEMICOLON, ";"),
            (IF, "if"),
            (LPAREN, "("),
            (INT, "5"),
            (LT, "<"),
            (INT, "10"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (TRUE, "true"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (ELSE, "else"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (FALSE, "false"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (INT, "10"),
            (EQ, "=="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (INT, "10"),
            (NOT_EQ, "!="),
            (INT, "9"),
            (SEMICOLON, ";"),
            (EOF, ""),
        ];

        walk_through_input_token(lex, tests);
    }

    fn walk_through_input_token(mut lex: Lexer, expected_tokens: Vec<(TokenType, &str)>) {
        for (i, (expected_type, expected_literal)) in expected_tokens.into_iter().enumerate() {
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
}
