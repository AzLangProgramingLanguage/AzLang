use crate::{
    dd,
    lexer::Token,
    parser::{
        ast::{Expr, Parameter, Type},
        expression::parse_expression,
        types::parse_type,
    },
};
use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;
type MethodResultType<'a> = (
    &'a str,
    Vec<Parameter<'a>>,
    Vec<Expr<'a>>,
    Option<Type<'a>>,
    bool,
);
pub fn parse_method<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<MethodResultType<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    expect_token(tokens, Token::Method)?;

    let name = match tokens.next() {
        Some(Token::Identifier(n)) => (*n).as_str(),
        other => return Err(eyre!("Method adı gözlənilirdi, tapıldı: {:?}", other)),
    };

    expect_token(tokens, Token::LParen)?;

    let mut params = Vec::new();
    while let Some(token) = tokens.peek() {
        match token {
            Token::ConstantDecl | Token::MutableDecl | Token::Identifier(_) => {
                // Mutability
                let is_mutable = match tokens.peek() {
                    Some(Token::MutableDecl) => {
                        tokens.next();
                        true
                    }
                    Some(Token::ConstantDecl) => {
                        tokens.next();
                        false
                    }
                    _ => false,
                };

                let param_name = match tokens.next() {
                    Some(Token::Identifier(s)) => (*s).as_str(),
                    other => return Err(eyre!("Parametr adı gözlənilirdi, tapıldı: {:?}", other)),
                };
                dbg!(param_name);

                let mut param_type = Type::Any;

                match tokens.peek() {
                    Some(Token::Comma) => {
                        tokens.next();
                    }
                    Some(Token::Colon) => {
                        tokens.next();

                        param_type = parse_type(tokens)?;
                    }
                    Some(Token::RParen) => break,
                    other => {
                        return Err(eyre!(
                            "Parametrdən sonra ',' və ya ')' gözlənilirdi, tapıldı: {:?}",
                            other
                        ));
                    }
                }
                params.push(Parameter {
                    name: param_name.to_string(),
                    typ: param_type,
                    is_mutable,
                    is_pointer: false,
                });
            }
            Token::RParen => break,
            other => {
                return Err(eyre!(
                    "Parametr və ya ')' gözlənilirdi, tapıldı: {:?}",
                    other
                ));
            }
        }
    }

    expect_token(tokens, Token::RParen)?;
    expect_token(tokens, Token::Colon)?;
    let return_type = Some(parse_type(tokens)?);
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
                let expr = parse_expression(tokens)?;
                body.push(expr);
            }
        }
    }

    Ok((name, params, body, return_type, false))
}

fn expect_token<'a, I>(tokens: &mut PeekMoreIterator<I>, expected: Token) -> Result<()>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens.next() {
        Some(t) if *t == expected => Ok(()),
        other => Err(eyre!("Gözlənilirdi: {:?}, tapıldı: {:?}", expected, other)),
    }
}
