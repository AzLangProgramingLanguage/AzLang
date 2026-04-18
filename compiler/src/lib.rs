mod cleaner;
use file_system::errors::FileSystemError;
use parser::parser;

use crate::{builder::build, cleaner::clean_ast, errors::CompilerError};
use std::{
    env, fs,
    path::{Path, PathBuf},
};
mod builder;
mod errors;
mod tests;

pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let sdk = file_system::read_file(path)?;

    let mut parsed_program = parser(sdk)?;

    let mut validator = validator::Validator::new();
    validator.validate(&mut parsed_program)?;

    clean_ast(&mut parsed_program, &validator);

    // let mut ctx = transpiler::TranspileContext::new();
    // let code = ctx.transpile(parsed_program);
    //
    // let output_zig = bin_create_dir()?;
    //
    // file_system::write_file(&output_zig, &code).unwrap_or_else(|err| {
    //     println!("\x1b[31m[Böyük Qardaş]:\x1b[0m {}", err.kind);
    //     std::process::exit(err.code());
    // });
    //
    // build(output_zig.to_str().unwrap(), path)?;
    Ok(())
}
fn bin_create_dir() -> Result<PathBuf, FileSystemError> {
    let bin_path = Path::new("./bin");
    if !bin_path.exists() {
        fs::create_dir(bin_path)?;
    }
    let deps_src = Path::new("./dependencies");
    let deps_dest = bin_path.join("dependencies");
    if deps_src.exists() {
        copy_dir_all(deps_src, &deps_dest)?;
    }

    Ok(bin_path.join("azlang_output.zig"))
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), FileSystemError> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
