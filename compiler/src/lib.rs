mod cleaner;
use crate::{cleaner::clean_ast, errors::CompilerError};
use parser::Parser;
use validator::validate::validate_expr;
mod errors;
pub fn compiler(path: &str) -> Result<(), CompilerError> {
    let sdk = file_system::read_file("sdk/data_structures.az")?;
    let mut parser = Parser::new(sdk);
    let mut parsed_program = parser.parse().map_err(|err| CompilerError::Parser(err))?;

    let mut validator = validator::ValidatorContext::new();
    for expr in parsed_program.expressions.iter_mut() {
        validate_expr(expr, &mut validator)?;
    }
    clean_ast(&mut parsed_program, &validator);

    /*     let mut validator = validator::ValidatorContext::new();
    for expr in parsed_program.expressions.iter_mut() {
        validate_expr(expr, &mut validator)?;
    }
    let mut runner = Runner::new(); */
    /*     runner.run(parsed_program);
     */
    Ok(())
}
