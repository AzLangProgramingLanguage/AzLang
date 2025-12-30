mod cleaner;
use std::env;

use crate::{builder::build, cleaner::clean_ast, errors::CompilerError};
use parser::Parser;

use validator::validate::validate_expr;
mod builder;
mod errors;
pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let sdk = file_system::read_file("sdk/data_structures.az")?;
    let mut parser = Parser::new(sdk);
    let mut parsed_program = parser.parse().map_err(|err| CompilerError::Parser(err))?;

    let mut validator = validator::ValidatorContext::new();
    validator.validate(&mut parsed_program)?;
    clean_ast(&mut parsed_program, &validator);
    let mut ctx = transpiler::TranspileContext::new();
    let code = ctx.transpile(parsed_program);
    let mut temp_path = env::temp_dir();

    temp_path.push("azlang_output.zig");
    file_system::write_file(&temp_path, &code)?;
    build(temp_path.to_str().unwrap(), path)?;

    Ok(())
}
