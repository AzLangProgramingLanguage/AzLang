use which::which;

use crate::errors::{BackendError, CompilerError};

#[test]
fn dependencies_test() -> Result<(), CompilerError> {
    which("fasm").map_err(|_| CompilerError::Backend(BackendError::Fasm))?;
    Ok(())
}
