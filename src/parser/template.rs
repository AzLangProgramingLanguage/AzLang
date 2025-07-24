use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;
use std::borrow::Cow;

use crate::{
    lexer::Token,
    parser::{
        ast::{Expr, TemplateChunk},
        expression::{parse_expression, parse_single_expr},
    },
};

pub fn parse_template_string_expr<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut chunks = Vec::new();
    loop {
        let token = tokens
            .peek()
            .ok_or_else(|| eyre!("Template string bitmədi (EOF)"))?;

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
                        Some(token) => {
                            chunks.push(TemplateChunk::Expr(Box::new(parse_expression(tokens)?)));
                        }
                        None => {
                            return Err(eyre!("Template string bitmədi (EOF)"));
                        }
                    }
                }
            }
            Token::Backtick => {
                break;
            }
            other => {
                return Err(eyre!(
                    "Template string içində tanınmayan token: {:?}",
                    other
                ));
            }
        }
    }

    tokens.next();
    Ok(Expr::TemplateString(chunks))
}
