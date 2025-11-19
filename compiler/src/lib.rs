use errors::{Errors, ParserError};
use tokenizer::tokens::Token;
mod parser;
pub fn compiler(path: &str) -> Result<(), impl Errors> {
    println!("Hello");
    Err(ParserError::UnexpectedToken(Token::Eof))
}
