use std::path::{Path, PathBuf};

use parser::parser;
use which::which;
mod errors;
#[cfg(test)]
mod tests;

use crate::errors::{BackendError, CompilerError};

pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let source = file_system::read_file(path)?;

    let parsed_program = parser(source)?;

    let validator = validator::Validator::default();
    let (_, program) = validator.validate(parsed_program)?;

    which("qbe").map_err(|_| CompilerError::Backend(BackendError::Qbe))?;
    which("as").map_err(|_| CompilerError::Backend(BackendError::Asembly))?;

    //     let main_file: PathBuf = PathBuf::from("main.qbe");
    //     file_system::write_file(
    //         &main_file,
    //         "export function w $main() {                # Main function
    //     @start
    // 	  %r =w call $add(w 1, w 1)          # Call add(1, 1)
    // 	  call $printf(l $fmt, ..., w %r)    # Show the result
    // 	  ret 0
    //     }
    // "
    //         .to_string(),
    //     )?;
    //
    Ok(())
}

