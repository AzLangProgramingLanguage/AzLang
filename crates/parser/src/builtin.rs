use core::panic;

use crate::{
    binary_op::parse_expression,
    errors::ParserError,
    shared_ast::{BuiltInFunction, StringEnum, Type},
};
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

use crate::ast::Expr;

pub fn parse_builtin<'a>(token: SpannedToken, tokens: &mut Tokens) -> Result<Expr, ParserError> {
    let (function, return_type) = match token.token {
        Token::Print => (BuiltInFunction::Print, Type::Void),
        Token::Input => (
            BuiltInFunction::Input,
            Type::String(StringEnum::DynamicString),
        ),
        Token::Len => (BuiltInFunction::Len, Type::Natural),
        Token::NumberFn => (BuiltInFunction::Number, Type::Integer),
        Token::Sum => (BuiltInFunction::Sum, Type::Integer),
        Token::RangeFn => (BuiltInFunction::Range, Type::Array(Box::new(Type::Integer))),

        Token::LastWord => (BuiltInFunction::LastWord, Type::Void),
        Token::Timer => (BuiltInFunction::Timer, Type::Integer),
        Token::Max => (BuiltInFunction::Max, Type::Integer),
        Token::Zig => (BuiltInFunction::Zig, Type::Void),
        Token::StrLower => (
            BuiltInFunction::StrLower,
            Type::String(StringEnum::DynamicString),
        ),
        Token::Allocator => (BuiltInFunction::Allocator, Type::Void),
        Token::StrUpper => (
            BuiltInFunction::StrUpper,
            Type::String(StringEnum::DynamicString),
        ),
        Token::Trim => (
            BuiltInFunction::Trim,
            Type::String(StringEnum::DynamicString),
        ),
        Token::Min => (BuiltInFunction::Min, Type::Integer),
        Token::StrReverse => (
            BuiltInFunction::StrReverse,
            Type::String(StringEnum::DynamicString),
        ),
        Token::ConvertString => (
            BuiltInFunction::ConvertString,
            Type::String(StringEnum::DynamicString),
        ),
        _ => return Err(ParserError::UnsupportedBuiltInFunction(token.token.clone())),
    };
    let mut args = Vec::new();
    if let Some(SpannedToken {
        token: Token::LParen,
        ..
    }) = tokens.peek()
    {
        tokens.next();
        while let Some(token) = tokens.peek() {
            match token {
                SpannedToken {
                    token: Token::RParen,
                    span,
                } => {
                    tokens.next();
                    break;
                }
                SpannedToken {
                    token: Token::Comma,
                    span,
                } => {
                    tokens.next();
                }
                SpannedToken {
                    token: Token::Newline,
                    span,
                } => {
                    return Err(ParserError::ExpectedToken(Token::RParen, Token::Newline));
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
