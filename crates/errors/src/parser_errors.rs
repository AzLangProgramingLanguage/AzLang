use core::fmt;
use std::fmt::Display;

use tokenizer::tokens::Token;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(Token),
    UnexpectedEOF,
    NotUserDirectValue,
    MethodNameNotFound(Token),
    UnsupportedBuiltInFunction(Token),
    ExpectedToken(Token, Token),
    LoopVarNameNotFound(Token),
    StructNameNotFound(Token),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken(token) => write!(f, "Gözlənilməyən token '{token}'"),
            ParserError::UnexpectedEOF => write!(f, "Gözlənilməyən EOF"),
            ParserError::NotUserDirectValue => write!(
                f,
                "Bir başa mətn, rəqəm və ya kəsr ədəd istifadə edə bilməzsiniz"
            ),
            ParserError::MethodNameNotFound(token) => {
                write!(
                    f,
                    "Metod və ya sahə adı gözlənilirdi amma bu tapıldı: '{token}'"
                )
            }
            ParserError::UnsupportedBuiltInFunction(token) => {
                write!(f, "Dəstəklənməyən funksiya: '{token}'")
            }
            ParserError::ExpectedToken(expected, found) => {
                write!(f, "Gözlənilirdi '{expected}', tapıldı '{found}'")
            }
            ParserError::LoopVarNameNotFound(token) => {
                write!(
                    f,
                    "Dövr yaradılarkən dəyişən adı gözlənilirdi, tapıldı: '{token}'"
                )
            }
            ParserError::StructNameNotFound(token) => {
                write!(f, "Struktur adı gözlənilirdi, tapıldı: '{token}'")
            }
        }
    }
}
