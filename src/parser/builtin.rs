use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;

use crate::{
    lexer::Token,
    parser::{
        ast::{BuiltInFunction, Expr, Type},
        expression::parse_expression,
    },
    translations::parser_errors::ParserError,
};

pub fn parse_builtin<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    token: &Token,
) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let (function, return_type) = match token {
        Token::Print => (BuiltInFunction::Print, Type::Void),
        Token::Input => (BuiltInFunction::Input, Type::String),
        Token::Len => (BuiltInFunction::Len, Type::Integer),
        Token::NumberFn => (BuiltInFunction::Number, Type::Integer),
        Token::Sum => (BuiltInFunction::Sum, Type::Integer),
        Token::RangeFn => (BuiltInFunction::Range, Type::Array(Box::new(Type::Integer))),

        Token::LastWord => (BuiltInFunction::LastWord, Type::String),
        Token::Sqrt => (BuiltInFunction::Sqrt, Type::ZigFloat),
        Token::Timer => (BuiltInFunction::Timer, Type::Integer),
        Token::Max => (BuiltInFunction::Max, Type::ZigFloat),
        Token::Zig => (BuiltInFunction::Zig, Type::Void),
        Token::Min => (BuiltInFunction::Min, Type::ZigFloat),
        Token::Mod => (BuiltInFunction::Mod, Type::ZigInteger),
        Token::Round => (BuiltInFunction::Round, Type::ZigFloat),
        Token::Floor => (BuiltInFunction::Floor, Type::ZigFloat),
        Token::Ceil => (BuiltInFunction::Ceil, Type::ZigFloat),
        Token::StrLower => (BuiltInFunction::StrLower, Type::String),
        Token::Allocator => (BuiltInFunction::Allocator, Type::Void),
        Token::StrUpper => (BuiltInFunction::StrUpper, Type::String),
        Token::Trim => (BuiltInFunction::Trim, Type::String),
        Token::StrReverse => (BuiltInFunction::StrReverse, Type::String),
        Token::ConvertString => (BuiltInFunction::ConvertString, Type::String),
        other => return Err(ParserError::UnknownFunction(format!("{:?}", other))),
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
