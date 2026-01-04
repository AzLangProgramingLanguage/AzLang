use core::fmt;
use std::{fmt::Display, num::{ParseFloatError, ParseIntError}};

#[derive(Debug)]
pub enum LexerError {
    UnClosedString,
    VariableCannotBeNumber,
    NumberAndAlpha,
    DoubleDotNumber,
    FloatUnKnow(ParseFloatError),
    NumberUnKnow(ParseIntError),
    CannotStartZeroNumber
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnClosedString => {
                write!(f, "String düzgün bağlanmayıb")
            } 
            LexerError::VariableCannotBeNumber => {
                write!(f, "Başlangıc ədədlə adlandırıla bilməz!")
            }
            LexerError::NumberAndAlpha => {
                write!(f,"Ədəddən sonra hərf gələ bilməz!")
            }
            LexerError::DoubleDotNumber => {
                write!(f,"İki dəfə nöqtə qoya bilməzsiniz")
            }
            LexerError::NumberUnKnow(s) => {
                write!(f,"Ədəd tokenizerdə bilinməyən problem oldu problem: {s}")
            }
            LexerError::FloatUnKnow(s) => {
                write!(f,"Kəsr tokenizerdə bilinməyən problem oldu problem: {s}")
            }
            LexerError::CannotStartZeroNumber => {
                write!(f,"0 ilə ədəd başlaya bilməz")
            }
        }
    }
}
