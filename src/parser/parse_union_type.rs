use peekmore::PeekMoreIterator;

use crate::{
    lexer::Token,
    parser::{
        ast::{Expr, MethodType},
        helper::{expect_token, skip_newlines},
        method::parse_method,
        types::parse_type,
    },
    translations::parser_errors::ParserError,
};

pub fn parse_union_type<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => (*name).as_str(),
        other => return Err(ParserError::UnionNameNotFound(format!("{:?}", other))),
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
                    transpiled_name: None,
                    params: method_expr.1,
                    body: method_expr.2,
                    return_type: method_expr.3,
                    is_allocator: method_expr.4,
                });

                skip_newlines(tokens)?;
            }
            Token::Dedent => {
                tokens.next();
                break;
            }
            Token::Eof => break,
            other => {
                return Err(ParserError::UnionUnknownToken(format!("{:?}", other)));
            }
        }
    }
    Ok(Expr::UnionType {
        name,
        transpiled_name: None,
        fields,
        methods,
    })
}
