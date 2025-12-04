use crate::{
    ast::Expr,
    errors::ParserError,
    expressions::{parse_expression, parse_single_expr},
    helpers::expect_token,
};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

pub fn parse_match<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let target = Box::new(parse_single_expr(tokens)?);

    expect_token(tokens, Token::Newline)?;
    expect_token(tokens, Token::Indent)?;

    let mut arms = Vec::new();

    while let Some(tok) = tokens.peek() {
        match tok {
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
                return Err(ParserError::UnexpectedToken((*unexpected).clone()));
            }
        }
    }

    Ok(Expr::Match { target, arms })
}
