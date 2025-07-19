use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;

use crate::{
    lexer::Token,
    parser::{
        ast::{BuiltInFunction, Expr, Type},
        expression::parse_single_expr,
    },
};

pub fn parse_builtin<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let token = tokens.peek().ok_or_else(|| eyre!("EOF gözlənilməz"))?;
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
        Token::Timer => (BuiltInFunction::Timer, Type::Float),
        Token::Max => (BuiltInFunction::Max, Type::Float),
        Token::Min => (BuiltInFunction::Min, Type::Float),
        Token::Mod => (BuiltInFunction::Mod, Type::Integer),
        Token::Round => (BuiltInFunction::Round, Type::Float),
        Token::Floor => (BuiltInFunction::Floor, Type::Float),
        Token::Ceil => (BuiltInFunction::Ceil, Type::Float),
        other => return Err(eyre!("Tanınmamış built-in function: {:?}", other)),
    };

    let mut args = Vec::new();
    tokens.next();

    if let Some(Token::LParen) = tokens.next() {
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
                    let expr = parse_single_expr(tokens)?;
                    args.push(expr);
                    tokens.next();
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
