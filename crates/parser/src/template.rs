use crate::errors::ParserError;
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{
    ast::{Expr, TemplateChunk},
    expressions::parse_expression,
};

pub fn parse_template_string_expr<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<Expr<'a>, ParserError>
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
                            chunks.push(TemplateChunk::Expr(Box::new(parse_expression(tokens)?)));
                        }
                        None => {
                            return Err(ParserError::UnexpectedEOF);
                        }
                    }
                }
            }
            Token::Backtick => {
                break;
            }
            other => {
                return Err(ParserError::UnexpectedToken(other.clone()));
            }
        }
    }

    tokens.next();
    Ok(Expr::TemplateString(chunks))
}
