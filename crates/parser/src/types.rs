use crate::{errors::ParserError, shared_ast::Type};
use std::borrow::Cow;
use tokenizer::{iterator::{SpannedToken, Tokens}, tokens::Token};

pub fn parse_type<'a>(tokens: &mut Tokens) -> Result<Type<'a>, ParserError>
{
    let token = match tokens.next() {
        Some(token) => token,
        None => return Err(ParserError::UnexpectedEOF),
    };

    let typ = match token.token {
        Token::Identifier(name) => Type::User(Cow::Owned(name)),
        Token::IntegerType => Type::Integer,
        Token::BigIntegerType => Type::BigInteger,
        Token::LowIntegerType => Type::LowInteger,
        Token::ZigString => Type::LiteralString,
        Token::ZigConstString => Type::LiteralConstString,
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
                Some(SpannedToken{ token: Token::Less, ..})=> {}
                None => return Err(ParserError::UnexpectedEOF),
                Some(other) => return Err(ParserError::ArrayExpected('<', other.token)),
            }

            let inner_type = parse_type(tokens)?;

            match tokens.next() {
                Some(SpannedToken{ token: Token::Greater, ..})=> {}
                None => return Err(ParserError::UnexpectedEOF),
                Some(other) => return Err(ParserError::ArrayExpected('>', other.token)),
            }

            Type::Array(Box::new(inner_type))
        }
        ref _other => return Err(ParserError::UnexpectedToken(token.span, token.token)),
    };

    Ok(typ)
}
