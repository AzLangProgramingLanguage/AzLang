use file_system::errors::FileSystemError;
use parser::errors::ParserError;
use validator::errors::ValidatorError;

pub enum CompilerError {
    BuildError,
    IO(FileSystemError),
    Parser(ParserError),
    Validator(ValidatorError),
}
impl CompilerError {
    pub fn display(&self) {
        match self {
            CompilerError::IO(e) => {
                print!("\x1b[31m[Böyük Qardaş]:\x1b[0m {} ", e.kind);
                println!("\x1b[31m{}\x1b[0m", e.file);
            }
            CompilerError::BuildError => {
                println!("\x1b[35m[Kiçik Bacı Tərcüməçi]:\x1b[0m Build zamanı bir xəta baş verdi")
            }
            CompilerError::Parser(e) => println!("\x1b[31m[Böyük Qardaş]:\x1b[0m {}", e),
            CompilerError::Validator(e) => println!("\x1b[33m[Dəmir Əmi Validator]:\x1b[0m {}", e),
        }
    }
    pub fn code(&self) -> i32 {
        match self {
            CompilerError::IO(e) => e.code(),
            CompilerError::BuildError => 30,
            CompilerError::Parser(_) => 34,
            CompilerError::Validator(_) => 35,
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
