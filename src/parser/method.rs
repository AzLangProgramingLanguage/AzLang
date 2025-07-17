use crate::{
    lexer::Token,
    parser::{
        ast::{Expr, Parameter, Type},
        expression::parse_single_expr,
    },
};
use color_eyre::eyre::{Result, eyre};
use std::iter::Peekable;

pub fn parse_method<'a, I>(
    tokens: &mut Peekable<I>,
) -> Result<(&'a str, Vec<Parameter<'a>>, Vec<Expr<'a>>, Option<Type<'a>>)>
where
    I: Iterator<Item = &'a Token>,
{
    expect_token(tokens, Token::Method)?;

    // Metod adı
    let name = match tokens.next() {
        Some(Token::Identifier(n)) => (*n).as_str(),
        other => return Err(eyre!("Method adı gözlənilirdi, tapıldı: {:?}", other)),
    };

    // Parametrlər hələlik boş olur
    expect_token(tokens, Token::LParen)?;
    expect_token(tokens, Token::RParen)?;

    // Return tipi varsa oxu
    let return_type = None;

    expect_token(tokens, Token::Newline)?;
    expect_token(tokens, Token::Indent)?;

    let mut body = Vec::new();

    while let Some(token) = tokens.peek() {
        match token {
            Token::Dedent => {
                tokens.next();
                break;
            }
            Token::Newline => {
                tokens.next();
            }
            Token::Eof => break,
            _ => {
                let expr = parse_single_expr(tokens)?;
                body.push(expr);
                while matches!(tokens.peek(), Some(Token::Semicolon | Token::Newline)) {
                    tokens.next();
                }
            }
        }
    }

    // default self parametri əlavə edirik
    let params = vec![Parameter {
        name: "self".into(),
        typ: Type::Any,
        is_mutable: false,
        is_pointer: false,
    }];

    Ok((name, params, body, return_type))
}

fn expect_token<'a, I>(tokens: &mut Peekable<I>, expected: Token) -> Result<()>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens.next() {
        Some(t) if *t == expected => Ok(()),
        other => Err(eyre!("Gözlənilirdi: {:?}, tapıldı: {:?}", expected, other)),
    }
}
