use core::fmt;
use std::fmt::Display;

use tokenizer::tokens::Token;

#[derive(Debug)]
pub enum ValidatorError {
    UnknownType(String),
}

impl Display for ValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidatorError::UnknownType(token) => write!(f, "Gözlənilməyən token '{token}'"),
        }
    }
}
