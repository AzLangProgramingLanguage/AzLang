use core::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub trait Errors: Debug + Display {}
#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(String),
}
impl Errors for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken(token) => write!(f, "Gözlənilməyən token '{}'", token),
        }
    }
}

struct DisplayMe<D>(D);
