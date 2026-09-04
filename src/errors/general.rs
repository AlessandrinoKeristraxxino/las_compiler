// crate/src/errors/general.rs

use super::Error;

#[derive(Debug, Clone)]
pub enum GeneralErrors {
    ExpectedSemicolon,
    ExpectedLParen,
    ExpectedRParen,
}

impl Error for GeneralErrors {
    fn display(error_type: Self, line: &usize, column: &usize) {
        match error_type {
            GeneralErrors::ExpectedSemicolon => {
                println!("Syntax Error: the compiler panicked at line {} column {}\n", line, column);
                println!("Syntax Error: Expected Semicolon `;`");
            },
            GeneralErrors::ExpectedLParen => {
                println!("Syntax Error: the compiler panicked at line {} column {}\n", line, column);
                println!("Syntax Error: Expected LParen `(`");
            },
            GeneralErrors::ExpectedRParen => {
                println!("Syntax Error: the compiler panicked at line {} column {}\n", line, column);
                println!("Syntax Error: Expected RParen `)`");
            }
        }
    }
}