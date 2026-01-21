use crate::errors::ParserError;
use crate::{ast::Expr, expressions::parse_single_expr};
use tokenizer::iterator::{SpannedToken, Tokens};
use tokenizer::tokens::Token;

pub fn parse_binary_op_expr<'a>(
    tokens: &mut Tokens,
) -> Result<Expr<'a>, ParserError>
{
    match tokens.peek_nth(1) {
        Some(SpannedToken { token: Token::Op(_),.. }) => {}
        Some(_) => return parse_single_expr(tokens),
        None => return Err(ParserError::UnexpectedEOF),
    }

    parse_binary_op_with_precedence(tokens, 0)
}

fn parse_binary_op_with_precedence<'a>(
    tokens: &mut Tokens,
    min_precedence: u8,
) -> Result<Expr<'a>, ParserError>
{
    let mut left = parse_single_expr(tokens)?;
    loop {
        let op = match tokens.peek() {
            Some(Token::Operator(s)) => s,
            Some(Token::Newline) | Some(Token::Eof) | Some(Token::RParen) | None => {
                break;
            }
            Some(_) => break,
        };

    }

  /*   loop {
        let op = match tokens.peek() {
            Some(Token::Operator(s)) => s,
            Some(Token::Newline) | Some(Token::Eof) | Some(Token::RParen) | None => {
                break;
            }
            Some(_) => break,
        };

        let precedence = operator_precedence(op);

        if precedence < min_precedence {
            break;
        }

        tokens.next();

        let right = parse_binary_op_with_precedence(tokens, precedence + 1)?;

        left = Expr::BinaryOp {
            left: Box::new(left),
            right: Box::new(right),
            op,
            return_type: Type::Any,
        };
    }
 */
    Ok(left)
}

fn operator_precedence(op: &str) -> u8 {
    match op {
        "*" | "/" | "%" => 3,
        "+" | "-" => 2,
        "==" | "!=" | "<" | ">" | "<=" | ">=" => 1,
        _ => 0,
    }
}
