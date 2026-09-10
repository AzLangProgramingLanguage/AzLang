use std::{
    path::{Path, PathBuf},
    process::Command,
};

use file_system::write_file;
use parser::parser;
use transpiler::Transpiler;
use which::which;
mod errors;
mod libc_checker;
mod pipline;
#[cfg(test)]
mod tests;

use crate::errors::{BackendError, CompilerError};

fn executer(static_libs: Vec<String>) -> std::process::Output {
    Command::new("qbe")
        .args(["-o", "main.s", "main.ssa"])
        .status()
        .expect("Error");
    Command::new("as")
        .args(["main.s", "-o", "main.o"])
        .status()
        .expect("Error");
    Command::new("ld.lld")
        .args(static_libs)
        .args(["main.o", "-o", "app"])
        .status()
        .expect("Linker Error");
    Command::new("./app").output().expect("Çalışdırılmadı")
}
pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let source = file_system::read_file(path)?;
    let parsed_program = parser(source)?;

    let validator = validator::Validator::default();
    let (context, program) = validator.validate(parsed_program)?;
    println!("Validate olundu");
    let transpiled_code = Transpiler::default().transpile(program);
    println!("Transpile olundu");

    let main_file: PathBuf = PathBuf::from("main.ssa");
    write_file(&main_file, transpiled_code)?;

    let output = executer(context.link_files);

    Ok(())
}
