use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

use crate::{ast::Expr, binary_op::parse_expression, errors::ParserError};

pub fn parse_list(tokens: &mut Tokens) -> Result<Expr, ParserError> {
    let mut elements = Vec::new();
    loop {
        match tokens.peek() {
            Some(SpannedToken {
                token: Token::ListEnd,
                ..
            }) => {
                tokens.next();
                break;
            }
            Some(SpannedToken {
                token: Token::Comma,
                ..
            }) => {
                tokens.next();
            }

            Some(_) => {
                let element = parse_expression(tokens)?;
                elements.push(element);
            }
            None => {
                return Err(ParserError::UnexpectedEOF);
            }
        }
    }
    Ok(Expr::List(elements))
}
