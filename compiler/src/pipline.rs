use std::{path::Path, process::Command};

use crate::errors::{BackendError, CompilerError};

pub fn assemble(asm: &Path, binary: &Path) -> Result<(), CompilerError> {
    let output = Command::new("fasm")
        .arg(asm)
        .arg(binary)
        .output()
        .map_err(|_| CompilerError::Backend(BackendError::Fasm))?;
    if !output.status.success() {
        eprintln!(
            "\x1b[31m[Fasm Error]:\x1b[0m\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(output.status.code().unwrap_or(1));
    }
    Ok(())
}
