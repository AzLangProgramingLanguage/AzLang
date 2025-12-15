use crate::{errors::ParserError, expressions::parse_expression};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{ast::Expr, expressions::parse_single_expr};

pub fn parse_binary_op_expr<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens.peek_nth(1) {
        Some(Token::Operator(_)) => {}
        Some(_) => return parse_single_expr(tokens),
        None => return Err(ParserError::UnexpectedEOF),
    }

    let mut variables: Vec<Expr<'a>> = Vec::new();
    let mut ops: Vec<&'a str> = Vec::new();
    loop {
        match tokens.peek() {
            Some(Token::Operator(s)) => {
                tokens.next();
                ops.push(s);
            }
            Some(Token::Newline) | Some(Token::Eof) | Some(Token::RParen) => {
                break;
            }

            Some(_) => {
                let expr = parse_single_expr(tokens)?;
                variables.push(expr);
            }
            None => return Err(ParserError::UnexpectedEOF),
        }
    }

    Ok(Expr::BinaryOp { variables, op: ops })
}
