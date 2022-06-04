use crate::ast::{Identifier, LetStatement, Program, ReturnStatement, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType, ASSIGN, EOF, IDENT, LET, RETURN, SEMICOLON};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            lexer,
            cur_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        while self.cur_token.token_type != EOF {
            let stat = self.parse_statement();
            if let Some(stat) = stat {
                statements.push(stat);
            }
            self.next_token();
        }
        Some(Program { statements })
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_type {
            token_let if token_let == LET => {
                let let_stat = self.parse_let_statement();
                match let_stat {
                    None => None,
                    Some(let_stat) => Some(Box::new(let_stat)),
                }
            }
            token_return if token_return == RETURN => {
                let return_stat = self.parse_return_statement();
                match return_stat {
                    None => None,
                    Some(return_stat) => Some(Box::new(return_stat)),
                }
            }
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let cur_token = self.cur_token.clone();

        if !self.expect_peek(IDENT) {
            return None;
        }

        let ident_name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(ASSIGN) {
            return None;
        }

        // TODO: we're skipping the expressions until we encounter a semicolon
        while !self.cur_token_is(SEMICOLON) {
            self.next_token();
        }

        Some(LetStatement {
            token: cur_token,
            name: ident_name,
            value: None, // TODO
        })
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let cur_token = self.cur_token.clone();

        self.next_token();

        // TODO: we're skipping the expressions until we encounter a semicolon
        while !self.cur_token_is(SEMICOLON) {
            self.next_token();
        }

        Some(ReturnStatement {
            token: cur_token,
            return_value: None, //TODO
        })
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    /// enforce the correctness of the order of tokens by checking the type of the next token
    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    fn errors(&self) -> &[String] {
        self.errors.as_slice()
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "expected next token to be {}, got {} instead",
            token_type, self.peek_token.token_type
        );
        self.errors.push(msg)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, LetStatement, Program, ReturnStatement, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_let_statements() {
        // valid lingo source code
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        lingo_source_code_parser(input, 3);

        // invalid input where tokens are missing
        let input = r#"
        let x 5;
        let = 10;
        let 838383;
        "#;
        lingo_source_code_parser(input, 3);
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
        return 5;
        return 10;
        return 993322;
        "#;

        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        match program {
            None => eprintln!("parse_program return none"),
            Some(program) => {
                if program.statements.len() != 3 {
                    eprintln!(
                        "program statements does not contain 3 statements. got={}",
                        program.statements.len()
                    );
                };

                for stat in program.statements {
                    let return_stat = stat
                        .as_any()
                        .downcast_ref::<ReturnStatement>()
                        .expect("statement not ReturnStatement");

                    if return_stat.token_literal() != "return" {
                        eprintln!(
                            "return_statement token_literal not 'return', got {}",
                            return_stat.token_literal()
                        );
                    }
                }
            }
        }
    }

    fn lingo_source_code_parser(code: &str, len: usize) {
        let mut lexer = Lexer::new(code.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        match program {
            None => eprintln!("parse_program returned none"),
            Some(program) => {
                if program.statements.len() != len {
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

    fn check_parser_errors(parser: &Parser) {
        let errors = parser.errors();

        if errors.is_empty() {
            return;
        }

        eprintln!("parser has {} errors", errors.len());

        for err in errors {
            eprintln!("parser error: {}", err);
        }

        // fail now
        assert!(false);
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
