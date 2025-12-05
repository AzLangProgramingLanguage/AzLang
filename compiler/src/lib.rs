mod cleaner;
use std::env;

use crate::{builder::build, cleaner::clean_ast, errors::CompilerError};
use parser::Parser;

use validator::validate::validate_expr;
mod builder;
mod errors;
mod transpiler;
pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let sdk = file_system::read_file("sdk/data_structures.az")?;
    let mut parser = Parser::new(sdk);
    let mut parsed_program = parser.parse().map_err(|err| CompilerError::Parser(err))?;

    let mut validator = validator::ValidatorContext::new();
    for expr in parsed_program.expressions.iter_mut() {
        validate_expr(expr, &mut validator)?;
    }
    clean_ast(&mut parsed_program, &validator);
    let mut ctx = transpiler::TranspileContext::new();
    let code = ctx.transpile(&parsed_program);
    println!("Code:  {}", code);
    //let mut temp_path = env::temp_dir();

    //temp_path.push("azlang_output.zig");
    /*   file_system::write_file(&temp_path, &code)?;
    build(temp_path.to_str().unwrap(), path); */

    Ok(())
}
