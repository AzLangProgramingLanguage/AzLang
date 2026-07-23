use crate::{binary_op::parse_expression, errors::ParserError};
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

use crate::ast::{Expr, TemplateChunk};

pub fn parse_template_string_expr(tokens: &mut Tokens) -> Result<Expr, ParserError> {
    let mut chunks = Vec::new();
    while let Some(token) = tokens.next() {
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
            } => loop {
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
            },

            SpannedToken {
                token: Token::Backtick,
                ..
            } => break,

            other => return Err(ParserError::UnexpectedToken(other.span, other.token)),
        }
    }

    Ok(Expr::TemplateString(chunks))
}
