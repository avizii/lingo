use phf::phf_map;
use std::ascii;
use std::fmt::{Display, Formatter};

pub type TokenType = &'static str;

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
pub const MINUS: TokenType = "-";
pub const BANG: TokenType = "!";
pub const ASTERISK: TokenType = "*";
pub const SLASH: TokenType = "/";
pub const LT: TokenType = "<";
pub const GT: TokenType = ">";
pub const EQ: TokenType = "==";
pub const NOT_EQ: TokenType = "!=";

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
pub const TRUE: TokenType = "TRUE";
pub const FALSE: TokenType = "FALSE";
pub const IF: TokenType = "IF";
pub const ELSE: TokenType = "ELSE";
pub const RETURN: TokenType = "RETURN";

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "fn" => FUNCTION,
    "let" => LET,
    "true" => TRUE,
    "false" => FALSE,
    "if" => IF,
    "else" => ELSE,
    "return" => RETURN,
};

/// check the `KEYWORDS` table to see whether the given identifier is in fact a keyword
/// if it is, it returns the keyword's `TokenType` constant.
/// if it isn't, we just get back `IDENT`, which is the `TokenType` for all user-defined identifiers.
pub fn lookup_ident(ident: &str) -> TokenType {
    if KEYWORDS.contains_key(ident) {
        KEYWORDS.get(ident).unwrap().to_owned()
    } else {
        IDENT
    }
}

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

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Type:{}, Literal: {}]", self.token_type, self.literal)
    }
}
