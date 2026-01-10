use core::fmt;
use std::{fmt::Display, rc::Rc};

use tokenizer::{iterator::SourceSpan, tokens::Token};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(SourceSpan,Token),
    UnexpectedEOF,
    NotUserDirectValue,
    MethodNameNotFound(Token),
    UnsupportedBuiltInFunction(Token),
    ExpectedToken(Token, Token),
    LoopVarNameNotFound(Token),
    StructNameNotFound(Token),
    ArrayExpected(char, Token),
    FunctionNameNotFound(Token),
    ParameterNameNotFound(Token),
    ParameterNotExpected(Token),
    RParenNotFound(Token),
    StructNotExpected(Token),
    BinaryOpLeftNotExpected(String),
    StructInitArgNotExpected(Token),
    StructInitArgSeparatorNotFound,
    DeclNameNotFound(Token),
    DeclAssignNotFound(Token),
    ObjectTypeNotExpected(Rc<Token>),
    ArrayNotClosed(Token),
    EnumDeclNameNotFound(Token),
    EnumNewLineNotFound(Token),
    UnionDeclNameNotFound(Token),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken(span,token) => write!(f, "{span} Gözlənilməyən token '{token}'"),
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
            ParserError::ArrayExpected(char, token) => {
                write!(f, "Siyahı üçün '{char}', gözlənilirdi tapıldı '{token}'")
            }
            ParserError::FunctionNameNotFound(token) => {
                write!(f, "Funksiya adı gözlənilirdi, tapıldı: '{token}'")
            }
            ParserError::ParameterNameNotFound(token) => {
                write!(f, "Parametr adı gözlənilirdi, tapıldı: '{token}'")
            }
            ParserError::ParameterNotExpected(token) => {
                write!(
                    f,
                    "Parametrdən sonra ',' və ya ')' gözlənilirdi, tapıldı: '{token}'"
                )
            }
            ParserError::RParenNotFound(token) => {
                write!(f, "')' gözlənilirdi, tapıldı: '{token}'")
            }
            ParserError::StructNotExpected(token) => {
                write!(f, "Struct daxilində gözlənilməz token: '{token}'")
            }
            ParserError::BinaryOpLeftNotExpected(string) => {
                write!(f, "Sol tərəf gözlənilirdi, tapıldı: '{string}'")
            }
            ParserError::StructInitArgSeparatorNotFound => {
                write!(
                    f,
                    "Struct init argümentləri arasında ',' və ya '}}' gözlənilirdi"
                )
            }
            ParserError::StructInitArgNotExpected(token) => {
                write!(
                    f,
                    "Struct init argümentləri arasında ':' gözlənilirdi, tapıldı: '{token}'"
                )
            }
            ParserError::DeclNameNotFound(token) => {
                write!(f, "Dəyişən adı gözlənilirdi, tapıldı: '{token}'")
            }
            ParserError::DeclAssignNotFound(token) => {
                write!(f, "'=' operatoru gözlənilirdi, tapıldı: '{token}'")
            }
            ParserError::ObjectTypeNotExpected(token) => {
                write!(f, "Obyekt tipi gözlənilirdi, tapıldı: '{token}'")
            }
            ParserError::ArrayNotClosed(token) => {
                write!(f, "Siyahı sonuna ']' gözlənilirdi, tapıldı: '{token}'")
            }
            ParserError::EnumDeclNameNotFound(token) => {
                write!(
                    f,
                    "tip`-dən sonra identifikator gözlənilirdi, tapıldı: '{token}'"
                )
            }
            ParserError::EnumNewLineNotFound(token) => {
                write!(
                    f,
                    "Enum tərifindən sonra `newline` gözlənilirdi, tapıldı: '{token}'"
                )
            }
            ParserError::UnionDeclNameNotFound(token) => {
                write!(f, "Birləşik tip adı gözlənilirdi, tapıldı: '{token}'")
            }
        }
    }
}
