use std::collections::VecDeque;

use crate::tokenizer::iterator::SpannedToken;
use crate::tokenizer::tokens::Token;
use file_system;
use logging::error;
use tokenizer;
use tokenizer::iterator::SourceSpan;
use tokenizer::iterator::Tokens;
use parser;
fn main() {
    let sdk = file_system::read_file("test.az").expect("Error var");
    let mut tokeniz = tokenizer::new_lexer::NewLexer::new(&sdk);
    let mut real_tokens = match tokeniz.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    let parser = match parser::Parser::new_parse(&mut real_tokens) {
        Ok(parser) => parser,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
}
