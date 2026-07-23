use crate::{
    ast::{Atom, Expr},
    binary_op::parse_expression,
    errors::ParserError,
    helpers::expect_token,
    shared_ast::Type,
};
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

pub fn parse_identifier(tokens: &mut Tokens, s: String) -> Result<Expr, ParserError> {
    match tokens.peek() {
        Some(SpannedToken {
            token: Token::ListStart,
            ..
        }) => {
            tokens.next();
            let index = parse_expression(tokens)?;
            expect_token(tokens, Token::ListEnd)?;
            Ok(Expr::Index {
                target: Box::new(Expr::VariableRef {
                    name: Atom::from(s.clone()),
                    symbol: None,
                }),
                index: Box::new(index),
                target_type: Type::Any,
            })
        }
        _ => Ok(Expr::VariableRef {
            name: Atom::from(s),
            symbol: None,
        }),
    }
}
