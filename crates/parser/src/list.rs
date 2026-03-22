use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

use crate::{ast::Expr, binary_op::parse_expression, errors::ParserError};

pub fn parse_list<'a>(tokens: &mut Tokens) -> Result<Expr, ParserError> {
    let mut elements = Vec::new();
    loop {
        match tokens.peek() {
            Some(SpannedToken {
                token: Token::ListEnd,
                span,
            }) => {
                tokens.next();
                break;
            }
            Some(SpannedToken {
                token: Token::Comma,
                span,
            }) => {
                tokens.next();
            }

            Some(other) => {
                let elemen = parse_expression(tokens)?;
                elements.push(elemen);
            }
            None => {
                return Err(ParserError::UnexpectedEOF);
            }
        }
    }
    Ok(Expr::List(elements))
}
