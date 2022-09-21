use std::io;

mod token;
mod lexer;
mod repl;

fn main() {
    repl::start(io::stdin(), io::stdout());
}
