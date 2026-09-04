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

    fn advance(&mut self, pos: usize, line: usize, column: usize) {
        self.pos += pos;
        self.line += line;
        self.column += column;
    }

    fn check_whitespace(&mut self) {
        if self.source_code[self.pos] == ' ' {
            self.advance(1, 1, 1);
        }
    }

    fn check_value(&mut self, tokens: &mut Vec<Token>) -> io::Result<()> {
        let mut number = String::new();

        while self.pos < self.source_code.len() && self.source_code[self.pos].is_ascii_digit() {
            number.push(self.source_code[self.pos]);
            self.advance(1, 1, 1);
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

                self.advance(1, 1, 1);
            },
            ')' => {
                tokens.push(Token {
                    token_type: TokenType::RParen,
                    line: self.line,
                    column: self.column,
                });

                self.advance(1, 1, 1);
            },
            ';' => {
                tokens.push(Token {
                    token_type: TokenType::Semicolon,
                    line: self.line,
                    column: self.column,
                });

                self.advance(1, 1, 1);
            },
            '=' => {
                tokens.push(Token {
                    token_type: TokenType::Assign,
                    line: self.line,
                    column: self.column,
                });

                self.advance(1, 1, 1);
            },
            _ => {}
        }
    }

    fn check_keyword(&mut self, tokens: &mut Vec<Token>) {
        let mut keyword = String::new();

        while self.pos < self.source_code.len() && (
            self.source_code[self.pos].is_alphanumeric() || self.source_code[self.pos] == '!'
        ) {
            keyword.push(self.source_code[self.pos]);
            self.advance(1, 1, 1);
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
            self.check_punctuation(&mut tokens);
            self.check_keyword(&mut tokens);
            
            match self.check_value(&mut tokens) {
                Ok(_) => {},
                Err(_) => {}
            }
        }

        println!("Lexing completed....\n");

        tokens
    }
}