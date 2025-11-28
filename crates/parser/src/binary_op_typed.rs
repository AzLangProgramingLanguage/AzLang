use crate::{errors::ParserError, parsing_for::parse_single_expr_typed, typed_ast::TypedExpr};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

pub fn parse_binary_op_expr_typed<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    min_prec: u8,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let mut left = parse_single_expr_typed(tokens)?;

    loop {
        let op_token = match tokens.peek() {
            Some(Token::Operator(op)) if op.as_str() != "." => {
                tokens.next();
                op.as_str()
            }
            _ => {
                break;
            }
        };

        let prec = get_precedence(op_token);
        if prec < min_prec {
            break;
        }

        let mut right = parse_single_expr_typed(tokens)?;

        loop {
            let next_prec = match tokens.peek_nth(1) {
                Some(Token::Operator(next_op)) => get_precedence(next_op),
                _ => 0,
            };
            if next_prec > prec {
                right = parse_binary_op_expr_typed(tokens, prec + 1)?;
            } else {
                break;
            }
        }

        if op_token == "=" {
            if let TypedExpr::VariableRef { name, .. } = left {
                left = TypedExpr::Assignment {
                    name,
                    value: Box::new(right),
                    symbol: None,
                };
            } else {
                return Err(ParserError::BinaryOpLeftNotExpected(op_token.to_string()));
            }
        } else {
            left = TypedExpr::BinaryOp {
                left: Box::new(left),
                op: op_token,
                right: Box::new(right),
            };
        }
    }
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
