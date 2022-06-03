use crate::ast::Program;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            lexer,
            cur_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&self) -> Option<Program> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, LetStatement, Program, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_let_statements() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let mut lexer = Lexer::new(input.to_string());
        let parser = Parser::new(lexer);

        let program = parser.parse_program();

        match program {
            None => eprintln!("parse_program returned none"),
            Some(program) => {
                if program.statements.len() != 3 {
                    eprintln!(
                        "program statements does not contain 3 statements. got={}",
                        program.statements.len()
                    );
                }

                let tests = ["x", "y", "foobar"];

                for (i, expected_identifier) in tests.into_iter().enumerate() {
                    let stat = program.statements.get(i).unwrap();
                    if !test_let_statement(stat, expected_identifier) {
                        return;
                    }
                }
            }
        }
    }

    fn test_let_statement(stat: &Box<dyn Statement>, name: &str) -> bool {
        if stat.token_literal() != "let" {
            eprintln!(
                "statement token_literal not 'let'. got={}",
                stat.token_literal()
            );
            return false;
        }

        // how to castdown trait object to a specific struct which implement the trait
        // https://bennetthardwick.com/rust/downcast-trait-object/
        let let_stat = stat
            .as_any()
            .downcast_ref::<LetStatement>()
            .expect("statement not LetStatement.");

        if let_stat.name.value != name {
            eprintln!(
                "let_stat name's value not '{}'. got={}",
                name, let_stat.name.value
            );
            return false;
        }

        if let_stat.name.token_literal() != name {
            eprintln!(
                "stat name not '{}'. got={}",
                name,
                let_stat.name.token_literal()
            );
            return false;
        };

        true
    }
}
