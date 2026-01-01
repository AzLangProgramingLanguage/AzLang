use file_system;
mod errors;
mod runner;
use crate::{errors::InterPreterError, runner::Runner};
use parser::Parser;
pub use validator::validate::validate_expr;

pub fn interpreter(_path: &str) -> Result<(), InterPreterError> {
    let sdk = file_system::read_file(_path)?;
    let mut parser = Parser::new(sdk);
    let mut parsed_program = parser
        .parse()
        .map_err(|err| InterPreterError::Parser(err))?;

    let mut validator = validator::ValidatorContext::new();
    validator.validate(&mut parsed_program)?;
    let mut runner = Runner::new();
    runner.run(parsed_program);
    Ok(())
}
