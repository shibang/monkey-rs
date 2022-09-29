use crate::{
    ast, lexer,
    token::{self, TokenType},
};

pub struct Parser {
    lexer: lexer::Lexer,
    cur_token: Option<token::Token>,
    peek_token: Option<token::Token>,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Parser {
        let mut p = Parser {
            lexer,
            cur_token: None,
            peek_token: None,
            errors: Vec::new(),
        };
        // 读取两个词法单元，以设置 cur_token 和 peek_token
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        let next_token = self.lexer.next_token();
        self.peek_token = Some(next_token);
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.to_vec()
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token_type,
            self.peek_token.as_ref().unwrap().token_type
        );
        self.errors.push(msg);
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> {
        let mut program = ast::Program::new();
        while let Some(cur_token) = &self.cur_token {
            if cur_token.token_type == TokenType::Eof {
                break;
            }
            if let Some(boxed_stmt) = self.parse_statement() {
                program.statements.push(boxed_stmt);
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        if let Some(cur_token) = &self.cur_token {
            match cur_token.token_type {
                TokenType::Let => self.parse_let_statement(),
                _ => None,
            }
        } else {
            None
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        if self.cur_token.is_none() {
            return None;
        }
        let token = self.cur_token.as_ref().unwrap().clone();
        let mut let_stmt = ast::LetStatement::new(token);

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }
        let token = self.cur_token.as_ref().unwrap();
        let_stmt.name = Some(ast::Identifier::new(token.clone(), token.literal.clone()));

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        // TODO: skip expression
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Some(Box::new(let_stmt))
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.as_ref().unwrap().token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.as_ref().unwrap().token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        }
        self.peek_error(token_type);
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Node, lexer::Lexer};

    use super::*;

    #[test]
    fn let_statement() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert!(program.is_some(), "parse_program() return None");

        let mut statements = program.unwrap().statements;
        assert_eq!(
            3,
            statements.len(),
            "program.statements does not contain 3 statements. got={}",
            statements.len()
        );

        let tests = vec!["x", "y", "foobar"];
        for (_, &tt) in tests.iter().enumerate() {
            let stmt = statements.remove(0);
            test_let_statmement(stmt, tt.to_string());
        }
    }

    fn test_let_statmement(stmt: Box<dyn ast::Statement>, ident: String) {
        assert_eq!(
            String::from("let"),
            stmt.token_literal(),
            "stmt.token_literal not 'let'. got={}",
            stmt.token_literal()
        );

        let let_stmt = stmt.as_any().downcast_ref::<ast::LetStatement>();
        assert!(let_stmt.is_some(), "stmt not ast::LetStatement");

        let let_stmt = let_stmt.unwrap();
        assert!(let_stmt.name.is_some(), "let_stmt.name is None");

        let let_stmt_name = let_stmt.name.as_ref().unwrap().clone();
        assert_eq!(
            ident, let_stmt_name.value,
            "let_stmt.name.value not '{}'. got={}",
            ident, let_stmt_name.value
        );

        assert_eq!(
            ident,
            let_stmt_name.token_literal(),
            "let_stmt.name.token_literal() not '{}'. got={}",
            ident,
            let_stmt_name.token_literal()
        );
    }

    fn check_parser_errors(parser: &Parser) {
        let errors = parser.errors();
        if errors.len() == 0 {
            return;
        }
        eprintln!("parser has {} errors", errors.len());
        for msg in errors.iter() {
            eprintln!("parser error: {}", msg);
        }
        panic!("");
    }
}
