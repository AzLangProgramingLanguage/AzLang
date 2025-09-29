use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;

use crate::{
    lexer::Token,
    parser::{
        ast::Expr,
        expression::{parse_expression, parse_single_expr},
        helper::expect_token,
    },
};

pub fn parse_match<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let target = Box::new(parse_single_expr(tokens)?);

    match tokens.peek() {
        Some(Token::Newline) => {
            tokens.next();
        }
        other => {
            return Err(eyre!("Match parsing xətası: {:?}", other));
        }
    }

    let mut arms = Vec::new();
    expect_token(tokens, Token::Indent)?;

    while let Some(token) = tokens.peek() {
        match token {
            Token::StringLiteral(_)
            | Token::Number(_)
            | Token::Underscore
            | Token::Identifier(_) => {
                let pattern = parse_single_expr(tokens)?;

                expect_token(tokens, Token::Arrow)?;

                let expr = parse_expression(tokens)?;

                arms.push((pattern, vec![expr]));

                expect_token(tokens, Token::Newline)?;
            }
            Token::Dedent => {
                tokens.next();
                break;
            }

            Token::Newline => {
                tokens.next();
            }

            Token::Eof => break,
            unexpected => {
                return Err(eyre!("Gözlənilməz token match armında: {:?}", unexpected));
            }
        }
    }

    Ok(Expr::Match {
        target: target,
        arms: arms,
    })
}
