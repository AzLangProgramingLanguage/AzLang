use string_cache::Atom;
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

use crate::{ast::Expr, errors::ParserError, list::parse_list};

pub fn literals_parse(token: SpannedToken, tokens: &mut Tokens) -> Result<Expr, ParserError> {
    match token.token {
        Token::StringLiteral(s) => Ok(Expr::String(Atom::from(s))),
        Token::Number(num) => Ok(Expr::Number(num)),
        Token::Float(num) => Ok(Expr::Float(num)),
        Token::ListStart => parse_list(tokens),
        _ => Err(ParserError::UnexpectedToken(token.span, token.token)),
    }
}
