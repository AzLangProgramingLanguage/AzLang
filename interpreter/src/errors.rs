use crate::validator::errors::ValidatorError;
use core::fmt;
use file_system::errors::FileSystem;
use parser::errors::ParserError;

// NOTE: parser::errors::ParserError istifadə etməyə ehtiyac yoxdur
// biz ParserError-i interpreter səviyyəsində string-ləşdirəcəyik.

#[derive(Debug)]
pub enum InterPreterError {
    Io(FileSystem),
    Parser(ParserError),
    Validator(ValidatorError),
}

impl fmt::Display for InterPreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterPreterError::Io(e) => write!(f, "{}", e),
            InterPreterError::Parser(e) => write!(f, "Böyük Qardaş: {}", e),
            InterPreterError::Validator(e) => write!(f, "Dəmir Əmi: {}", e),
        }
    }
}

impl From<FileSystem> for InterPreterError {
    fn from(e: FileSystem) -> Self {
        InterPreterError::Io(e)
    }
}

// From<ValidatorError> saxla əgər lazım olsa
impl From<ValidatorError> for InterPreterError {
    fn from(e: ValidatorError) -> Self {
        InterPreterError::Validator(e)
    }
}
