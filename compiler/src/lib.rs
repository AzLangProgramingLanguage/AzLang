mod cleaner;
use std::env;

use crate::{builder::build, cleaner::clean_ast, errors::CompilerError, methods::methods_string};
use parser::Parser;
mod methods;

mod builder;
mod errors;
pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let sdk = file_system::read_file(path)?;
    let tokens = tokenizer::Lexer::new(&sdk)
        .tokenize()
        .map_err(|err| CompilerError::Lexer(err))?;
    let mut parser = Parser::new(tokens);
    let mut parsed_program = parser.parse().map_err(|err| CompilerError::Parser(err))?;

    let mut validator = validator::ValidatorContext::new();
    validator.validate(&mut parsed_program)?;
    clean_ast(&mut parsed_program, &validator);
    let mut ctx = transpiler::TranspileContext::new();
    let code = ctx.transpile(parsed_program);
    let mut temp_path = env::temp_dir();

    temp_path.push("azlang_output.zig");
    let mut temp_path_2 = env::temp_dir();
    temp_path_2.push("to_string.zig");
    file_system::write_file(&temp_path_2, &methods_string())?;
    file_system::write_file(&temp_path, &code)?;
    build(temp_path.to_str().unwrap(), path)?;

    Ok(())
}
