// mod cleaner;
use parser::parser;
mod helpers;
use crate::{builder::build, errors::CompilerError, helpers::bin_create_dir};
mod builder;
mod errors;
#[cfg(test)]
mod tests;

pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let sdk = file_system::read_file(path)?;

    let parsed_program = parser(sdk)?;

    let validator = validator::Validator::default();
    let (_, program) = validator.validate(parsed_program)?;

    let output_zig = bin_create_dir()?;

    let mut ctx = transpiler::TranspileContext::default();

    let code = ctx.transpile(program);
    let buildfile = ctx.transpile_build();
    file_system::write_file(&output_zig.join("./src/main.zig"), code)?;
    file_system::write_file(&output_zig.join("./build.zig"), buildfile)?;

    build(output_zig)?;
    Ok(())
}
