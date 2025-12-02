mod cleaner;
use std::{env, process::Command};

use crate::{cleaner::clean_ast, errors::CompilerError};
use parser::Parser;
mod transpiler;
use validator::validator_typed::validate_typed::validate_expr_typed;
mod errors;
pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let sdk = file_system::read_file("sdk/data_structures.az")?;
    let mut parser = Parser::new(sdk);
    let mut parsed_program = parser
        .parse_for_transpile()
        .map_err(|err| CompilerError::Parser(err))?;

    let mut validator = validator::validator_typed::ValidatorTypedContext::new();
    for expr in parsed_program.expressions.iter_mut() {
        validate_expr_typed(expr, &mut validator)?;
    }
    clean_ast(&mut parsed_program, &validator);
    let mut ctx = transpiler::TranspileContext::new();
    let code = ctx.transpile(&parsed_program);
    println!("Code:  {}", code);
    let mut temp_path = env::temp_dir();

    temp_path.push("azlang_output.zig");
    file_system::write_file(temp_path, &code)?;
    let mut compile = Command::new("zig");

    Ok(())
}
