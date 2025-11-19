use errors::InterPreterError;
use file_system;
mod runner;
mod validator;
use tokenizer::tokens::Token;
pub use validator::validate::validate_expr;

use crate::runner::Runner;
mod parser;
pub fn interpreter(path: &str) -> Result<String, InterPreterError> {
    let mut tokens: Vec<Token> = Vec::new();
    let sdk = file_system::read_file("sdk/data_structures.az")?;
    let sdk_tokens = {
        let mut lexer = tokenizer::Lexer::new(sdk.as_str());
        lexer.tokenize()
    };
    tokens.extend(sdk_tokens);
    /*     let user_input = file_system::read_file(path)?;
     */
    let parser = parser::Parser::new(&mut tokens);
    let mut parsed_program = parser.parse()?;
    let mut validator = validator::ValidatorContext::new();
    for expr in parsed_program.expressions.iter_mut() {
        validate_expr(expr, &mut validator)?;
    }
    let mut runner = Runner::new();
    runner.run(parsed_program);

    Ok("Works".to_string())
}
