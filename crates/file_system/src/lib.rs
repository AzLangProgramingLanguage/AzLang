use std::fs;
use std::io;
use std::path::PathBuf;
pub mod errors;
use crate::errors::FileSystem;
pub fn read_file(path: &str) -> Result<String, FileSystem> {
    if !path.ends_with(".az") {
        return Err(FileSystem::UnsupportedFile(path.into()));
    }
    let read_to_string = fs::read_to_string(path);
    return match read_to_string {
        Ok(s) => Ok(s),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => Err(FileSystem::FileNotFound(path.into())),
            _ => Err(FileSystem::UnsupportedFile(path.into())),
        },
    };
}
pub fn write_file(path: PathBuf, content: &String) -> Result<(), FileSystem> {
    let write = fs::write(&path, content);
    return match write {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                Err(FileSystem::FileNotFound(path.to_string_lossy().to_string()))
            }
            _ => Err(FileSystem::UnsupportedFile(
                path.to_string_lossy().to_string(),
            )),
        },
    };
}
