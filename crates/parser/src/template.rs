use crate::errors::ParserError;
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{
    ast::{Expr, TemplateChunk},
    expressions::parse_expression,
};

fn parse_template_core<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<Vec<TemplateChunk<'a>>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let mut chunks = Vec::new();

    loop {
        let Some(token) = tokens.peek() else { break };

        match token {
            Token::StringLiteral(s) => {
                chunks.push(TemplateChunk::Literal(s));
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
                            let expr = parse_expression(tokens)?;
                            chunks.push(TemplateChunk::Expr(Box::new(expr)));
                        }
                        None => return Err(ParserError::UnexpectedEOF),
                    }
                }
            }

            Token::Backtick => break,

            other => return Err(ParserError::UnexpectedToken((*other).clone())),
        }
    }

    tokens.next();
    Ok(chunks)
}

pub fn parse_template_string_expr<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let chunks = parse_template_core(tokens)?;
    Ok(Expr::TemplateString(chunks))
}
