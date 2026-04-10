use std::fs;
use std::io;
use std::path::PathBuf;

use crate::errors::FileSystemError;
use crate::errors::FileSystemKind;
pub mod errors;

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

pub fn read_file(path: &str) -> Result<String, FileSystemError> {
    if !path.ends_with(".az") {
        return Err(FileSystemError {
            kind: FileSystemKind::UnsupportedFile,
            file: path.to_string(),
        });
    }
    let read_to_string = fs::read_to_string(path);
    return match read_to_string {
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
    };
}
pub fn write_file(path: &PathBuf, content: &String) -> Result<(), FileSystemError> {
    let write = fs::write(&path, content);
    return match write {
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
    };
}
