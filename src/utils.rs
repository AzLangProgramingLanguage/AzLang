use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};

use color_eyre::eyre::{Result, eyre};

pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn read_file_with_imports(path: &str) -> Result<String> {
    let mut visited = HashSet::new();
    let mut result = String::new();
    read_recursive(path, &mut result, &mut visited)?;
    Ok(result)
}

fn read_recursive(path: &str, result: &mut String, visited: &mut HashSet<String>) -> Result<()> {
    if visited.contains(path) {
        return Ok(());
    }
    visited.insert(path.to_string());

    let content =
        fs::read_to_string(path).map_err(|e| eyre!("{} faylı oxuna bilmədi: {}", path, e))?;

    for line in content.lines() {
        if let Some(import_path) = extract_import_path(line) {
            let full_path = format!("{}.az", import_path);
            read_recursive(&full_path, result, visited)?;
        }
    }

    result.push_str(&content);
    result.push('\n');
    Ok(())
}

fn extract_import_path(line: &str) -> Option<String> {
    if line.trim_start().starts_with("ƏlavəEt(") {
        let between_quotes = line.split('"').nth(1)?;
        Some(between_quotes.replace(".az", ""))
    } else {
        None
    }
}
