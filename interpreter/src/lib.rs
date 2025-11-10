use errors::{Errors, ParserError};
use tokenizer::tokens::Token;

mod parser;
pub fn interpreter(path: &str) -> Result<(), impl Errors> {
    let hello = String::from("Hello");
    let mut tokens: Vec<Token> = Vec::new();
    let parser = parser::Parser::new(&mut tokens);
    Err(ParserError::UnexpectedToken("Salam".into()))
}
