use color_eyre::eyre::Result;
use peekmore::PeekMoreIterator;

use crate::{
    lexer::Token,
    parser::{
        ast::{Expr, TemplateChunk},
        expression::parse_expression,
    },
    translations::parser_errors::ParserError,
};

pub fn parse_template_string_expr<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let mut chunks = Vec::new();
    loop {
        let token = tokens.peek().ok_or(ParserError::Eof)?;

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
                            return Err(ParserError::Eof);
                        }
                    }
                }
            }
            Token::Backtick => {
                break;
            }
            other => {
                return Err(ParserError::TemplateTokenNotFound(other.to_string()));
            }
        }
    }

    tokens.next();
    Ok(Expr::TemplateString(chunks))
}
