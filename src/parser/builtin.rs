use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;

use crate::{
    lexer::Token,
    parser::{
        ast::{BuiltInFunction, Expr, Type},
        expression::parse_expression,
    },
};

pub fn parse_builtin<'a, I>(tokens: &mut PeekMoreIterator<I>, token: &Token) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let (function, return_type) = match token {
        Token::Print => (BuiltInFunction::Print, Type::Void),
        Token::Input => (BuiltInFunction::Input, Type::Metn),
        Token::Len => (BuiltInFunction::Len, Type::Integer),
        Token::NumberFn => (BuiltInFunction::Number, Type::Integer),
        Token::Sum => (BuiltInFunction::Sum, Type::Integer),
        Token::RangeFn => (
            BuiltInFunction::Range,
            Type::Siyahi(Box::new(Type::Integer)),
        ),
        Token::LastWord => (BuiltInFunction::LastWord, Type::Metn),
        Token::Sqrt => (BuiltInFunction::Sqrt, Type::Float),
        Token::Timer => (BuiltInFunction::Timer, Type::Integer),
        Token::Max => (BuiltInFunction::Max, Type::Float),
        Token::Zig => (BuiltInFunction::Zig, Type::Void),
        Token::Min => (BuiltInFunction::Min, Type::Float),
        Token::Mod => (BuiltInFunction::Mod, Type::Integer),
        Token::Round => (BuiltInFunction::Round, Type::Float),
        Token::Floor => (BuiltInFunction::Floor, Type::Float),
        Token::Ceil => (BuiltInFunction::Ceil, Type::Float),
        Token::StrLower => (BuiltInFunction::StrLower, Type::Metn),
        Token::Allocator => (BuiltInFunction::Allocator, Type::Void),
        Token::StrUpper => (BuiltInFunction::StrUpper, Type::Metn),
        Token::Trim => (BuiltInFunction::Trim, Type::Metn),
        Token::StrReverse => (BuiltInFunction::StrReverse, Type::Metn),
        Token::ConvertString => (BuiltInFunction::ConvertString, Type::Metn),
        other => return Err(eyre!("Bilinməyən funksiya: {:?}", other)),
    };
    let mut args = Vec::new();

    if let Some(Token::LParen) = tokens.peek() {
        tokens.next();
        while let Some(token) = tokens.peek() {
            match token {
                Token::RParen => {
                    tokens.next();
                    break;
                }
                Token::Comma => {
                    tokens.next();
                }
                _ => {
                    let expr = parse_expression(tokens)?;
                    args.push(expr);
                }
            }
        }
    }

    Ok(Expr::BuiltInCall {
        function,
        args,
        return_type,
    })
}
