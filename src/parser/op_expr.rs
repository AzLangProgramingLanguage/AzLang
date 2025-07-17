use std::iter::Peekable;

use color_eyre::eyre::{Result, eyre};

use crate::{
    lexer::Token,
    parser::{ast::Expr, expression::parse_single_expr},
};

pub fn parse_binary_op_expr<'a, I>(tokens: &mut Peekable<I>, min_prec: u8) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut left = parse_single_expr(tokens)?;
    tokens.next();
    loop {
        let op_token = match tokens.peek() {
            Some(Token::Operator(op)) if op.as_str() != "." => op.as_str(),
            _ => break,
        };
        let prec = get_precedence(op_token);
        if prec < min_prec {
            break;
        }
        tokens.next();
        let mut right = parse_single_expr(tokens)?;

        loop {
            let next_prec = match tokens.peek() {
                Some(Token::Operator(next_op)) => get_precedence(next_op.as_str()),
                _ => 0,
            };
            if next_prec > prec {
                right = parse_binary_op_expr(tokens, prec + 1)?;
            } else {
                break;
            }
        }

        if op_token == "=" {
            if let Expr::VariableRef { name, symbol: _ } = left {
                left = Expr::Assignment {
                    name,
                    value: Box::new(right),
                    symbol: None,
                };
            } else {
                return Err(eyre!("Sol tərəfdə dəyişən gözlənilirdi"));
            }
        } else {
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: op_token,
                right: Box::new(right),
            };
        }
    }
    tokens.next();
    Ok(left)
}
pub fn get_precedence(op: &str) -> u8 {
    match op {
        "=" => 1,
        "və" | "vəya" => 2,
        "==" | "!=" | "<" | "<=" | ">" | ">=" => 3,
        "+" | "-" => 4,
        "*" | "/" | "%" => 5,
        _ => 0,
    }
}
