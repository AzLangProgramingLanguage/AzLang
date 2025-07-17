use color_eyre::eyre::{Result, eyre};
use std::borrow::Cow;
use std::iter::Peekable;

use crate::{
    lexer::Token,
    parser::{
        ast::{Expr, TemplateChunk},
        expression::parse_single_expr,
    },
};

pub fn parse_template_string_expr<'a, I>(tokens: &mut Peekable<I>) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut chunks = Vec::new();

    loop {
        let token = tokens
            .next()
            .ok_or_else(|| eyre!("Template string bitmədi (EOF)"))?;

        match token {
            Token::StringLiteral(s) => {
                chunks.push(TemplateChunk::Literal(s));
            }
            Token::InterpolationStart => {
                if matches!(tokens.peek(), Some(Token::InterpolationEnd)) {
                    tokens.next();
                    chunks.push(TemplateChunk::Expr(Box::new(Expr::VariableRef {
                        name: Cow::Borrowed(""),
                        symbol: None,
                    })));
                    continue;
                }
                let expr = parse_single_expr(tokens)?;
                chunks.push(TemplateChunk::Expr(Box::new(expr)));

                match tokens.next() {
                    Some(Token::InterpolationEnd) => {}
                    other => {
                        return Err(eyre!(
                            "Template interpolasiyası bağlanmalıdır (`}}`), tapıldı: {:?}",
                            other
                        ));
                    }
                }
            }
            Token::Backtick => {
                // Template bağlanışı
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

    Ok(Expr::TemplateString(chunks))
}
