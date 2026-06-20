use crate::{
    ast::{Atom, Expr, Statement},
    errors::ParserError,
    expressions::parse_single_expr,
    helpers::expect_token,
};
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

pub fn parse_loop<'a>(tokens: &mut Tokens) -> Result<Statement, ParserError> {
    let iterable = parse_single_expr(tokens)?;

    expect_token(tokens, Token::In)?;

    let var_name = match tokens.next() {
        Some(SpannedToken {
            token: Token::Identifier(name),
            ..
        }) => name,
        Some(other) => return Err(ParserError::LoopVarNameNotFound(other.token)),
        None => return Err(ParserError::LoopVarNameNotFound(Token::Eof)),
    };

    expect_token(tokens, Token::Newline)?;
    expect_token(tokens, Token::Indent)?;

    let mut body = Vec::new();

    while let Some(token) = tokens.peek() {
        match token.token {
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
            }
        }
    }

    Ok(Statement::Loop {
        var_name: Atom::from(var_name),
        iterable: Box::new(iterable),
        body,
    })
}
