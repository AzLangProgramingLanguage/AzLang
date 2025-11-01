use crate::{lexer::Token, parser::ast::Type};
use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;
use std::borrow::Cow;

pub fn parse_type<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Type<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let token = tokens
        .next()
        .ok_or_else(|| eyre!("Tip gözlənilirdi, amma EOF tapıldı"))?;

    let typ = match token {
        Token::Identifier(name) => Type::User(Cow::Borrowed(name), Cow::Borrowed(name)),
        Token::IntegerType => Type::Integer,
        Token::BigIntegerType => Type::BigInteger,
        Token::LowIntegerType => Type::LowInteger,
        Token::BoolType => Type::Bool,
        Token::NaturalType => Type::Natural,
        Token::StringType => Type::String,
        Token::CharType => Type::Char,
        Token::Void => Type::Void,
        Token::FloatType => Type::Float,
        Token::ZigConstString => Type::LiteralConstString,
        Token::ZigString => Type::LiteralString,
        Token::ZigConstArray => Type::ZigConstArray,
        Token::ZigArray => Type::ZigArray,
        Token::ZigNatural => Type::ZigNatural,
        Token::ZigFloat => Type::ZigFloat,
        Token::ZigInteger => Type::ZigInteger,
        Token::Value => Type::User(Cow::Borrowed("ValueEnum"), Cow::Borrowed("ValueEnum")),
        Token::Array => {
            match tokens.next() {
                Some(Token::Operator(op)) if op == "<" => {}
                other => return Err(eyre!("Siyahı üçün '<' gözlənilirdi, tapıldı: {:?}", other)),
            }

            let inner_type = parse_type(tokens)?;

            match tokens.next() {
                Some(Token::Operator(op)) if op == ">" => {}
                other => return Err(eyre!("Siyahı üçün '>' gözlənilirdi, tapıldı: {:?}", other)),
            }

            Type::Array(Box::new(inner_type))
        }
        other => return Err(eyre!("Tanınmayan tip tokeni: {:?}", other)),
    };

    Ok(typ)
}
