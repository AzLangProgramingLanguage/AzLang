use file_system;
mod errors;
mod runner;
use crate::{errors::InterPreterError, runner::Runner};
use parser::Parser;
pub use validator::validate::validate_expr;

pub fn interpreter(_path: &str, output: &mut String) -> Result<(), InterPreterError> {
    let sdk = file_system::read_file(_path)?;
    let mut lexer = tokenizer::Lexer::new(&sdk);
    let tokens = lexer
        .tokenize()
        .map_err(|err| InterPreterError::Lexer(err))?;
    let mut parser = Parser::new(tokens);
    let mut parsed_program = parser
        .parse()
        .map_err(|err| InterPreterError::Parser(err))?;

    let mut validator = validator::ValidatorContext::new();
    validator.validate(&mut parsed_program)?;
    let mut runner = Runner::new(output);
    runner.run(parsed_program);
    Ok(())
}
