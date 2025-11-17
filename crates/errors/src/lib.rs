pub mod file_system;
pub mod parser_errors;
pub mod validator_error;
use core::fmt;
pub use file_system::FileSystem;
pub use parser_errors::ParserError;
use std::fmt::{Debug, Display};

use crate::validator_error::ValidatorError;

pub trait Errors: Debug + Display {}

impl Errors for ParserError {}
impl Errors for FileSystem {}

#[derive(Debug)]
pub enum InterPreterError {
    Io(FileSystem),
    Parser(ParserError),
    Validator(ValidatorError),
}

impl fmt::Display for InterPreterError {
    /* TODO:  Əslində burası lazımsızdır */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterPreterError::Io(e) => write!(f, "{}", e),
            InterPreterError::Parser(e) => write!(f, "Böyük Qardaş: {}", e),
            InterPreterError::Validator(e) => write!(f, "Dəmir Əmi: {}", e),
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

impl From<ValidatorError> for InterPreterError {
    fn from(e: ValidatorError) -> Self {
        InterPreterError::Validator(e)
    }
}
