use crate::{
    errors::ParserError,
    parsing_for::parse_expression_typed,
    shared_ast::Type,
    typed_ast::{ParameterTyped, TypedExpr},
};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{
    ast::{Expr, Parameter},
    expressions::parse_expression,
    helpers::expect_token,
    types::parse_type,
};

fn parse_function_core<'a, I, Param, ExprOut>(
    tokens: &mut PeekMoreIterator<I>,
    param_builder: impl Fn(String, Type<'a>, bool) -> Param,
    expr_parser: impl Fn(&mut PeekMoreIterator<I>) -> Result<ExprOut, ParserError>,
    result_builder: impl Fn(&'a str, Vec<Param>, Vec<ExprOut>, Option<Type<'a>>) -> ExprOut,
) -> Result<ExprOut, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => (*name).as_str(),
        None => return Err(ParserError::UnexpectedEOF),
        Some(other) => return Err(ParserError::FunctionNameNotFound(other.clone())),
    };

    expect_token(tokens, Token::LParen)?;

    let mut params = Vec::new();

    while let Some(token) = tokens.peek() {
        match token {
            Token::ConstantDecl | Token::MutableDecl | Token::Identifier(_) => {
                let is_mutable = matches!(tokens.peek(), Some(Token::MutableDecl));
                if is_mutable || matches!(tokens.peek(), Some(Token::ConstantDecl)) {
                    tokens.next();
                }

                let param_name = match tokens.next() {
                    Some(Token::Identifier(s)) => (*s).as_str(),
                    other => {
                        return Err(ParserError::ParameterNameNotFound(other.unwrap().clone()));
                    }
                };

                let mut param_type = Type::Any;

                match tokens.peek() {
                    Some(Token::Comma) => {
                        tokens.next();
                    }
                    Some(Token::Colon) => {
                        tokens.next();
                        param_type = parse_type(tokens)?;
                    }
                    Some(Token::RParen) => break,
                    None => return Err(ParserError::UnexpectedEOF),
                    Some(other) => {
                        return Err(ParserError::ParameterNotExpected((*other).clone()));
                    }
                }

                params.push(param_builder(
                    param_name.to_string(),
                    param_type,
                    is_mutable,
                ));
            }
            Token::Comma => {
                tokens.next();
            }
            Token::RParen => break,
            other => return Err(ParserError::RParenNotFound((*other).clone())),
        }
    }

    expect_token(tokens, Token::RParen)?;
    expect_token(tokens, Token::Colon)?;

    let return_type = Some(parse_type(tokens)?);

    expect_token(tokens, Token::Newline)?;
    expect_token(tokens, Token::Indent)?;

    let mut body = Vec::new();
    while let Some(token) = tokens.peek() {
        match token {
            Token::Dedent => {
                tokens.next();
                break;
            }
            Token::Newline => {
                tokens.next();
            }
            Token::Eof => break,
            _ => {
                let expr = expr_parser(tokens)?;
                body.push(expr);
            }
        }
    }

    Ok(result_builder(name, params, body, return_type))
}
pub fn parse_function_def<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_function_core(
        tokens,
        |name, typ, is_mut| Parameter {
            name,
            typ,
            is_mutable: is_mut,
            is_pointer: false,
        },
        |toks| parse_expression(toks),
        |name, params, body, ret| Expr::FunctionDef {
            name,
            params,
            body,
            return_type: ret,
        },
    )
}
pub fn parse_function_def_typed<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_function_core(
        tokens,
        |name, typ, is_mut| ParameterTyped {
            name,
            typ,
            is_mutable: is_mut,
            is_pointer: false,
        },
        |toks| parse_expression_typed(toks),
        |name, params, body, ret| TypedExpr::FunctionDef {
            name,
            params,
            body,
            return_type: ret,
            transpiled_name: None,
            is_allocator: false,
        },
    )
}
