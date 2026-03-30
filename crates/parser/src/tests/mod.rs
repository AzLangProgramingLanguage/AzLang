use tokenizer::iterator::{SourceSpan, Tokens};
use tokenizer::tokens::Token;

pub fn create_tokens(tokens_vec: Vec<Token>) -> Tokens {
    let mut tokens = Tokens::default();
    for token in tokens_vec {
        tokens.push(
            token,
            SourceSpan {
                start: 0,
                end: 0,
                line: 0,
            },
        );
    }
    tokens
}
mod assignment;
mod builtin_test;
mod condition;
mod decl;
mod enum_test;
