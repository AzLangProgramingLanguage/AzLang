use tokenizer::iterator::{SourceSpan, Tokens};
mod object;

use tokenizer::tokens::Token;

type TestResult = Result<(), Box<dyn std::error::Error>>;
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
mod binary_op_test;
mod condition;
mod decl;
mod enum_test;
mod function_call;
mod function_decl;
mod while_loop;
