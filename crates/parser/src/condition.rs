use crate::ast::{Else, Expr, IF};
use crate::{binary_op::parse_binary_op_expr, errors::ParserError, expressions::parse_expression};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

fn parse_block<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Vec<Expr<'a>>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let mut block = Vec::new();
    let mut indent = 0;

    while let Some(tok) = tokens.peek() {
        match tok {
            Token::Indent => {
                indent += 1;
                tokens.next();
            }
            Token::Dedent => {
                indent -= 1;
                tokens.next();
                if indent <= 0 {
                    break;
                }
            }
            Token::Newline => {
                indent = 0;
                tokens.next();
            }
            Token::Eof => break,
            _ => block.push(parse_expression(tokens)?),
        }
    }
    Ok(block)
}

pub fn parse_if_expr<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let condition = parse_binary_op_expr(tokens)?;
    let then_branch = parse_block(tokens)?;

    let mut else_if_branch = Vec::new();
    let mut else_branch = None;

    loop {
        match tokens.peek() {
            Some(Token::ElseIf) => {
                tokens.next();
                let cond = parse_binary_op_expr(tokens)?;
                let then_b = parse_block(tokens)?;
                else_if_branch.push(IF {
                    condition: Box::new(cond),
                    body: then_b,
                });
            }
            Some(Token::Else) => {
                tokens.next();
                let then_b = parse_block(tokens)?;
                else_branch = Some(Else { body: then_b });
            }
            _ => break,
        }
    }

    Ok(Expr::Condition {
        main: IF {
            condition: Box::new(condition),
            body: then_branch,
        },
        elif: else_if_branch,
        other: else_branch,
    })
}
