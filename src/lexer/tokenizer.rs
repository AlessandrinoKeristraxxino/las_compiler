// crate/src/lexer/tokenizer.rs

use std::io;

use crate::lexer::token::{Token, TokenType};

pub struct Lexer {
    pub pos: usize,
    pub line: usize,
    pub column: usize,
    pub source_code: Vec<char>,
}

impl Lexer {
    pub fn new(source_code: &Vec<char>) -> Self {
        Self {
            pos: 0,
            line: 0,
            column: 0,
            source_code: source_code.clone(),
        }
    }

    fn advance(&mut self) {
        if self.pos < self.source_code.len() {
            if self.source_code[self.pos] == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }

            self.pos += 1;
        }
    }

    fn check_whitespace(&mut self) {
        while self.pos < self.source_code.len() && self.source_code[self.pos].is_whitespace() {
            self.advance();
        }
    }

    fn check_value(&mut self, tokens: &mut Vec<Token>) -> io::Result<()> {
        let mut number = String::new();

        while self.pos < self.source_code.len() && self.source_code[self.pos].is_ascii_digit() {
            number.push(self.source_code[self.pos]);
            self.advance();
        }

        let value = number.parse::<i64>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        tokens.push(Token {
            token_type: TokenType::Value(value),
            line: self.line,
            column: self.column,
        });

    Ok(())
}

    fn check_punctuation(&mut self, tokens: &mut Vec<Token>) {
        match self.source_code[self.pos] {
            '(' => {
                tokens.push(Token {
                    token_type: TokenType::LParen,
                    line: self.line,
                    column: self.column,
                });

                self.advance();
            },
            ')' => {
                tokens.push(Token {
                    token_type: TokenType::RParen,
                    line: self.line,
                    column: self.column,
                });

                self.advance();
            },
            ';' => {
                tokens.push(Token {
                    token_type: TokenType::Semicolon,
                    line: self.line,
                    column: self.column,
                });

                self.advance();
            },
            '=' => {
                tokens.push(Token {
                    token_type: TokenType::Assign,
                    line: self.line,
                    column: self.column,
                });

                self.advance();
            },
            _ => {}
        }
    }

    fn check_keyword(&mut self, tokens: &mut Vec<Token>) {
        if !self.source_code[self.pos].is_alphabetic() {
            return;
        }

        let mut keyword = String::new();

        while self.pos < self.source_code.len() && (
            self.source_code[self.pos].is_alphanumeric() || self.source_code[self.pos] == '!'
        ) {
            keyword.push(self.source_code[self.pos]);
            self.advance();
        }

        match keyword.as_str() {
            "let" => {
                tokens.push(Token {
                    token_type: TokenType::Let,
                    line: self.line,
                    column: self.column,
                });
            },
            "print!" => {
                tokens.push(Token {
                    token_type: TokenType::Print,
                    line: self.line,
                    column: self.column,
                });
            },
            "println!" => {
                tokens.push(Token {
                    token_type: TokenType::Println,
                    line: self.line,
                    column: self.column,
                });
            },
            identifier => {
                tokens.push(Token {
                    token_type: TokenType::Identifier(identifier.to_string()),
                    line: self.line,
                    column: self.column,
                });
            }
        }
    }

    pub fn lexing(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.pos < self.source_code.len() {
            self.check_whitespace();

            if self.pos >= self.source_code.len() {
                break;
            }

            let c = self.source_code[self.pos];

            if c.is_ascii_digit() {
                let _ = self.check_value(&mut tokens);
            } else if c.is_alphabetic() {
                self.check_keyword(&mut tokens);
            } else {
                self.check_punctuation(&mut tokens);
            }
        }

        println!("{:#?}", tokens);
        println!("Lexing completed....\n");

        tokens
    }
}