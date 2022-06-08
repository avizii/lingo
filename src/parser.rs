use crate::ast::{
    Boolean, Expression, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral,
    LetStatement, PrefixExpression, Program, ReturnStatement, Statement,
};
use crate::lexer::Lexer;
use crate::token::{
    Token, TokenType, ASSIGN, ASTERISK, BANG, EOF, EQ, FALSE, GT, IDENT, INT, LET, LPAREN, LT,
    MINUS, NOT_EQ, PLUS, RETURN, RPAREN, SEMICOLON, SLASH, TRUE,
};
use iota::iota;
use phf::phf_map;
use std::collections::HashMap;
use std::num::ParseIntError;

iota! {
    const LOWEST: u8 = 1 << iota;
        , EQUALS  // ==
        , LESSGREATER // > or <
        , SUM // +
        , PRODUCT // *
        , PREFIX // -X or !X
        , CALL // myFunction(X)
}

type PrefixParseFn = fn(&mut Parser) -> Box<dyn Expression>;
type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Box<dyn Expression>;

pub struct Parser {
    lexer: Lexer,
    errors: Vec<String>,
    cur_token: Token,
    peek_token: Token,
    /// called when we encounter the associated token type in prefix position
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    /// called when we encounter the associated token type in infix position
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,

    precedences: HashMap<TokenType, u8>,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();

        let mut precedences = HashMap::new();
        precedences.insert(EQ, EQUALS);
        precedences.insert(NOT_EQ, EQUALS);
        precedences.insert(LT, LESSGREATER);
        precedences.insert(GT, LESSGREATER);
        precedences.insert(PLUS, SUM);
        precedences.insert(MINUS, SUM);
        precedences.insert(SLASH, PRODUCT);
        precedences.insert(ASTERISK, PRODUCT);

        let parse_identifier_fn: fn(&mut Parser) -> Box<dyn Expression> = |parser: &mut Parser| {
            Box::new(Identifier {
                token: parser.cur_token.clone(),
                value: parser.cur_token.literal.clone(),
            })
        };

        let parse_integer_literal_fn: fn(&mut Parser) -> Box<dyn Expression> =
            |parser: &mut Parser| {
                let token = parser.cur_token.clone();
                let literal = parser
                    .cur_token
                    .literal
                    .parse::<usize>()
                    .expect("could not parse input as usize");

                Box::new(IntegerLiteral {
                    token,
                    value: literal,
                })
            };

        let parse_prefix_expression_fn: fn(&mut Parser) -> Box<dyn Expression> =
            |parser: &mut Parser| {
                let token = parser.cur_token.clone();
                let operator = parser.cur_token.literal.clone();

                parser.next_token();

                let right = parser
                    .parse_expression(PREFIX)
                    .expect("could not parse next token as Expression");

                Box::new(PrefixExpression {
                    token,
                    operator,
                    right,
                })
            };

        let parse_prefix_boolean_fn: fn(&mut Parser) -> Box<dyn Expression> =
            |parser: &mut Parser| {
                Box::new(Boolean {
                    token: parser.cur_token.clone(),
                    value: parser.cur_token_is(TRUE),
                })
            };

        let parse_prefix_grouped_expression_fn: fn(&mut Parser) -> Box<dyn Expression> =
            |parser: &mut Parser| {
                parser.next_token();

                let expression = parser.parse_expression(LOWEST).unwrap();

                if !parser.expect_peek(RPAREN) {
                    eprintln!("expect RPAREN error.")
                }
                expression
            };

        let mut prefix_parse_fns = HashMap::new();
        prefix_parse_fns.insert(IDENT, parse_identifier_fn);
        prefix_parse_fns.insert(INT, parse_integer_literal_fn);
        prefix_parse_fns.insert(BANG, parse_prefix_expression_fn);
        prefix_parse_fns.insert(MINUS, parse_prefix_expression_fn);
        prefix_parse_fns.insert(TRUE, parse_prefix_boolean_fn);
        prefix_parse_fns.insert(FALSE, parse_prefix_boolean_fn);
        prefix_parse_fns.insert(LPAREN, parse_prefix_grouped_expression_fn);

        let parse_infix_expression_fn: fn(&mut Parser, Box<dyn Expression>) -> Box<dyn Expression> =
            |parser: &mut Parser, left: Box<dyn Expression>| {
                let token = parser.cur_token.clone();
                let operator = parser.cur_token.literal.clone();

                let precedence = parser.cur_precedence();

                parser.next_token();

                let right = parser
                    .parse_expression(precedence)
                    .expect("could not parse next token as Expression");

                Box::new(InfixExpression {
                    token,
                    left,
                    operator,
                    right,
                })
            };

        let mut infix_parse_fns = HashMap::new();
        infix_parse_fns.insert(PLUS, parse_infix_expression_fn);
        infix_parse_fns.insert(MINUS, parse_infix_expression_fn);
        infix_parse_fns.insert(SLASH, parse_infix_expression_fn);
        infix_parse_fns.insert(ASTERISK, parse_infix_expression_fn);
        infix_parse_fns.insert(EQ, parse_infix_expression_fn);
        infix_parse_fns.insert(NOT_EQ, parse_infix_expression_fn);
        infix_parse_fns.insert(LT, parse_infix_expression_fn);
        infix_parse_fns.insert(GT, parse_infix_expression_fn);

        Self {
            lexer,
            cur_token,
            peek_token,
            errors: Vec::new(),
            prefix_parse_fns,
            infix_parse_fns,
            precedences,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn register_prefix(&mut self, token_type: TokenType, prefix_fn: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, prefix_fn);
    }

