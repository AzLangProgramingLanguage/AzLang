use std::path::PathBuf;

use file_system::write_file;
use parser::parser;
mod errors;
mod pipline;

use crate::{
    errors::CompilerError,
    pipline::{CompilerPipline, executer},
};

pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let source = file_system::read_file(path)?;
    let (context, transpiled_code) = CompilerPipline(parser(source)?).validate()?.transpile();

    let main_file = PathBuf::from("main.ssa");
    write_file(&main_file, transpiled_code)?;

    executer(context.link_files);

    Ok(())
}

#[cfg(test)]
mod tests;
