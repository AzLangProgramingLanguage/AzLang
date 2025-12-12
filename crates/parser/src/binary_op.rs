use crate::{errors::ParserError, expressions::parse_expression};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{ast::Expr, expressions::parse_single_expr};

pub fn parse_binary_op_expr<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    min_prec: u8,
) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens.peek_nth(1) {
        Some(Token::Operator(s)) => {}
        Some(other) => return parse_single_expr(tokens),
        None => return Err(ParserError::UnexpectedEOF),
    }

    let mut variables: Vec<Expr<'a>> = Vec::new();
    let mut ops: Vec<&'a str> = Vec::new();
    loop {
        println!("Token {:?}", tokens.peek());
        match tokens.next() {
            Some(Token::Operator(s)) => {
                ops.push(s);
            }
            Some(Token::Newline) | Some(Token::Eof) => {
                break;
            }

            Some(other) => {
                println!("Error  {:?}", other);
                matches!(Token::MutableDecl, Token::MutableDecl);
                let token = parse_single_expr(tokens)?;
                variables.push(token);
            }

            None => return Err(ParserError::UnexpectedEOF),
        }
    }
    println!("{:?}", variables);

    std::process::exit(1);
    Ok(Expr::BinaryOp { variables, op: ops })
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
