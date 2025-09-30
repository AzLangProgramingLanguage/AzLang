use color_eyre::eyre::Result;
use peekmore::PeekMoreIterator;

use crate::{
    dd,
    lexer::Token,
    parser::{ast::Expr, expression::parse_expression, op_expr::parse_binary_op_expr},
};

pub fn parse_if_expr<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let condition = parse_binary_op_expr(tokens, 0)?; //Problem burada

    let then_branch = parse_block(tokens)?;
    let mut else_branch = Vec::new();
    while let Some(token) = tokens.peek() {
        match token {
            Token::ElseIf => {
                tokens.next();
                else_branch.push(parse_else_if_expr(tokens)?);
            }
            Token::Else => {
                tokens.next();
                else_branch.push(parse_else_expr(tokens)?);
            }
            _ => break,
        }
    }
    Ok(Expr::If {
        condition: Box::new(condition),
        then_branch,
        else_branch,
    })
}
pub fn parse_else_if_expr<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let condition = parse_binary_op_expr(tokens, 0)?;
    let then_branch = parse_block(tokens)?;
    Ok(Expr::ElseIf {
        condition: Box::new(condition),
        then_branch,
    })
}

pub fn parse_else_expr<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    tokens.next();
    let then_branch = parse_block(tokens)?;
    Ok(Expr::Else { then_branch })
}
fn parse_block<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Vec<Expr<'a>>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut block = Vec::new();
    let mut indent_level = 0;

    while let Some(token) = tokens.peek() {
        match token {
            Token::Indent => {
                indent_level += 1;
                tokens.next();
            }
            Token::Dedent => {
                indent_level -= 1;
                tokens.next();

                if indent_level <= 0 {
                    break;
                }
            }
            Token::Newline => {
                indent_level = 0;
                tokens.next();
            }
            Token::Eof => break,
            _ => {
                let expr = parse_expression(tokens)?;
                block.push(expr);
            }
        }
    }

    Ok(block)
}
