use core::fmt;
use std::fmt::Display;
use parser::errors::ParserError;
use tokenizer::errors::LexerError;
use validator::errors::ValidatorError;

#[derive(Debug)]
pub enum CompilerError {
    BuildError,
}
impl Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::BuildError => write!(f, "Program kompilyasiya edilərkən xəta baş verdi"),
        }
    }
    
}

