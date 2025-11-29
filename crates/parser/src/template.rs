use crate::{
    errors::ParserError,
    parsing_for::parse_expression_typed,
    typed_ast::{TypedExpr, TypedTemplateChunk},
};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{
    ast::{Expr, TemplateChunk},
    expressions::parse_expression,
};

fn parse_template_core<'a, I, Chunk, Out>(
    tokens: &mut PeekMoreIterator<I>,
    literal_fn: impl Fn(&'a str) -> Chunk,
    expr_fn: impl Fn(Out) -> Chunk,
    parse_expr: impl Fn(&mut PeekMoreIterator<I>) -> Result<Out, ParserError>,
    finish: impl Fn(Vec<Chunk>) -> Out,
) -> Result<Out, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let mut chunks = Vec::new();

    loop {
        let token = match tokens.peek() {
            Some(token) => *token,
            None => break,
        };

        match token {
            Token::StringLiteral(s) => {
                chunks.push(literal_fn(s));
                tokens.next();
            }
            Token::InterpolationStart => {
                tokens.next();

                loop {
                    match tokens.peek() {
                        Some(Token::InterpolationEnd) => {
                            tokens.next();
                            break;
                        }
                        Some(_) => {
                            let expr_parsed = parse_expr(tokens)?;
                            chunks.push(expr_fn(expr_parsed));
                        }
                        None => return Err(ParserError::UnexpectedEOF),
                    }
                }
            }
            Token::Backtick => break,
            other => return Err(ParserError::UnexpectedToken(other.clone())),
        }
    }

    tokens.next();
    Ok(finish(chunks))
}

pub fn parse_template_string_expr_typed<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_template_core(
        tokens,
        |s| TypedTemplateChunk::Literal(s),
        |expr| TypedTemplateChunk::TypedExpr(Box::new(expr)),
        |toks| parse_expression_typed(toks),
        |chunks| TypedExpr::TemplateString(chunks),
    )
}

pub fn parse_template_string_expr<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_template_core(
        tokens,
        |s| TemplateChunk::Literal(s),
        |expr| TemplateChunk::Expr(Box::new(expr)),
        |toks| parse_expression(toks),
        |chunks| Expr::TemplateString(chunks),
    )
}
