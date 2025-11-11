use errors::parser_errors::ParserError;
use errors::{Errors, InterpreterError};
use file_system;
use tokenizer::tokens::Token;
mod parser;
pub fn interpreter(path: &str) -> Result<String, InterpreterError> {
    //println!("{path}");
    let mut tokens: Vec<Token> = Vec::new();
    let user_input = file_system::read_file(path)?;
    let parser = parser::Parser::new(&mut tokens);
    let ast = parser.parse()?;
    Ok("Works".to_string())
}
