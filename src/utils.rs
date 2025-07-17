use std::fs;
use std::io::{self, Write};

pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
