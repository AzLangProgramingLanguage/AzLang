use core::fmt;
use file_system::errors::FileSystem;
use parser::errors::ParserError;
use validator::errors::ValidatorError;

#[derive(Debug)]
pub enum CompilerError {
    Io(FileSystem),
    Parser(ParserError),
    Validator(ValidatorError),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::Io(e) => write!(f, "{}", e),
            CompilerError::Parser(e) => write!(f, "Böyük Qardaş: {}", e),
            CompilerError::Validator(e) => write!(f, "Dəmir Əmi: {}", e),
        }
    }
}

impl From<FileSystem> for CompilerError {
    fn from(e: FileSystem) -> Self {
        CompilerError::Io(e)
    }
}

impl From<ValidatorError> for CompilerError {
    fn from(e: ValidatorError) -> Self {
        CompilerError::Validator(e)
    }
}
