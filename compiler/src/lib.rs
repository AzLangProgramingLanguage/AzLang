/* use errors::{CompilerError, Errors, ParserError};
 */
use parser::Parser;
use tokenizer::tokens::Token;
//mod parser;
pub fn compiler(path: &str) -> Result<(), String> {
    /*     let mut tokens: Vec<Token> = Vec::new();
    let sdk = file_system::read_file("sdk/data_structures.az")?;
    let sdk_tokens = {
        let mut lexer = tokenizer::Lexer::new(sdk.as_str());
        lexer.tokenize()
    };
    tokens.extend(sdk_tokens); */
    /*     let user_input = file_system::read_file(path)?;
     */
    /*     let parser = Parser::new(&mut tokens);
    let mut parsed_program = parser.parse()?; */

    /*     let mut validator = validator::ValidatorContext::new();
    for expr in parsed_program.expressions.iter_mut() {
        validate_expr(expr, &mut validator)?;
    }
    let mut runner = Runner::new(); */
    /*     runner.run(parsed_program);
     */
    Ok(())
}
