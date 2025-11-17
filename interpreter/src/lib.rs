use errors::InterPreterError;
use file_system;
use tokenizer::tokens::Token;
mod parser;
mod validator;
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
    let ast = parser.parse()?;

    Ok("Works".to_string())
}
