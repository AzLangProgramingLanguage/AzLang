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
pub enum InterPreterError {
    Io(FileSystem),
    Parser(ParserError),
}

impl fmt::Display for InterPreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterPreterError::Io(e) => write!(f, "{}", e),
            InterPreterError::Parser(e) => write!(f, "Böyük Qardaş: {}", e),
        }
    }
}

impl Errors for InterPreterError {}

impl From<FileSystem> for InterPreterError {
    fn from(e: FileSystem) -> Self {
        InterPreterError::Io(e)
    }
}

impl From<ParserError> for InterPreterError {
    fn from(e: ParserError) -> Self {
        InterPreterError::Parser(e)
    }
}
