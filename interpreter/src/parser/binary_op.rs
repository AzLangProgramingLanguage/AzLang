use errors::ParserError;
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::parser::{ast::Expr, expressions::parse_single_expr};

/* FIXME Burası bərbatt*/

pub fn parse_binary_op_expr<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    min_prec: u8,
) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let mut left = parse_single_expr(tokens)?;

    loop {
        let op_token = match tokens.peek() {
            Some(Token::Operator(op)) if op.as_str() != "." => {
                tokens.next();
                op.to_string()
            }
            _ => {
                break;
            }
        };

        let prec = get_precedence(&op_token);
        if prec < min_prec {
            break;
        }

        let mut right = parse_single_expr(tokens)?;

        // Sağ tərəfi daha yüksək prioritetlə yenidən yoxla
        loop {
            let next_prec = match tokens.peek_nth(1) {
                Some(Token::Operator(next_op)) => get_precedence(next_op),
                _ => 0,
            };
            if next_prec > prec {
                right = parse_binary_op_expr(tokens, prec + 1)?;
            } else {
                break;
            }
        }

        if op_token == "=" {
            if let Expr::VariableRef { name, .. } = left {
                left = Expr::Assignment {
                    name,
                    value: Box::new(right),
                    symbol: None,
                };
            } else {
                return Err(ParserError::BinaryOpLeftNotExpected(op_token.to_string()));
            }
        } else {
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: op_token, /* TODO: Burasına baxx. */
                right: Box::new(right),
            };
        }
    }
    Ok(left)
}

pub fn get_precedence(op: &String) -> u8 {
    match op.as_str() {
        "=" => 1,
        "və" | "vəya" => 2,
        "==" | "!=" | "<" | "<=" | ">" | ">=" => 3,
        "+" | "-" => 4,
        "*" | "/" | "%" => 5,
        _ => 0,
    }
}
