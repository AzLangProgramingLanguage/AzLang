use file_system::errors::FileSystemError;
use parser::errors::ParserError;
use validator::errors::ValidatorError;

pub enum InterPreterError {
    IO(FileSystemError),
    Parser(ParserError),
    Validator(ValidatorError),
}

impl InterPreterError {
    pub fn display(&self) {
        match self {
            InterPreterError::IO(e) => {
                print!("\x1b[31m[Böyük Qardaş]:\x1b[0m {} ", e.kind);
                println!("\x1b[31m{}\x1b[0m", e.file);
            }
            InterPreterError::Parser(e) => println!("\x1b[31m[Böyük Qardaş]:\x1b[0m {}", e),
            InterPreterError::Validator(e) => {
                println!("\x1b[31m[Dəmir Əmi Validator]:\x1b[0m {}", e)
            }
        }
    }
    pub fn code(&self) -> i32 {
        match self {
            InterPreterError::IO(e) => e.code(),
            InterPreterError::Parser(_) => 34,
            InterPreterError::Validator(_) => 35,
        }
    }
}

impl From<FileSystemError> for InterPreterError {
    fn from(e: FileSystemError) -> Self {
        InterPreterError::IO(e)
    }
}

impl From<ValidatorError> for InterPreterError {
    fn from(e: ValidatorError) -> Self {
        InterPreterError::Validator(e)
    }
}
impl From<ParserError> for InterPreterError {
    fn from(e: ParserError) -> Self {
        InterPreterError::Parser(e)
    }
}
