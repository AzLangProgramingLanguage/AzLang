use crate::parser::ast::Type;
use errors::ParserError;
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
        Token::Identifier(name) => Type::Istifadeci(Cow::Borrowed(name)),
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
        Token::StringType => Type::Metn,
        Token::CharType => Type::Char,
        Token::Void => Type::Void,
        Token::FloatType => Type::Float,
        Token::Array => {
            match tokens.next() {
                Some(Token::Operator(op)) if op == "<" => {}
                other => return Err(ParserError::UnexpectedToken(other)),
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
