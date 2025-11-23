use crate::errors::ParserError;
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{
    ast::{Expr, MethodType},
    expressions::parse_expression,
    helpers::{expect_token, skip_newlines},
    method::parse_method,
    types::parse_type,
};

pub fn parse_struct_def<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => (*name).as_str(), /* TODO: unwrap işletme Hardcoded  */
        other => return Err(ParserError::StructNameNotFound(other.unwrap().clone())),
    };
    expect_token(tokens, Token::Newline)?;

    let mut fields = Vec::new();
    let mut methods: Vec<MethodType<'a>> = Vec::new();

    expect_token(tokens, Token::Indent)?;

    while let Some(token) = tokens.peek() {
        match token {
            Token::Identifier(field_name) => {
                let field_name = (*field_name).as_str();
                tokens.next();

                expect_token(tokens, Token::Colon)?;
                let field_type = parse_type(tokens)?;
                if let Some(Token::Operator(s)) = tokens.next()
                    && s == "="
                {
                    let value = parse_expression(tokens)?;
                    fields.push((field_name, field_type, Some(value)));
                } else {
                    fields.push((field_name, field_type, None));
                }

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
                return Err(ParserError::StructNotExpected((*other).clone())); /* FIXME: Double clone */
            }
        }
    }
    Ok(Expr::StructDef {
        name,
        fields,
        methods,
    })
}
