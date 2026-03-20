use core::fmt;
use std::fmt::Display;


pub enum FileSystemKind {
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
            FileSystemKind::UnsupportedFile => 31,
            FileSystemKind::FileNotFound => 32,
        }
    }
}

impl Display for FileSystemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileSystemKind::UnsupportedFile => write!(f, "Dəstəklənməyən Fayl, yalnız .az faylları dəstəklənir"),
            FileSystemKind::FileNotFound => write!(f, "Fayl tapılmadı"),
        }
    }
}
