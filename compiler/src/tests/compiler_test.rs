use std::{path::PathBuf, process::Command};

use file_system::write_file;
use transpiler::Transpiler;
use which::which;

use crate::{
    errors::{BackendError, CompilerError},
    parser,
};

fn executer() -> std::process::Output {
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
        .args(["write.s", "-o", "write.o"])
        .status()
        .expect("Assembler  can't compile to object ");

    Command::new("ld")
        .args(["starter.o", "main.o", "exit.o", "write.o", "-o", "app"])
        .status()
        .expect("Linker Error");
    Command::new("./app").output().expect("Çalışdırılmadı")
}
#[test]
fn dependencies_test() -> Result<(), CompilerError> {
    which("qbe").map_err(|_| CompilerError::Backend(BackendError::Qbe))?;
    which("as").map_err(|_| CompilerError::Backend(BackendError::BinUtils))?;
    which("ld").map_err(|_| CompilerError::Backend(BackendError::BinUtils))?;
    Ok(())
}
#[test]
fn compiler_exit_output_file() -> Result<(), CompilerError> {
    let parsed_program = parser(String::from(
        "
@link(\"../../exit.o\")
func exit(const int val): void
exit(50)",
    ))?;

    let validator = validator::Validator::default();
    let (context, program) = validator.validate(parsed_program)?;
    let transpiled_code = Transpiler::default().transpile(program);

    let main_file: PathBuf = PathBuf::from("main.ssa");
    write_file(&main_file, transpiled_code)?;

    let output = executer();
    assert_eq!(output.status.code(), Some(50));
    Ok(())
}
#[test]
fn compiler_exit_binary_output_file() -> Result<(), CompilerError> {
    let parsed_program = parser(String::from(
        "
@link(\"../../exit.o\")
func exit(const int val): void
exit(50+20)",
    ))?;

    let validator = validator::Validator::default();
    let (context, program) = validator.validate(parsed_program)?;
    let transpiled_code = Transpiler::default().transpile(program);

    let main_file: PathBuf = PathBuf::from("main.ssa");
    write_file(&main_file, transpiled_code)?;

    let output = executer();
    assert_eq!(output.status.code(), Some(70));
    Ok(())
}
#[test]
fn compiler_print_output_file() -> Result<(), CompilerError> {
    let parsed_program = parser(String::from(
        "
@link(\"../../write.o\")
func write(const str val,const int size): void
write(\"2222\",4)",
    ))?;

    let validator = validator::Validator::default();
    let (context, program) = validator.validate(parsed_program)?;
    let transpiled_code = Transpiler::default().transpile(program);

    let main_file: PathBuf = PathBuf::from("main.ssa");
    write_file(&main_file, transpiled_code)?;

    let output = executer();
    let string: Vec<u8> = b"2222".to_vec();

    assert_eq!(output.stdout, string);
    assert_eq!(output.status.code(), Some(0));

    Ok(())
}
//cargo test -- --test-threads=1
