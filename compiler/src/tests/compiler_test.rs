use std::{path::PathBuf, process::Command};

use transpiler::transpile_program;
use which::which;

use crate::{
    errors::{BackendError, CompilerError},
    libc_checker, parser,
};
/*
*
*  I change my mind,  github actions will not test this code.
*
* */
#[test]
fn dependencies_test() -> Result<(), CompilerError> {
    which("qbe").map_err(|_| CompilerError::Backend(BackendError::Qbe))?;
    which("as").map_err(|_| CompilerError::Backend(BackendError::BinUtils))?;
    which("ld").map_err(|_| CompilerError::Backend(BackendError::BinUtils))?;
    Ok(())
}
#[test]
fn compiler_output_file() -> Result<(), CompilerError> {
    let parsed_program = parser(String::from("exit 2"))?;

    let validator = validator::Validator::default();
    let (_, program) = validator.validate(parsed_program)?;
    // let transpiled_code = transpile_program(program);

    let linker = libc_checker::libc_link_checker().expect("Error");

    let main_file: PathBuf = PathBuf::from("main.ssa");

    Command::new("qbe")
        .args(["-o", "main.s", "main.ssa"])
        .status()
        .expect("Error");
    Command::new("as")
        .args(["main.s", "-o", "main.o"])
        .status()
        .expect("Assembler  can't compile to object ");
    Command::new("as")
        .args(["starter.s", "-o", "starter.o"])
        .status()
        .expect("Assembler  can't compile to object ");
    Command::new("as")
        .args(["print.s", "-o", "print.o"])
        .status()
        .expect("Assembler  can't compile to object ");

    Command::new(linker)
        .args(["starter.o", "main.o", "print.o", "-o", "app"])
        .status()
        .expect("Linker Error");
    Ok(())
}
