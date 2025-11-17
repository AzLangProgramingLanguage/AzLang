use errors::ParserError;
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::parser::{
    ast::Expr,
    expressions::{parse_expression, parse_single_expr},
    helpers::expect_token,
};

pub fn parse_match<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let target = Box::new(parse_single_expr(tokens)?);
    let mut arms = Vec::new();
    expect_token(tokens, Token::Newline)?;
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
                let unexpected = (*unexpected).clone();
                return Err(ParserError::UnexpectedToken(unexpected));
            }
        }
    }

    Ok(Expr::Match {
        target: target,
        arms: arms,
    })
}
