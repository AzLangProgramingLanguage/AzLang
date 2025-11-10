pub mod ast;
use ast::Program;
use errors::ParserError;
use tokenizer::tokens::Token;
pub struct Parser<'a> {
    tokens: &'a mut Vec<Token>,
}
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut Vec<Token>) -> Self {
        Self { tokens }
    }
    pub fn parse(&self) -> Result<Program, ParserError> {
        let tokens = &mut self.tokens.iter().peekable();
        Err(ParserError::UnexpectedToken("Salam".into()))
    }
}
