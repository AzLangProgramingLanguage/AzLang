use crate::{
    errors::ParserError,
    parsing_for::{parse_expression_typed, parse_single_expr_typed},
    typed_ast::TypedExpr,
};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{
    ast::Expr,
    expressions::{parse_expression, parse_single_expr},
    helpers::expect_token,
};

pub fn parse_match_core<'a, I, Out>(
    tokens: &mut PeekMoreIterator<I>,
    single_expr: impl Fn(&mut PeekMoreIterator<I>) -> Result<Out, ParserError>,
    expression: impl Fn(&mut PeekMoreIterator<I>) -> Result<Out, ParserError>,
    finish: impl Fn(Box<Out>, Vec<(Out, Vec<Out>)>) -> Out,
) -> Result<Out, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let target = Box::new(single_expr(tokens)?);
    let mut arms = Vec::new();
    expect_token(tokens, Token::Newline)?;
    expect_token(tokens, Token::Indent)?;

    while let Some(token) = tokens.peek() {
        match token {
            Token::StringLiteral(_)
            | Token::Number(_)
            | Token::Underscore
            | Token::Identifier(_) => {
                let pattern = single_expr(tokens)?;

                expect_token(tokens, Token::Arrow)?;

                let expr = expression(tokens)?;

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

    Ok(finish(target, arms))
}

pub fn parse_match<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_match_core(
        tokens,
        |tokens| parse_single_expr(tokens),
        |tokens| parse_expression(tokens),
        |target, arms| Expr::Match { target, arms },
    )
}

pub fn parse_match_typed<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_match_core(
        tokens,
        |tokens| parse_single_expr_typed(tokens),
        |tokens| parse_expression_typed(tokens),
        |target, arms| TypedExpr::Match { target, arms },
    )
}
