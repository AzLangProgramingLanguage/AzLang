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
    InCorrectSpaceSize,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnClosedString(span, str) => {
                write!(f, "{}, String düzgün bağlanmayıb \"{}\"", span, str)
            }
            LexerError::VariableCannotBeNumber => {
                write!(f, "Başlangıc ədədlə adlandırıla bilməz!")
            }
            LexerError::NumberAndAlpha => {
                write!(f, "Ədəddən sonra hərf gələ bilməz!")
            }
            LexerError::DoubleDotNumber => {
                write!(f, "İki dəfə nöqtə qoya bilməzsiniz")
            }
            LexerError::NumberUnKnow(s) => {
                write!(f, "Ədəd tokenizerdə bilinməyən problem oldu problem: {s}")
            }
            LexerError::FloatUnKnow(s) => {
                write!(f, "Kəsr tokenizerdə bilinməyən problem oldu problem: {s}")
            }
            LexerError::CannotStartZeroNumber(span, str) => {
                write!(
                    f,
                    "{} Yanlış ədəd formatı \"{}\" . Onluq ədədlərin başlangıcı sıfırla başlaya bilməz.",
                    span,
                    str
                )
            }
            
            LexerError::InCorrectSpaceSize => {
                write!(f, "Uyğunsuz boşluq var.")
            }
        }
    }
}
