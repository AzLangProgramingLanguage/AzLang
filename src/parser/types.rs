use crate::{lexer::Token, parser::ast::Type};
use color_eyre::eyre::{Result, eyre};
use std::{borrow::Cow, iter::Peekable};

pub fn parse_type<'a, I>(tokens: &mut Peekable<I>) -> Result<Type<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let token = tokens
        .next()
        .ok_or_else(|| eyre!("Tip gözlənilirdi, amma EOF tapıldı"))?;

    let typ = match token {
        Token::Identifier(name) => Type::Istifadeci(Cow::Borrowed(name)),
        Token::IntegerType => Type::Integer,
        Token::BigIntegerType => Type::BigInteger,
        Token::LowIntegerType => Type::LowInteger,
        Token::BoolType => Type::Bool,
        Token::StringType => Type::Metn,
        Token::CharType => Type::Char,
        Token::FloatType => Type::Float,
        Token::Array => {
            match tokens.next() {
                Some(Token::Operator(op)) if op == "<" => {}
                other => return Err(eyre!("Siyahı üçün '<' gözlənilirdi, tapıldı: {:?}", other)),
            }

            let inner_type = parse_type(tokens)?;

            // '>' gözlənilir
            match tokens.next() {
                Some(Token::Operator(op)) if op == ">" => {}
                other => return Err(eyre!("Siyahı üçün '>' gözlənilirdi, tapıldı: {:?}", other)),
            }

            Type::Siyahi(Box::new(inner_type))
        }
        other => return Err(eyre!("Tanınmayan tip tokeni: {:?}", other)),
    };

    Ok(typ)
}
