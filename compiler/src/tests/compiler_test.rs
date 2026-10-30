use std::{path::PathBuf, process::Command};

use file_system::write_file;
use transpiler::Transpiler;
use validator::Validator;
use which::which;

use crate::{
    errors::{BackendError, CompilerError},
    parser,
    pipline::executer,
};

#[test]
fn dependencies_test() -> Result<(), CompilerError> {
    which("fasm").map_err(|_| CompilerError::Backend(BackendError::Fasm))?;
    which("as").map_err(|_| CompilerError::Backend(BackendError::BinUtils))?;
    which("ld").map_err(|_| CompilerError::Backend(BackendError::BinUtils))?;
    Ok(())
}
#[test]
fn compiler_exit_output_file() -> Result<(), CompilerError> {
    let parsed_program = parser(String::from("const int a = 5"))?;
    let mut validator = Validator::default();
    let mut validated = validator.validate(parsed_program.ast)?;
    let mut transpiler = Transpiler::default();
    validated.variables = validator.constvars.clone();
    let transpiled_code = transpiler.transpile_start(validated);

    let main_file = PathBuf::from("test.asm");
    write_file(&main_file, transpiled_code)?;
    executer("test", vec![]);
    assert_eq!(
        Command::new("./test")
            .status()
            .expect("Output error")
            .code()
            .expect("There is no error code"),
        0
    );
    Ok(())
}
//cargo test -- --test-threads=1
