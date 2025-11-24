use crate::{errors::ParserError, shared_ast::Type};
use peekmore::PeekMoreIterator;
use std::borrow::Cow;
use tokenizer::tokens::Token;

pub fn parse_type<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Type<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let token = match tokens.next() {
        Some(token) => token,
        None => return Err(ParserError::UnexpectedEOF),
    };

    let typ = match token {
        Token::Identifier(name) => Type::User(Cow::Borrowed(name)),
        Token::IntegerType => Type::Integer,
        Token::BigIntegerType => Type::BigInteger,
        Token::LowIntegerType => Type::LowInteger,
        Token::ZigConstString => Type::ZigConstString,
        Token::ZigString => Type::ZigString,
        Token::ZigConstArray => Type::ZigConstArray,
        Token::ZigArray => Type::ZigArray,
        Token::ZigNatural => Type::ZigNatural,
        Token::ZigFloat => Type::ZigFloat,
        Token::ZigInteger => Type::ZigInteger,
        Token::BoolType => Type::Bool,
        Token::NaturalType => Type::Natural,
        Token::StringType => Type::String,
        Token::CharType => Type::Char,
        Token::Void => Type::Void,
        Token::FloatType => Type::Float,
        Token::Array => {
            match tokens.next() {
                Some(Token::Operator(op)) if op == "<" => {}
                None => return Err(ParserError::UnexpectedEOF),
                Some(other) => return Err(ParserError::ArrayExpected('<', other.clone())),
            }

            let inner_type = parse_type(tokens)?;

            match tokens.next() {
                Some(Token::Operator(op)) if op == ">" => {}
                None => return Err(ParserError::UnexpectedEOF),
                Some(other) => return Err(ParserError::ArrayExpected('>', other.clone())),
            }

            Type::Array(Box::new(inner_type))
        }
        other => return Err(ParserError::UnexpectedToken(other.clone())),
    };

    Ok(typ)
}
