use crate::errors::ParserError;
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{
    ast::{Expr, MethodType},
    helpers::{expect_token, skip_newlines},
    method::parse_method,
    types::parse_type,
};
/* TODO: burası qısaldılmalıdır. */

pub fn parse_union_type<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => (*name).as_str(),
        None => return Err(ParserError::UnexpectedEOF),
        Some(token) => return Err(ParserError::UnionDeclNameNotFound(token.clone())),
    };
    expect_token(tokens, Token::Newline)?;
    let mut fields = Vec::new();
    let mut methods = Vec::new();
    expect_token(tokens, Token::Indent)?;
    while let Some(token) = tokens.peek() {
        match token {
            Token::Identifier(field_name) => {
                let field_name = (*field_name).as_str();
                tokens.next();

                expect_token(tokens, Token::Colon)?;
                let field_type = parse_type(tokens)?;
                fields.push((field_name, field_type));
                skip_newlines(tokens)?;
            }
            Token::Method => {
                let method_expr = parse_method(tokens)?;
                methods.push(MethodType {
                    name: method_expr.0,
                    params: method_expr.1,
                    body: method_expr.2,
                    return_type: method_expr.3,
                });

                skip_newlines(tokens)?;
            }
            Token::Dedent => {
                tokens.next();
                break;
            }
            Token::Eof => break,
            other => {
                return Err(ParserError::UnexpectedToken((*other).clone()));
            }
        }
    }
    Ok(Expr::UnionType {
        name,
        fields,
        methods,
    })
}
