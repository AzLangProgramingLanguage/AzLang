use std::{path::PathBuf, process::Command};

use file_system::write_file;
use transpiler::Transpiler;
use which::which;

use crate::{
    errors::{BackendError, CompilerError},
    parser,
};

#[test]
fn dependencies_test() -> Result<(), CompilerError> {
    which("qbe").map_err(|_| CompilerError::Backend(BackendError::Qbe))?;
    which("as").map_err(|_| CompilerError::Backend(BackendError::BinUtils))?;
    which("ld").map_err(|_| CompilerError::Backend(BackendError::BinUtils))?;
    Ok(())
}
#[test]
fn compiler_exit_output_file() -> Result<(), CompilerError> {
    let source = file_system::read_file("asda")?;
    let parsed_program = parser(source)?;

    let validator = validator::Validator::default();
    let (context, program) = validator.validate(parsed_program)?;
    let transpiled_code = Transpiler::default().transpile(program);

    let main_file: PathBuf = PathBuf::from("main.ssa");
    write_file(&main_file, transpiled_code)?;
    let output = executer(context.link_files);
    assert_eq!(output.status.code(), Some(50));
    Ok(())
}
//cargo test -- --test-threads=1
