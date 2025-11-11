pub mod file_system;
pub mod parser_errors;
use core::fmt;
pub use file_system::FileSystem;
pub use parser_errors::ParserError;
use std::fmt::{Debug, Display};

pub trait Errors: Debug + Display {}

impl Errors for ParserError {}
impl Errors for FileSystem {}

#[derive(Debug)]
pub enum InterpreterError {
    Io(FileSystem),
    Parser(ParserError),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::Io(e) => write!(f, "{}", e),
            InterpreterError::Parser(e) => write!(f, "Böyük Qardaş: {}", e),
        }
    }
}

impl Errors for InterpreterError {}

impl From<FileSystem> for InterpreterError {
    fn from(e: FileSystem) -> Self {
        InterpreterError::Io(e)
    }
}

impl From<ParserError> for InterpreterError {
    fn from(e: ParserError) -> Self {
        InterpreterError::Parser(e)
    }
}
