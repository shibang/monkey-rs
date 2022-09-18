#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,
    // ident + literal
    Ident,
    Int,

    // operator
    Assign,
    Plus,

    // separator
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // keywords
    Function,
    Let,
}

impl AsRef<str> for TokenType {
    fn as_ref(&self) -> &str {
        match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::Eof => "EOF",
            TokenType::Ident => "IDENT",
            TokenType::Int => "INT",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",
            TokenType::Function => "FUNCTION",
            TokenType::Let => "LET"
        }
    }
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        _ => TokenType::Ident
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: impl ToString) -> Token {
        Token{
            token_type,
            literal: literal.to_string(),
        }
    }
}

