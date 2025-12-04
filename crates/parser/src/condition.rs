use crate::ast::Expr;
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
    let condition = parse_binary_op_expr(tokens, 0)?;
    let then_branch = parse_block(tokens)?;

    let mut else_branch = Vec::new();

    loop {
        match tokens.peek() {
            Some(Token::ElseIf) => {
                tokens.next();
                let cond = parse_binary_op_expr(tokens, 0)?;
                let then_b = parse_block(tokens)?;
                else_branch.push(Expr::ElseIf {
                    condition: Box::new(cond),
                    then_branch: then_b,
                });
            }
            Some(Token::Else) => {
                tokens.next();
                let then_b = parse_block(tokens)?;
                else_branch.push(Expr::Else {
                    then_branch: then_b,
                });
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

pub fn parse_else_if_expr<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
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

pub fn parse_else_expr<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    tokens.next();
    let then_branch = parse_block(tokens)?;
    Ok(Expr::Else { then_branch })
}
