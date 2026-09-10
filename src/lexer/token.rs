// crate/src/lexer/token.rs

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Print,
    Println,

    Let,
    Identifier(String),

    Assign,
    Semicolon,
    LParen,
    RParen,

    Quote,

    Number(i64),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}
