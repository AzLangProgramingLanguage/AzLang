use crate::{errors::ParserError, parsing_for::parse_single_expr_typed, typed_ast::TypedExpr};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{ast::Expr, expressions::parse_single_expr, helpers::expect_token};

/* TODO: Burası qısaldılmalıdır */
pub fn parse_loop<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let iterable = parse_single_expr(tokens)?;

    expect_token(tokens, Token::In)?;

    let var_name = match tokens.next() {
        Some(Token::Identifier(name)) => (*name).as_str(),
        Some(other) => return Err(ParserError::LoopVarNameNotFound(other.clone())),
        None => return Err(ParserError::LoopVarNameNotFound(Token::Eof)),
    };

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
                let expr = parse_single_expr(tokens)?;
                body.push(expr);

                while matches!(tokens.peek(), Some(Token::Semicolon | Token::Newline)) {
                    tokens.next();
                }
            }
        }
    }

    Ok(Expr::Loop {
        var_name,
        iterable: Box::new(iterable),
        body,
    })
}

pub fn parse_loop_typed<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let iterable = parse_single_expr_typed(tokens)?;

    expect_token(tokens, Token::In)?;

    let var_name = match tokens.next() {
        Some(Token::Identifier(name)) => (*name).as_str(),
        Some(other) => return Err(ParserError::LoopVarNameNotFound(other.clone())),
        None => return Err(ParserError::LoopVarNameNotFound(Token::Eof)),
    };

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
                let expr = parse_single_expr_typed(tokens)?;
                body.push(expr);

                while matches!(tokens.peek(), Some(Token::Semicolon | Token::Newline)) {
                    tokens.next();
                }
            }
        }
    }

    Ok(TypedExpr::Loop {
        var_name,
        iterable: Box::new(iterable),
        body,
    })
}
