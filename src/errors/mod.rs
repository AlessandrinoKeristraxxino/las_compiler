// crate/src/errors/mod.rs

pub mod variables;
pub mod general;

pub trait Error {
    fn display(error_type: Self, line: &usize, column: &usize);
}
