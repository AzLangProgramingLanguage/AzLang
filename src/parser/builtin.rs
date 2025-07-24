use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;

use crate::{
    lexer::Token,
    parser::{ast::{BuiltInFunction, Expr, Type}, expression::{parse_expression, parse_single_expr}}};

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
        Token::Min => (BuiltInFunction::Min, Type::Float),
        Token::Mod => (BuiltInFunction::Mod, Type::Integer),
        Token::Round => (BuiltInFunction::Round, Type::Float),
        Token::Floor => (BuiltInFunction::Floor, Type::Float),
        Token::Ceil => (BuiltInFunction::Ceil, Type::Float),
        other => return Err(eyre!("Bilinməyən funksiya: {:?}", other)),
    };

    let mut args = Vec::new();

    if let Some(Token::LParen) = tokens.peek() {
        tokens.next();
        while let Some(token) = tokens.peek() {
            match token {
                Token::RParen => {
                    tokens.next();
                    break; //Burada dayanması lazımken dayanmır sıradaki Tokene keçir yeni NewLine
                }
                Token::Comma => {
                    tokens.next();
                }
                _ => {
                    println!("parse_builtin çıxır, növbəti token2 {:?}", tokens.peek()); //Burada NewLine Çıxır amma bu Newline ye çatmadan dayanması lazımdı
                    let expr = parse_expression(tokens)?;
                    println!("parse_builtin den sonra gelen, {:?}",tokens.peek());
                    args.push(expr);
                }
            }
        }
    }

    println!("parse_builtin çıxır, növbəti token {:?}", tokens.peek()); //parse_builtin çıxır, növbəti token Some(RParen)
    /* : Parser xətası: Naməlum token: RParen */
    Ok(Expr::BuiltInCall {
        function,
        args,
        return_type,
    })
}

/* Tokens: [
    Print,
    LParen,
    Mod,
    LParen,
    Operator(
        "-",
    ),
    Number(
        1,
    ),
    RParen,
    RParen,
    Newline,
    Newline,
    Eof,
] */
