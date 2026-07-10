// mod cleaner;
use file_system::errors::FileSystemError;
use parser::parser;
use validator::ast::ExternalFunctionDef;
mod helpers;
use crate::{
    builder::{build, get_zig_path},
    errors::CompilerError,
    helpers::bin_create_dir,
};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
mod builder;
mod errors;
mod tests;

pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let sdk = file_system::read_file(path)?;

    let parsed_program = parser(sdk)?;

    let validator = validator::Validator::default();
    let (context, program) = validator.validate(parsed_program)?;

    let output_zig = bin_create_dir()?;

    let mut ctx = transpiler::TranspileContext::default();

    let code = ctx.transpile(program);
    let buildfile = ctx.transpile_build();
    file_system::write_file(&output_zig.join("./src/main.zig"), code)?;
    file_system::write_file(&output_zig.join("./build.zig"), buildfile)?;

    build(output_zig)?;
    Ok(())
}