    fn register_infix(&mut self, token_type: TokenType, infix_fn: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, infix_fn);
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
            _ => {
                let expression_stat = self.parse_expression_statement();
                match expression_stat {
                    None => None,
                    Some(expression_stat) => Some(Box::new(expression_stat)),
                }
            }
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

    fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
        let cur_token = self.cur_token.clone();

        let expression = self.parse_expression(LOWEST);

        // we want expression statements to have optional semicolons
        if self.peek_token_is(SEMICOLON) {
            self.next_token();
        }

        Some(ExpressionStatement {
            token: cur_token,
            expression: expression,
        })
    }

    fn parse_expression(&mut self, precedence: u8) -> Option<Box<dyn Expression>> {
        let prefix_fn = self.prefix_parse_fns.get(&self.cur_token.token_type);
        match prefix_fn {
            None => {
                self.no_prefix_parse_fn_error(self.cur_token.token_type);
                None
            }
            Some(prefix_fn) => {
                let mut expression: Option<Box<dyn Expression>>;
                let left_expression: Box<dyn Expression> = prefix_fn(self);
                expression = Some(left_expression);

                while !self.peek_token_is(SEMICOLON) && precedence < self.peek_precedence() {
                    let infix_fn = self.infix_parse_fns.get(self.peek_token.token_type);
                    if let Some(infix_fn) = infix_fn {
                        // TODO why blow code for function call can not compile
                        // self.next_token();

                        self.cur_token = self.peek_token.clone();
                        self.peek_token = self.lexer.next_token();

                        let infix_expression: Box<dyn Expression> =
                            infix_fn(self, expression.unwrap());
                        expression = Some(infix_expression);
                    }
                }

                expression
            }
        }
    }

    fn parse_identifier(&self) -> Box<dyn Expression> {
        Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        })
    }

    fn parse_boolean(&self) -> Box<dyn Expression> {
        Box::new(Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TRUE),
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

    fn no_prefix_parse_fn_error(&mut self, token_type: TokenType) {
        let msg = format!("no prefix parse function for {} found", token_type);
        self.errors.push(msg)
    }

    fn peek_precedence(&self) -> u8 {
        match self.precedences.get(self.peek_token.token_type) {
            None => LOWEST,
            Some(precedence) => *precedence,
        }
    }

    fn cur_precedence(&self) -> u8 {
        match self.precedences.get(self.cur_token.token_type) {
            None => LOWEST,
            Some(precedence) => *precedence,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        Expression, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement,
        Node, PrefixExpression, Program, ReturnStatement, Statement,
    };
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

    #[test]
    fn test_identifier_expression() {
        let code = "foobar;";

        let mut lexer = Lexer::new(code.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);

        let expression_statement = program
            .statements
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("statement not ExpressionStatement");

        let identifier = expression_statement
            .expression
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<Identifier>()
            .expect("expression not Identifier");

        assert_eq!(identifier.value, "foobar");
        assert_eq!(identifier.token_literal(), "foobar");
    }

    #[test]
    fn test_integer_literal_expression() {
        let code = "5;";

        let mut lexer = Lexer::new(code.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);

        let expression_statement = program
            .statements
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("statement not ExpressionStatement");

        let literal = expression_statement
            .expression
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<IntegerLiteral>()
            .expect("expression not IntegerLiteral");

        assert_eq!(literal.value, 5_usize);
        assert_eq!(literal.token_literal(), "5");
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let prefixs = vec![("!5;", "!", 5_usize), ("-15;", "-", 15)];

        for (input, operator, value) in prefixs {
            let mut lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program().unwrap();
            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1);

            let expression_statement = program
                .statements
                .first()
                .unwrap()
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .expect("statement not ExpressionStatement");

            let expression = expression_statement
                .expression
                .as_ref()
                .unwrap()
                .as_any()
                .downcast_ref::<PrefixExpression>()
                .expect("expression not PrefixExpression");

            assert_eq!(expression.operator, operator);

            assert!(test_integer_literal(&expression.right, value));
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let infixs = vec![
            ("5 + 5;", 5_usize, "+", 5_usize),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for (input, left_value, operator, right_value) in infixs {
            let mut lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program().unwrap();
            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1);

            let expression_statement = program
                .statements
                .first()
                .unwrap()
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .expect("statement not ExpressionStatement");

            let expression = expression_statement
                .expression
                .as_ref()
                .unwrap()
                .as_any()
                .downcast_ref::<InfixExpression>()
                .expect("expression not PrefixExpression");

            assert!(test_integer_literal(&expression.left, left_value));

            assert_eq!(expression.operator, operator);

            assert!(test_integer_literal(&expression.right, right_value));
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let expressions = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
        ];
        for (input, expected) in expressions {
            let mut lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program().unwrap();
            check_parser_errors(&parser);

            assert_eq!(program.format(), expected);
        }
    }

    #[test]
    fn test_expression_precedence_parsing() {
        let input = "2 + 2 + 3 * 1 - 2 + 5 * 4 - 1";
        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        check_parser_errors(&parser);

        println!("{}", program.format());
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

    fn test_integer_literal(expression: &Box<dyn Expression>, value: usize) -> bool {
        let integer_literal = expression
            .as_any()
            .downcast_ref::<IntegerLiteral>()
            .expect("expression not IntegerLiteral");

        if integer_literal.value != value {
            eprintln!(
                "integer_literal's value not {}. got={}",
                value, integer_literal.value
            );
            return false;
        }

        if integer_literal.token_literal() != value.to_string().as_str() {
            eprintln!(
                "integer_literal's token_literal not {}. got={}",
                value,
                integer_literal.token_literal()
            );
            return false;
        }
        true
    }
}
