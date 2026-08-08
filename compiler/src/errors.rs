use std::fmt::Display;

use file_system::errors::FileSystemError;
use parser::errors::ParserError;
use validator::errors::ValidatorError;

#[derive(Debug, PartialEq)]
pub enum BackendError {
    Qbe,
    Asembly,
}
impl Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendError::Qbe => writeln!(f, "QBE doesn't installed"),
            BackendError::Asembly => writeln!(f, "binutils doesn't installed"),
        }
    }
}

#[derive(Debug)]
pub enum CompilerError {
    IO(FileSystemError),
    Parser(ParserError),
    Validator(ValidatorError),
    Backend(BackendError),
}
impl CompilerError {
    pub fn display(&self) {
        match self {
            CompilerError::IO(e) => {
                println!("\x1b[1;31m[Big Brother]:\x1b[0m {} ", e);
            }
            CompilerError::Parser(e) => println!("\x1b[31m[Big Brother]:\x1b[0m {}", e),
            CompilerError::Validator(e) => println!("\x1b[33m[Validator]:\x1b[0m {}", e),
            CompilerError::Backend(e) => println!("\x1b[34m[Compiler]:\x1b[0m {}", e),
        }
    }
    pub fn code(&self) -> i32 {
        match self {
            CompilerError::IO(e) => e.code(),
            CompilerError::Parser(_) => 34,
            CompilerError::Validator(_) => 35,
            CompilerError::Backend(_) => 36,
        }
    }
}

impl From<FileSystemError> for CompilerError {
    fn from(e: FileSystemError) -> Self {
        CompilerError::IO(e)
    }
}

impl From<ValidatorError> for CompilerError {
    fn from(e: ValidatorError) -> Self {
        CompilerError::Validator(e)
    }
}
impl From<ParserError> for CompilerError {
    fn from(e: ParserError) -> Self {
        CompilerError::Parser(e)
    }
}




