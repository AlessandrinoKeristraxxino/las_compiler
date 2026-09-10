// crate/src/parser/ast.rs

use crate::lexer::token::{Token, TokenType};
use crate::errors::{
    Error,
    general::GeneralErrors,
    variables::VariableErrors,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(i64),
    String(String),
    Variable(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt { 
    Let {
        name: String,
        value: Expr,
    },
    Print(Expr),
    Println(Option<Expr>),
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Self {
            tokens: tokens.clone(),
            current: 0,
        }
    }

    fn parse_statement(&mut self, statements: &mut Vec<Stmt>) -> (bool, i32) {
        let mut is_panicked = false;
        let mut errors = 0;

        if self.current < self.tokens.len() {
            match &self.tokens[self.current] {
                Token {
                    token_type: TokenType::Let,
                    line: _ident_line,
                    column: _ident_column,
                    ..
                } => {
                    self.current += 1;
                    let (line, column) = self.get_pos();

                    if self.current < self.tokens.len() {
                        match &self.tokens[self.current] {
                            // IDENTIFIER
                            Token {
                                token_type: TokenType::Identifier(value),
                                line: _ident_line,
                                column: _ident_column,
                                ..
                            } => {
                                let name = value.clone();
                                self.current += 1;
                                let (line, column) = self.get_pos();

                                if self.current < self.tokens.len() {
                                    match &self.tokens[self.current] {
                                        // ASSIGN
                                        Token {
                                            token_type: TokenType::Assign,
                                            line: _ident_line,
                                            column: _ident_column,
                                            ..
                                        } => {
                                            self.current += 1;
                                            let (line, column) = self.get_pos();

                                            if self.current < self.tokens.len() {
                                                match &self.tokens[self.current] {
                                                    // NUMBER
                                                    Token {
                                                        token_type: TokenType::Number(number),
                                                        line: _ident_line,
                                                        column: _ident_column,
                                                        ..
                                                    } => {
                                                        self.current += 1;
                                                        let (line, column) = self.get_pos();

                                                        match self.check_semicolon() {
                                                            true => {
                                                                statements.push(Stmt::Let { 
                                                                    name,
                                                                    value: Expr::Number(*number)
                                                                });
                                                            },
                                                            false => {
                                                                GeneralErrors::display(
                                                                    GeneralErrors::ExpectedSemicolon,
                                                                    &line,
                                                                    &column
                                                                );

                                                                is_panicked = true;
                                                                errors += 1;
                                                            }
                                                        }
                                                    },
                                                    // QUOTE
                                                    Token {
                                                        token_type: TokenType::Quote,
                                                        line: _ident_line,
                                                        column: _ident_column,
                                                        ..
                                                    } => {
                                                        self.current += 1;
                                                        let (line, column) = self.get_pos();

                                                        if self.current < self.tokens.len() {
                                                            match &self.tokens[self.current] {
                                                                // STRING
                                                                Token {
                                                                    token_type: TokenType::String(string),
                                                                    line: _ident_line,
                                                                    column: _ident_column,
                                                                    ..
                                                                } => {
                                                                    self.current += 1;
                                                                    let (line, column) = self.get_pos();

                                                                    if self.current < self.tokens.len() {
                                                                        match &self.tokens[self.current] {
                                                                            Token {
                                                                                token_type: TokenType::Quote,
                                                                                line: _ident_line,
                                                                                column: _ident_column,
                                                                            } => {
                                                                                self.current += 1;
                                                                                let (line, column) = self.get_pos();

                                                                                match self.check_semicolon() {
                                                                                    true => {
                                                                                        statements.push(Stmt::Let { 
                                                                                            name,
                                                                                            value: Expr::String(string.clone()),
                                                                                        });
                                                                                    },
                                                                                    false => {
                                                                                        GeneralErrors::display(
                                                                                            GeneralErrors::ExpectedSemicolon,
                                                                                            &line,
                                                                                            &column
                                                                                        );

                                                                                        is_panicked = true;
                                                                                        errors += 1;
                                                                                    }
                                                                                }
                                                                            },
                                                                            _ => {
                                                                                GeneralErrors::display(
                                                                                    GeneralErrors::ExpectedQuote,
                                                                                    &line,
                                                                                    &column,
                                                                                );

                                                                                is_panicked = true;
                                                                                errors += 1;
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                _ => {
                                                                    VariableErrors::display(
                                                            VariableErrors::ExpectedString,
                                                                        &line,
                                                                        &column,
                                                                    );

                                                                    is_panicked = true;
                                                                    errors += 1;
                                                                }
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        VariableErrors::display(
                                                            VariableErrors::ExpectedNumber,
                                                            &line,
                                                            &column
                                                        );
                                                        GeneralErrors::display(
                                                            GeneralErrors::ExpectedQuote,
                                                            &line,
                                                            &column,
                                                        );

                                                        is_panicked = true;
                                                        errors += 1;
                                                    }
                                                }
                                            }
                                        },
                                        _ => {
                                            VariableErrors::display(
                                                VariableErrors::ExpectedAssign,
                                                &line,
                                                &column
                                            );

                                            is_panicked = true;
                                            errors += 1;
                                        }
                                    }
                                }
                            }
                            _ => {
                                VariableErrors::display(
                                    VariableErrors::ExpectedIdentiefier,
                                    &line,
                                    &column,
                                );

                                is_panicked = true;
                                errors += 1;
                            }
                        }
                    } else {
                        VariableErrors::display(
                            VariableErrors::ExpectedIdentiefier,
                            &line,
                            &column,
                        );

                        is_panicked = true;
                        errors += 1;
                    }
                },
                // PRINT
                Token {
                    token_type: TokenType::Print,
                    line: _ident_line,
                    column: _ident_column,
                    ..
                } => {
                    self.current += 1;
                    let (line, column) = self.get_pos();

                    if self.current < self.tokens.len() {
                        match &self.tokens[self.current] {
                            // LPAREN
                            Token {
                                token_type: TokenType::LParen,
                                line: _ident_line,
                                column: _ident_column,
                                ..
                            } => {
                                self.current += 1;
                                let (line, column) = self.get_pos();

                                if self.current < self.tokens.len() {
                                    match &self.tokens[self.current] {
                                        // IDENTIFIER
                                        Token {
                                            token_type: TokenType::Identifier(var_name),
                                            line: _ident_line,
                                            column: _ident_column,
                                            ..
                                        } => {
                                            self.current += 1;
                                            let (line, column) = self.get_pos();
                                            
                                            if self.current < self.tokens.len() {
                                                match &self.tokens[self.current] {
                                                    // RPAREN
                                                    Token {
                                                        token_type: TokenType::RParen,
                                                        line: _ident_line,
                                                        column: _ident_column,
                                                        ..
                                                    } => {
                                                        self.current += 1;
                                                        let (line, column) = self.get_pos();

                                                        match self.check_semicolon() {
                                                            true => {
                                                                statements.push(Stmt::Print(Expr::Variable(var_name.clone())));
                                                            },
                                                            false => {
                                                                GeneralErrors::display(
                                                                    GeneralErrors::ExpectedSemicolon,
                                                                    &line,
                                                                    &column
                                                                );

                                                                is_panicked = true;
                                                                errors += 1;
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        GeneralErrors::display(
                                                            GeneralErrors::ExpectedRParen,
                                                            &line,
                                                            &column
                                                        );

                                                        is_panicked = true;
                                                        errors += 1;
                                                    }
                                                }
                                            }
                                        },
                                        // NUMBER
                                        Token {
                                            token_type: TokenType::Number(number),
                                            line: _ident_line,
                                            column: _ident_column,
                                            ..
                                        } => {
                                            self.current += 1;
                                            let (line, column) = self.get_pos();
                                            
                                            if self.current < self.tokens.len() {
                                                match &self.tokens[self.current] {
                                                    // RPAREN
                                                    Token {
                                                        token_type: TokenType::RParen,
                                                        line: _ident_line,
                                                        column: _ident_column,
                                                        ..
                                                    } => {
                                                        self.current += 1;
                                                        let (line, column) = self.get_pos();

                                                        match self.check_semicolon() {
                                                            true => {
                                                                statements.push(Stmt::Print(Expr::Number(*number)));
                                                            },
                                                            false => {
                                                                GeneralErrors::display(
                                                                    GeneralErrors::ExpectedSemicolon,
                                                                    &line,
                                                                    &column
                                                                );

                                                                is_panicked = true;
                                                                errors += 1;
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        GeneralErrors::display(
                                                            GeneralErrors::ExpectedRParen,
                                                            &line,
                                                            &column
                                                        );

                                                        is_panicked = true;
                                                        errors += 1;
                                                    }
                                                }
                                            }
                                        },
                                        // QUOTE
                                        Token {
                                            token_type: TokenType::Quote,
                                            line: _ident_line,
                                            column: _ident_column,
                                            ..
                                        } => {
                                            self.current += 1;
                                            let (line, column) = self.get_pos();

                                            if self.current < self.tokens.len() {
                                                match &self.tokens[self.current] {
                                                // STRING
                                                    Token {
                                                        token_type: TokenType::String(string),
                                                        line: _ident_line,
                                                        column: _ident_column,
                                                        ..
                                                    } => {
                                                        self.current += 1;
                                                        let (line, column) = self.get_pos();

                                                        if self.current < self.tokens.len() {
                                                            match &self.tokens[self.current] {
                                                                Token {
                                                                    token_type: TokenType::Quote,
                                                                    line: _ident_line,
                                                                    column: _ident_column,
                                                                } => {
                                                                    self.current += 1;
                                                                    let (line, column) = self.get_pos();
                                                                    
                                                                    if self.current < self.tokens.len() {
                                                                        match &self.tokens[self.current] {
                                                                            Token {
                                                                                token_type: TokenType::RParen,
                                                                                line: _ident_line,
                                                                                column: _ident_column,
                                                                            } => {
                                                                                self.current += 1;
                                                                                let (line, column) = self.get_pos();

                                                                                match self.check_semicolon() {
                                                                                    true => {
                                                                                        statements.push(Stmt::Print(Expr::String(string.clone())));
                                                                                    },
                                                                                    false => {
                                                                                        GeneralErrors::display(
                                                                                            GeneralErrors::ExpectedSemicolon,
                                                                                            &line,
                                                                                            &column,
                                                                                        );

                                                                                        is_panicked = true;
                                                                                        errors += 1;
                                                                                    },
                                                                                }
                                                                            },
                                                                            _ => {
                                                                                GeneralErrors::display(
                                                                                    GeneralErrors::ExpectedRParen,
                                                                                    &line,
                                                                                    &column,
                                                                                );

                                                                                is_panicked = true;
                                                                                errors += 1;
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                _ => {
                                                                    GeneralErrors::display(
                                                                        GeneralErrors::ExpectedQuote,
                                                                        &line,
                                                                        &column,
                                                                    );

                                                                    is_panicked = true;
                                                                    errors += 1;
                                                                }
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        VariableErrors::display(
                                                VariableErrors::ExpectedString,
                                                            &line,
                                                            &column,
                                                        );

                                                        is_panicked = true;
                                                        errors += 1;
                                                    }
                                                }
                                            }
                                        },
                                        _ => {
                                            VariableErrors::display(
                                    VariableErrors::ExpectedIdentiefier,
                                                &line,
                                                &column
                                            );
                                            GeneralErrors::display(
                                                GeneralErrors::ExpectedQuote,
                                                &line,
                                                &column,
                                            );

                                            is_panicked = true;
                                            errors += 1;                        
                                        }
                                    }
                                }
                            },
                            _ => {
                                GeneralErrors::display(
                                    GeneralErrors::ExpectedLParen, 
                                    &line, 
                                    &column
                                );

                                is_panicked = true;
                                errors += 1;
                            }
                        }
                    }
                },
                Token {
                    token_type: TokenType::Println,
                    line: _ident_line,
                    column: _ident_column,
                    ..
                } => {
                    self.current += 1;
                    let (line, column) = self.get_pos();

                    if self.current < self.tokens.len() {
                        match &self.tokens[self.current] {
                            // LPAREN
                            Token {
                                token_type: TokenType::LParen,
                                line: _ident_line,
                                column: _ident_column,
                                ..
                            } => {
                                self.current += 1;
                                let (line, column) = self.get_pos();

                                if self.current < self.tokens.len() {
                                    match &self.tokens[self.current] {
                                        // EMPTY ARGUMENT LIST
                                        Token {
                                            token_type: TokenType::RParen,
                                            ..
                                        } => {
                                            self.current += 1;
                                            let (line, column) = self.get_pos();

                                            match self.check_semicolon() {
                                                true => {
                                                    statements.push(Stmt::Println(None));
                                                },
                                                false => {
                                                    GeneralErrors::display(
                                                        GeneralErrors::ExpectedSemicolon,
                                                        &line,
                                                        &column
                                                    );

                                                    is_panicked = true;
                                                    errors += 1;
                                                }
                                            }
                                        },
                                        // IDENTIFIER
                                        Token {
                                            token_type: TokenType::Identifier(var_name),
                                            line: _ident_line,
                                            column: _ident_column,
                                            ..
                                        } => {
                                            self.current += 1;
                                            let (line, column) = self.get_pos();
                                            
                                            if self.current < self.tokens.len() {
                                                match &self.tokens[self.current] {
                                                    // RPAREN
                                                    Token {
                                                        token_type: TokenType::RParen,
                                                        line: _ident_line,
                                                        column: _ident_column,
                                                        ..
                                                    } => {
                                                        self.current += 1;
                                                        let (line, column) = self.get_pos();

                                                        match self.check_semicolon() {
                                                            true => {
                                                                statements.push(Stmt::Println(Some(Expr::Variable(var_name.clone()))));
                                                            },
                                                            false => {
                                                                GeneralErrors::display(
                                                                    GeneralErrors::ExpectedSemicolon,
                                                                    &line,
                                                                    &column
                                                                );

                                                                is_panicked = true;
                                                                errors += 1;
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        GeneralErrors::display(
                                                            GeneralErrors::ExpectedRParen,
                                                            &line,
                                                            &column
                                                        );

                                                        is_panicked = true;
                                                        errors += 1;
                                                    }
                                                }
                                            }
                                        },
                                        // NUMBER
                                        Token {
                                            token_type: TokenType::Number(number),
                                            line: _ident_line,
                                            column: _ident_column,
                                            ..
                                        } => {
                                            self.current += 1;
                                            let (line, column) = self.get_pos();
                                            
                                            if self.current < self.tokens.len() {
                                                match &self.tokens[self.current] {
                                                    // RPAREN
                                                    Token {
                                                        token_type: TokenType::RParen,
                                                        line: _ident_line,
                                                        column: _ident_column,
                                                        ..
                                                    } => {
                                                        self.current += 1;
                                                        let (line, column) = self.get_pos();

                                                        match self.check_semicolon() {
                                                            true => {
                                                                statements.push(Stmt::Println(Some(Expr::Number(*number))));
                                                            },
                                                            false => {
                                                                GeneralErrors::display(
                                                                    GeneralErrors::ExpectedSemicolon,
                                                                    &line,
                                                                    &column
                                                                );

                                                                is_panicked = true;
                                                                errors += 1;
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        GeneralErrors::display(
                                                            GeneralErrors::ExpectedRParen,
                                                            &line,
                                                            &column
                                                        );

                                                        is_panicked = true;
                                                        errors += 1;
                                                    }
                                                }
                                            }
                                        },
                                        // QUOTE
                                        Token {
                                            token_type: TokenType::Quote,
                                            line: _ident_line,
                                            column: _ident_column,
                                            ..
                                        } => {
                                            self.current += 1;
                                            let (line, column) = self.get_pos();

                                            if self.current < self.tokens.len() {
                                                match &self.tokens[self.current] {
                                                // STRING
                                                    Token {
                                                        token_type: TokenType::String(string),
                                                        line: _ident_line,
                                                        column: _ident_column,
                                                        ..
                                                    } => {
                                                        self.current += 1;
                                                        let (line, column) = self.get_pos();

                                                        if self.current < self.tokens.len() {
                                                            match &self.tokens[self.current] {
                                                                Token {
                                                                    token_type: TokenType::Quote,
                                                                    line: _ident_line,
                                                                    column: _ident_column,
                                                                } => {
                                                                    self.current += 1;
                                                                    let (line, column) = self.get_pos();
                                                                    
                                                                    if self.current < self.tokens.len() {
                                                                        match &self.tokens[self.current] {
                                                                            Token {
                                                                                token_type: TokenType::RParen,
                                                                                line: _ident_line,
                                                                                column: _ident_column,
                                                                            } => {
                                                                                self.current += 1;
                                                                                let (line, column) = self.get_pos();

                                                                                match self.check_semicolon() {
                                                                                    true => {
                                                                                        statements.push(Stmt::Println(Some(Expr::String(string.clone()))));
                                                                                    },
                                                                                    false => {
                                                                                        GeneralErrors::display(
                                                                                            GeneralErrors::ExpectedSemicolon,
                                                                                            &line,
                                                                                            &column,
                                                                                        );

                                                                                        is_panicked = true;
                                                                                        errors += 1;
                                                                                    },
                                                                                }
                                                                            },
                                                                            _ => {
                                                                                GeneralErrors::display(
                                                                                    GeneralErrors::ExpectedRParen,
                                                                                    &line,
                                                                                    &column,
                                                                                );

                                                                                is_panicked = true;
                                                                                errors += 1;
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                _ => {
                                                                    GeneralErrors::display(
                                                                        GeneralErrors::ExpectedQuote,
                                                                        &line,
                                                                        &column,
                                                                    );

                                                                    is_panicked = true;
                                                                    errors += 1;
                                                                }
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        VariableErrors::display(
                                                VariableErrors::ExpectedString,
                                                            &line,
                                                            &column,
                                                        );

                                                        is_panicked = true;
                                                        errors += 1;
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            VariableErrors::display(
                                    VariableErrors::ExpectedIdentiefier,
                                                &line,
                                                &column
                                            );
                                            VariableErrors::display(
                                                VariableErrors::ExpectedNumber,
                                                &line,
                                                &column,
                                            );
                                            GeneralErrors::display(
                                                GeneralErrors::ExpectedQuote,
                                                &line,
                                                &column,
                                            );

                                            is_panicked = true;
                                            errors += 1;                        
                                        }
                                    }
                                }
                            },
                            _ => {
                                GeneralErrors::display(
                                    GeneralErrors::ExpectedLParen, 
                                    &line, 
                                    &column
                                );

                                is_panicked = true;
                                errors += 1;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        (is_panicked, errors)
    }

    fn check_semicolon(&self) -> bool {
        if self.current < self.tokens.len() {
            match &self.tokens[self.current] {
                Token {
                    token_type: TokenType::Semicolon,
                    line: _ident_line,
                    column: _ident_column,
                    ..
                } => {
                    return true;
                },
                _ => {
                    return false;
                }
            }
        } else {
            return false;
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        return (self.tokens[self.current].line, self.tokens[self.current].column);
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        
        let mut errors = 0;
        let mut is_panicked = false;

        while self.current < self.tokens.len() {
            let (panicked, errors_temp) = self.parse_statement(&mut statements);
            self.current += 1;

            if panicked {
                is_panicked = true;
            }
            errors += errors_temp;
        }

        if is_panicked {
            eprintln!("Program panicked with {} errors.", errors);
            eprintln!("Retry!");
            std::process::exit(1);
        }

        statements
    }
}