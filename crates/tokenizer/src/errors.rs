use core::fmt;
use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};

use crate::iterator::SourceSpan;

#[derive(Debug)]
pub enum LexerError {
    UnClosedString(SourceSpan, String),
    VariableCannotBeNumber,
    NumberAndAlpha,
    DoubleDotNumber,
    FloatUnKnow(ParseFloatError),
    NumberUnKnow(ParseIntError),
    CannotStartZeroNumber(SourceSpan, String),
    InCorrectSpaceSize(SourceSpan),
    UnexpectedToken(SourceSpan, char),
    UnknownOperator(SourceSpan, String),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedToken(span, token) => {
                write!(f, "{}, Unexpected token \"{}\"", span, token)
            }
            LexerError::UnClosedString(span, str) => {
                write!(f, "{}, String is not properly closed \"{}\"", span, str)
            }
            LexerError::VariableCannotBeNumber => {
                write!(f, "A name cannot start with a number!")
            }
            LexerError::NumberAndAlpha => {
                write!(f, "A letter cannot follow a number!")
            }
            LexerError::DoubleDotNumber => {
                write!(f, "You cannot use two consecutive dots")
            }
            LexerError::NumberUnKnow(s) => {
                write!(f, "Unknown problem while tokenizing number, problem: {s}")
            }
            LexerError::FloatUnKnow(s) => {
                write!(f, "Unknown problem while tokenizing float, problem: {s}")
            }
            LexerError::CannotStartZeroNumber(span, str) => {
                write!(
                    f,
                    "{} Invalid number format \"{}\" . Decimal numbers cannot start with zero.",
                    span, str
                )
            }
            LexerError::UnknownOperator(span, str) => {
                write!(f, "{} Unexpected operator \"{}\"", span, str)
            }

            LexerError::InCorrectSpaceSize(span) => {
                write!(f, "{} Invalid space size.", span)
            }
        }
    }
}
