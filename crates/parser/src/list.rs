use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

use crate::{ast::Expr, binary_op::parse_expression, errors::ParserError};

pub fn parse_list<'a>(tokens: &mut Tokens) -> Result<Expr<'a>, ParserError> {
    let mut elements = Vec::new();
    loop {
        let elemen = parse_expression(tokens)?;
        elements.push(elemen);

        match tokens.next() {
            Some(SpannedToken {
                token: Token::ListEnd,
                span,
            }) => {
                break;
            }
            Some(SpannedToken {
                token: Token::Comma,
                span,
            }) => {}

            Some(other) => {
                return Err(ParserError::UnexpectedToken(other.span, other.token));
            }
            None => {
                return Err(ParserError::UnexpectedEOF);
            }
        }
    }
    Ok(Expr::List(elements))
}
