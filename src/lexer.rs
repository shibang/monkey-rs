use crate::token;
use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    pos: usize,
    read_pos: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            pos: 0,
            read_pos: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_pos).unwrap();
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_pos >= self.input.len() {
            return '\0';
        }
        self.input.chars().nth(self.read_pos).unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::Eq, "==")
                } else {
                    Token::new(TokenType::Assign, "=")
                }
            }
            ';' => Token::new(TokenType::Semicolon, ";"),
            '(' => Token::new(TokenType::LParen, "("),
            ')' => Token::new(TokenType::RParen, ")"),
            ',' => Token::new(TokenType::Comma, ","),
            '+' => Token::new(TokenType::Plus, "+"),
            '-' => Token::new(TokenType::Minus, "-"),
            '{' => Token::new(TokenType::LBrace, "{"),
            '}' => Token::new(TokenType::RBrace, "}"),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::NotEq, "!=")
                } else {
                    Token::new(TokenType::Bang, "!")
                }
            }
            '/' => Token::new(TokenType::Slash, "/"),
            '*' => Token::new(TokenType::Asterisk, "*"),
            '<' => Token::new(TokenType::Lt, "<"),
            '>' => Token::new(TokenType::Gt, ">"),
            '\0' => Token::new(TokenType::Eof, "\0"),
            _letter if self.is_letter() => {
                let literal = self.read_identifier();
                let token_type = token::lookup_ident(&literal);
                return Token::new(token_type, literal); // return is necessary
            }
            _number if self.ch.is_ascii_digit() => {
                let literal = self.read_number();
                return Token::new(TokenType::Int, literal);
            }
            _ => {
                return Token::new(TokenType::Illegal, ""); // return is necessary
            }
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> &str {
        let pos = self.pos;
        while self.is_letter() {
            self.read_char();
        }
        &self.input[pos..self.pos]
    }

    fn skip_whitespace(&mut self) {
        while [' ', '\t', '\r', '\n'].contains(&self.ch) {
            self.read_char()
        }
    }

    fn read_number(&mut self) -> &str {
        let pos = self.pos;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        &self.input[pos..self.pos]
    }

    fn is_letter(&self) -> bool {
        self.ch.is_ascii_alphabetic() || self.ch == '_'
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn next_token() {
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
        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Gt, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::LParen, "("),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Int, "10"),
            (TokenType::Eq, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "10"),
            (TokenType::NotEq, "!="),
            (TokenType::Int, "9"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, "\0"),
        ];

        let mut lexer = Lexer::new(input.to_string());
        for (i, tt) in tests.iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(
                token.token_type, tt.0,
                "tests[{}] - token type wrong. expected={:?}, got={:?}",
                i, tt.0, token.token_type
            );
            assert_eq!(
                token.literal, tt.1,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, tt.1, token.literal
            );
        }
    }
}
