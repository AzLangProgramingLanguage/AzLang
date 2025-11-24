use file_system;
mod errors;
mod runner;
use crate::{errors::InterPreterError, runner::Runner};
use parser::Parser;
pub use validator::validate::validate_expr;

pub fn interpreter(path: &str) -> Result<(), InterPreterError> {
    let sdk = file_system::read_file("sdk/data_structures.az")?;
    let mut parser = Parser::new(sdk);
    let mut parsed_program = parser
        .parse()
        .map_err(|err| InterPreterError::Parser(err))?;

    let mut validator = validator::ValidatorContext::new();
    for expr in parsed_program.expressions.iter_mut() {
        validate_expr(expr, &mut validator)?;
    }
    let mut runner = Runner::new();
    runner.run(parsed_program);
    Ok(())
}
