use core::fmt;
use std::{fmt::Display, io};
pub enum FileSystemKind {
    IOError(io::Error),
    UnsupportedFile,
    FileNotFound,
}
pub struct FileSystemError {
    pub kind: FileSystemKind,
    pub file: String,
}
impl FileSystemError {
      pub fn code(&self) -> i32 {
        match self.kind {
            FileSystemKind::IOError(_) => 30,
            FileSystemKind::UnsupportedFile => 31,
            FileSystemKind::FileNotFound => 32,
        }
    }
}

impl From<io::Error> for FileSystemError {
    fn from(e: io::Error) -> Self {
        FileSystemError {
            kind: FileSystemKind::IOError(e),
            file: String::new(),
        }
    }
}

    

impl Display for FileSystemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileSystemKind::IOError(e) => write!(f, "IO Error: {}", e),
            FileSystemKind::UnsupportedFile => write!(f, "Dəstəklənməyən Fayl, yalnız .az faylları dəstəklənir"),
            FileSystemKind::FileNotFound => write!(f, "Fayl tapılmadı"),
            
        }
    }
}
