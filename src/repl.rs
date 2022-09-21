use std::io;
use std::io::{BufRead, BufReader};
use crate::lexer::Lexer;
use crate::token::TokenType;

const PROMPT: &str = ">> ";

pub fn start(input: impl io::Read, mut output: impl io::Write) {
    let mut reader = BufReader::new(input);
    loop {
        write!(output, "{}", PROMPT).unwrap();
        output.flush().unwrap();

        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    return
                }
            }
            Err(_) => {
                return
            }
        }

        let mut lexer = Lexer::new(line);
        let mut token = lexer.next_token();
        while token.token_type != TokenType::Eof {
            writeln!(output, "{:?}", token).unwrap();
            token = lexer.next_token();
        }
    }
}