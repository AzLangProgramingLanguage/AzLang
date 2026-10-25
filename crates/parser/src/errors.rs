use core::fmt;
use std::{fmt::Display, rc::Rc};

use tokenizer::{errors::LexerError, iterator::SourceSpan, tokens::Token};

#[derive(Debug)]
pub enum ParserError {
    LexerError(LexerError),
    UnexpectedToken(SourceSpan, Token),
    UnexpectedEOF,
    NotUserDirectValue,
    MethodNameNotFound(Token),
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
    FunctionAlreadyAsigned(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::LexerError(error) => write!(f, "{}", error),
            ParserError::UnexpectedToken(span, token) => {
                write!(f, "{span} Unexpected token '{token}'")
            }
            ParserError::UnexpectedEOF => write!(f, "Unexpected EOF"),
            ParserError::NotUserDirectValue => write!(
                f,
                "A string, number or float literal cannot be used directly"
            ),
            ParserError::MethodNameNotFound(token) => {
                write!(
                    f,
                    "Method or field name expected but found: '{token}'"
                )
            }
            ParserError::ExpectedToken(expected, found) => {
                write!(f, "Expected '{expected}', found '{found}'")
            }
            ParserError::LoopVarNameNotFound(token) => {
                write!(f, "Loop variable name expected, found: '{token}'")
            }
            ParserError::StructNameNotFound(token) => {
                write!(f, "Struct name expected, found: '{token}'")
            }
            ParserError::ArrayExpected(char, token) => {
                write!(f, "Expected '{char}' for list, found '{token}'")
            }
            ParserError::FunctionNameNotFound(token) => {
                write!(f, "Function name expected, found: '{token}'")
            }
            ParserError::ParameterNameNotFound(token) => {
                write!(f, "Parameter name expected, found: '{token}'")
            }
            ParserError::ParameterNotExpected(token) => {
                write!(f, "Expected ',' or ')' after parameter, found: '{token}'")
            }
            ParserError::RParenNotFound(token) => {
                write!(f, "')' expected, found: '{token}'")
            }
            ParserError::StructNotExpected(token) => {
                write!(f, "Unexpected token inside struct: '{token}'")
            }
            ParserError::BinaryOpLeftNotExpected(string) => {
                write!(f, "Left operand expected, found: '{string}'")
            }
            ParserError::StructInitArgSeparatorNotFound => {
                write!(f, "Expected ',' or '}}' between struct init arguments")
            }
            ParserError::StructInitArgNotExpected(token) => {
                write!(
                    f,
                    "Expected ':' between struct init arguments, found: '{token}'"
                )
            }
            ParserError::DeclNameNotFound(token) => {
                write!(f, "Variable name expected, found: '{token}'")
            }
            ParserError::DeclAssignNotFound(token) => {
                write!(f, "'=' operator expected, found: '{token}'")
            }
            ParserError::ObjectTypeNotExpected(token) => {
                write!(f, "Object type expected, found: '{token}'")
            }
            ParserError::ArrayNotClosed(token) => {
                write!(f, "']' expected at the end of the list, found: '{token}'")
            }
            ParserError::EnumDeclNameNotFound(token) => {
                write!(f, "Identifier expected after `type`, found: '{token}'")
            }
            ParserError::EnumNewLineNotFound(token) => {
                write!(f, "`newline` expected after enum declaration, found: '{token}'")
            }
            ParserError::UnionDeclNameNotFound(token) => {
                write!(f, "Union type name expected, found: '{token}'")
            }
            ParserError::FunctionAlreadyAsigned(s) => {
                write!(f, "Function named {s} already exists")
            }
        }
    }
}

impl From<LexerError> for ParserError {
    fn from(e: LexerError) -> Self {
        ParserError::LexerError(e)
    }
}
impl std::error::Error for ParserError {}
