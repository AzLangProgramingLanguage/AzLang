use crate::errors::ParserError;
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

use crate::{
    ast::{Expr, TemplateChunk},
    expressions::parse_expression,
};

pub fn parse_template_string_expr<'a>(tokens: &mut Tokens) -> Result<Expr<'a>, ParserError> {
    let mut chunks = Vec::new();
    loop {
        let Some(token) = tokens.next() else { break };

        match token {
            SpannedToken {
                token: Token::StringLiteral(s),
                ..
            } => {
                chunks.push(TemplateChunk::Literal(s));
            }

            SpannedToken {
                token: Token::InterpolationStart,
                ..
            } => {
                tokens.next();
                loop {
                    match tokens.peek() {
                        Some(SpannedToken {
                            token: Token::InterpolationEnd,
                            ..
                        }) => {
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

            SpannedToken {
                token: Token::Backtick,
                ..
            } => break,

            other => return Err(ParserError::UnexpectedToken(other.span, other.token)),
        }
    }

    Ok(Expr::TemplateString(chunks))
}
