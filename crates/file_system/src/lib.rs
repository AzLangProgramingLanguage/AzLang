use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::errors::FileSystemError;
use crate::errors::FileSystemKind;
pub mod errors;

pub fn read_file(path: &str) -> Result<String, FileSystemError> {
    if !path.ends_with(".az") {
        return Err(FileSystemError {
            kind: FileSystemKind::UnsupportedFile,
            file: path.to_string(),
        });
    }
    let read_to_string = fs::read_to_string(path);
    match read_to_string {
        Ok(s) => Ok(s),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => Err(FileSystemError {
                kind: FileSystemKind::FileNotFound,
                file: path.to_string(),
            }),
            _ => Err(FileSystemError {
                kind: FileSystemKind::UnsupportedFile,
                file: path.to_string(),
            }),
        },
    }
}
pub fn copy_file(path: &str, move_path: &str) -> Result<(), FileSystemError> {
    match fs::copy(path, move_path) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => Err(FileSystemError {
                kind: FileSystemKind::FileNotFound,
                file: path.to_string(),
            }),
            _ => Err(FileSystemError {
                kind: FileSystemKind::UnsupportedFile, //TODO: Menasız Error Mesajı
                file: path.to_string(),
            }),
        },
    }
}

pub fn write_file(path: &PathBuf, content: String) -> Result<(), FileSystemError> {
    let write = fs::write(path, content);
    match write {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => Err(FileSystemError {
                kind: FileSystemKind::FileNotFound,
                file: path.to_string_lossy().to_string(),
            }),
            _ => Err(FileSystemError {
                kind: FileSystemKind::UnsupportedFile,
                file: path.to_string_lossy().to_string(),
            }),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_file_test() {
        let path = "test.az";
        let read_file = read_file(path);
        assert!(read_file.is_err());
    }
}
