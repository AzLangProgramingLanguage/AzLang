use std::fs;
use std::io;

use errors::FileSystem;
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

