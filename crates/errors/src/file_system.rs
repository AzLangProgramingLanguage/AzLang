use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum FileSystem {
    UnsupportedFile(String),
    FileNotFound(String),
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileSystem::UnsupportedFile(file) => write!(f, "Dəstəklənməyən Fayl: {file}"),
            FileSystem::FileNotFound(file) => write!(f, "Fayl tapılmadı: {file}"),
        }
    }
}
