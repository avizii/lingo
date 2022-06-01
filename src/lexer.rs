use crate::token;
use crate::token::{
    Token, ASSIGN, COMMA, EOF, ILLEGAL, INT, LBRACE, LPAREN, PLUS, RBRACE, RPAREN, SEMICOLON,
};
use std::ascii;

/// the lexer only supports ASCII characters instead of the full Unicode range
/// in oder to keep things simple and concentrate on the essential parts of our interpreter.
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
            b'=' => Token::new(ASSIGN, self.ch),
            b';' => Token::new(SEMICOLON, self.ch),
            b'(' => Token::new(LPAREN, self.ch),
            b')' => Token::new(RPAREN, self.ch),
            b',' => Token::new(COMMA, self.ch),
            b'+' => Token::new(PLUS, self.ch),
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
                    let token_type = token::lookup_ident(literal);
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
        let ident = &self.input[pos..self.position];
        // println!("read ident: {}", ident);
        ident
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
