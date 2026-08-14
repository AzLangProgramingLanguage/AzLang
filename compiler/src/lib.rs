use std::{
    path::{Path, PathBuf},
    process::Command,
};

use parser::parser;
use which::which;
mod errors;
mod libc_checker;
#[cfg(test)]
mod tests;

use crate::errors::{BackendError, CompilerError};

pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let source = file_system::read_file(path)?;

    let parsed_program = parser(source)?;

    let validator = validator::Validator::default();
    let (_, program) = validator.validate(parsed_program)?;

    which("qbe").map_err(|_| CompilerError::Backend(BackendError::Qbe))?;
    which("as").map_err(|_| CompilerError::Backend(BackendError::BinUtils))?;
    which("ld").map_err(|_| CompilerError::Backend(BackendError::BinUtils))?;
    let linker = libc_checker::libc_link_checker().expect("Error");

    Ok(())
}
