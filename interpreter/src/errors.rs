use core::fmt;
use file_system::errors::FileSystem;
use parser::errors::ParserError;
use tokenizer::errors::LexerError;
use validator::errors::ValidatorError;

#[derive(Debug)]
pub enum InterPreterError {
    Io(FileSystem),
    Lexer(LexerError),
    Parser(ParserError),
    Validator(ValidatorError),
}

impl fmt::Display for InterPreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterPreterError::Io(e) => write!(f, "{}", e),
            InterPreterError::Lexer(e) => write!(f, " {}", e),
            InterPreterError::Parser(e) => write!(f, "{}", e),
            InterPreterError::Validator(e) => write!(f, "Dəmir Əmi: {}", e),
        }
    }
}

impl From<FileSystem> for InterPreterError {
    fn from(e: FileSystem) -> Self {
        InterPreterError::Io(e)
    }
}

impl From<ValidatorError> for InterPreterError {
    fn from(e: ValidatorError) -> Self {
        InterPreterError::Validator(e)
    }
}
