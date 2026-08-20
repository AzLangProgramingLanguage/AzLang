use std::{path::PathBuf, process::Command};

use file_system::write_file;
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
    let parsed_program = parser(String::from(
        "
@link(\"../../exit.o\") 
func exit(const int val): void
exit(50)",
    ))?;

    let validator = validator::Validator::default();
    let (context, program) = validator.validate(parsed_program)?;
    let transpiled_code = transpile_program(program);

    let linker = libc_checker::libc_link_checker().expect("Error");

    let main_file: PathBuf = PathBuf::from("main.ssa");
    write_file(&main_file, transpiled_code)?;

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
        .args(["exit.s", "-o", "exit.o"])
        .status()
        .expect("Assembler  can't compile to object ");

    Command::new("as")
        .args(["print.s", "-o", "print.o"])
        .status()
        .expect("Assembler  can't compile to object ");

    Command::new(linker)
        .args(["starter.o", "main.o", "exit.o", "print.o", "-o", "app"])
        .status()
        .expect("Linker Error");
    Ok(())
}
