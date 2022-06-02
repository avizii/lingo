use crate::lexer::Lexer;
use crate::token::EOF;
use std::io::{stdin, stdout, Read, Write};

const PROMPT: &str = ">> ";

/// read eval print loop
pub fn start() {
    let mut s = String::new();
    loop {
        print!("{}", PROMPT);
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        let mut lex = Lexer::new(s.clone());

        loop {
            let token = lex.next_token();

            if token.token_type == EOF {
                break;
            }

            println!("{}", token);
        }

        s.clear();
    }
}
