use std::collections::VecDeque;

use file_system;
use logging::error;
use tokenizer;
use tokenizer::iterator::Tokens;
use tokenizer::iterator::SourceSpan;
use crate::tokenizer::iterator::SpannedToken;

fn main() {
    let sdk = file_system::read_file("test.az").expect("Error var");
    let mut tokeniz = tokenizer::new_lexer::NewLexer::new(&sdk);
    let real_tokens = tokeniz.tokenize();
    match real_tokens {
        Ok(tokens) => println!("{:?}", tokens),
        Err(e) => error(&e.to_string()),
    }
}
