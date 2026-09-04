// crate/src/errors/variables.rs

use super::Error;

pub enum VariableErrors {
    ExpectedIdentiefier,
    ExpectedAssign,
    ExpectedValue,
}

impl Error for VariableErrors {
    fn display(error_type: Self, line: &usize, column: &usize) {
        match error_type {
            VariableErrors::ExpectedAssign => {
                println!("Syntax Error: the compiler panicked at line {} column {}\n", line, column);
                println!("Syntax Error: Expected Assign `=`");
            },
            VariableErrors::ExpectedValue => {
                println!("Syntax Error: the compiler panicked at line {} column {}\n", line, column);
                println!("Syntax Error: Expected Value `i64`");
            },
            VariableErrors::ExpectedIdentiefier => {
                println!("Syntax Error: the compiler panicked at line {} column {}\n", line, column);
                println!("Syntax Error: Expected Identifier `variable_name`");
            },
        }
    }
}