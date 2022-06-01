use crate::token::{Token, ASSIGN, COMMA, EOF, LBRACE, LPAREN, PLUS, RBRACE, RPAREN, SEMICOLON};
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
        let token = match self.ch {
            61 => Token::new(ASSIGN, self.ch),    // =
            59 => Token::new(SEMICOLON, self.ch), // ;
            40 => Token::new(LPAREN, self.ch),    // (
            41 => Token::new(RPAREN, self.ch),    // )
            44 => Token::new(COMMA, self.ch),     // ,
            43 => Token::new(PLUS, self.ch),      // +
            123 => Token::new(LBRACE, self.ch),   // {
            125 => Token::new(RBRACE, self.ch),   // }
            0 => Token {
                token_type: EOF,
                literal: "".to_string(),
            },
            _ => panic!("unsupported token char"),
        };
        self.read_char();
        token
    }
}
