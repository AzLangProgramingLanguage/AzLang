use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

use crate::{
    ast::{Else, Expr, IF},
    binary_op::parse_expression,
    errors::ParserError,
};

fn parse_block<'a>(tokens: &mut Tokens) -> Result<Vec<Expr<'a>>, ParserError> {
    let mut block = Vec::new();
    let mut indent = 0;

    while let Some(tok) = tokens.peek() {
        match tok.token {
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

pub fn parse_if_expr<'a>(tokens: &mut Tokens) -> Result<Expr<'a>, ParserError> {
    let condition = parse_expression(tokens)?;
    let then_branch = parse_block(tokens)?;

    let mut else_if_branch = Vec::new();
    let mut else_branch = None;

    loop {
        match tokens.peek() {
            Some(SpannedToken {
                token: Token::ElseIf,
                span,
            }) => {
                tokens.next();
                let cond = parse_expression(tokens)?;
                let then_b = parse_block(tokens)?;
                else_if_branch.push(IF {
                    condition: Box::new(cond),
                    body: then_b,
                });
            }
            Some(SpannedToken {
                token: Token::Else,
                span,
            }) => {
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
