use crate::{
    binary_op_typed::parse_binary_op_expr_typed, errors::ParserError,
    parsing_for::parse_expression_typed, typed_ast::TypedExpr,
};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{ast::Expr, binary_op::parse_binary_op_expr, expressions::parse_expression};

fn parse_block_core<'a, I, ExprOut>(
    tokens: &mut PeekMoreIterator<I>,
    expr_parser: impl Fn(&mut PeekMoreIterator<I>) -> Result<ExprOut, ParserError>,
) -> Result<Vec<ExprOut>, ParserError>
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
                let expr = expr_parser(tokens)?;
                block.push(expr);
            }
        }
    }

    Ok(block)
}
pub fn parse_if_expr<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_if_core(
        tokens,
        |t, p| parse_binary_op_expr(t, p),
        |t| parse_expression(t),
        |cond, then_b, else_b| Expr::If {
            condition: Box::new(cond),
            then_branch: then_b,
            else_branch: else_b,
        },
        |cond, then_b| Expr::ElseIf {
            condition: Box::new(cond),
            then_branch: then_b,
        },
        |then_b| Expr::Else {
            then_branch: then_b,
        },
    )
}
fn parse_if_core<'a, I, ExprOut>(
    tokens: &mut PeekMoreIterator<I>,
    binop_parser: impl Fn(&mut PeekMoreIterator<I>, u8) -> Result<ExprOut, ParserError>,
    expr_parser: impl Fn(&mut PeekMoreIterator<I>) -> Result<ExprOut, ParserError>,
    make_if: impl Fn(ExprOut, Vec<ExprOut>, Vec<ExprOut>) -> ExprOut,
    make_elseif: impl Fn(ExprOut, Vec<ExprOut>) -> ExprOut,
    make_else: impl Fn(Vec<ExprOut>) -> ExprOut,
) -> Result<ExprOut, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let condition = binop_parser(tokens, 0)?;
    let then_branch = parse_block_core(tokens, &expr_parser)?;

    let mut else_branch = Vec::new();
    while let Some(token) = tokens.peek() {
        match token {
            Token::ElseIf => {
                tokens.next();
                let condition = binop_parser(tokens, 0)?;
                let then_b = parse_block_core(tokens, &expr_parser)?;
                else_branch.push(make_elseif(condition, then_b));
            }
            Token::Else => {
                tokens.next();
                let then_b = parse_block_core(tokens, &expr_parser)?;
                else_branch.push(make_else(then_b));
            }
            _ => break,
        }
    }

    Ok(make_if(condition, then_branch, else_branch))
}

pub fn parse_if_expr_typed<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_if_core(
        tokens,
        |t, p| parse_binary_op_expr_typed(t, p),
        |t| parse_expression_typed(t),
        |cond, then_b, else_b| TypedExpr::If {
            condition: Box::new(cond),
            then_branch: then_b,
            else_branch: else_b,
        },
        |cond, then_b| TypedExpr::ElseIf {
            condition: Box::new(cond),
            then_branch: then_b,
        },
        |then_b| TypedExpr::Else {
            then_branch: then_b,
        },
    )
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

pub fn parse_else_if_expr_typed<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let condition = parse_binary_op_expr_typed(tokens, 0)?;
    let then_branch = parse_block_core(tokens, |t| parse_expression_typed(t))?;
    Ok(TypedExpr::ElseIf {
        condition: Box::new(condition),
        then_branch,
    })
}

pub fn parse_else_expr_typed<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    tokens.next();
    let then_branch = parse_block_core(tokens, |t| parse_expression_typed(t))?;
    Ok(TypedExpr::Else { then_branch })
}

pub fn parse_else_expr<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    tokens.next();
    let then_branch = parse_block(tokens)?;
    Ok(Expr::Else { then_branch })
}
fn parse_block<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Vec<Expr<'a>>, ParserError>
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
