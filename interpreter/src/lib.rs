use file_system;
mod errors;
mod runner;
use parser::Parser;
pub use validator::validate::validate_expr;

use crate::{errors::InterPreterError, runner::Runner};
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
    /*     let sdk_tokens = {
        let mut lexer = tokenizer::Lexer::new(sdk.as_str());
        lexer.tokenize()
    }; */
    /*     let user_input = file_system::read_file(path)?;
     */
    /*   let mut parser = Parser::new(sdk);
    let mut parsed_program = parser.parse()?;
    */

    Ok(())
}
