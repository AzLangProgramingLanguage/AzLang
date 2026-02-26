use crate::{
    binary_op::parse_expression,
    errors::ParserError,
    shared_ast::{BuiltInFunction, Type},
};
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

use crate::ast::Expr;

pub fn parse_builtin<'a>(
    tokens: &mut Tokens,
    token: &SpannedToken,
) -> Result<Expr<'a>, ParserError> {
    let (function, return_type) = match &token.token {
        Token::Print => (BuiltInFunction::Print, Type::Void),
        Token::Input => (BuiltInFunction::Input, Type::String),
        Token::Len => (BuiltInFunction::Len, Type::Integer),
        Token::NumberFn => (BuiltInFunction::Number, Type::Integer),
        Token::Sum => (BuiltInFunction::Sum, Type::Integer),
        Token::RangeFn => (BuiltInFunction::Range, Type::Array(Box::new(Type::Integer))),

        Token::LastWord => (BuiltInFunction::LastWord, Type::String),
        Token::Timer => (BuiltInFunction::Timer, Type::Integer),
        Token::Max => (BuiltInFunction::Max, Type::Integer),
        Token::Zig => (BuiltInFunction::Zig, Type::Void),
        Token::StrLower => (BuiltInFunction::StrLower, Type::String),
        Token::Allocator => (BuiltInFunction::Allocator, Type::Void),
        Token::StrUpper => (BuiltInFunction::StrUpper, Type::String),
        Token::Trim => (BuiltInFunction::Trim, Type::String),
        Token::Min => (BuiltInFunction::Min, Type::Integer),
        Token::StrReverse => (BuiltInFunction::StrReverse, Type::String),
        Token::ConvertString => (BuiltInFunction::ConvertString, Type::String),
        other => return Err(ParserError::UnsupportedBuiltInFunction(other.clone())),
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
                    return Err(ParserError::ExpectedToken(Token::RParen, Token::Newline)); //TODO: Uncesarry Clone  Error Must be Increate messsage
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
