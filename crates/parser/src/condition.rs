use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

use crate::{
    ast::{Else, IF, Statement},
    binary_op::{parse_expression, parse_statement},
    errors::ParserError,
    helpers::expect_token,
};

fn parse_block<'a>(tokens: &mut Tokens) -> Result<Vec<Statement>, ParserError> {
    let mut block = Vec::new();
    let mut indent = 0;

    while let Some(tok) = tokens.next() {
        match tok.token {
            Token::Indent => {
                indent += 1;
            }
            Token::Dedent => {
                indent -= 1;
                if indent <= 0 {
                    break;
                }
            }
            Token::Newline => {
                indent = 0;
            }
            Token::Eof => break,
            _ => block.push(parse_statement(tokens)?),
        }
    }
    Ok(block)
}

pub fn parse_if_expr<'a>(tokens: &mut Tokens) -> Result<Statement, ParserError> {
    let condition = parse_expression(tokens)?;
    let then_branch = parse_block(tokens)?;

    let mut else_if_branch: Vec<IF> = Vec::new();
    let mut else_branch = None;

    loop {
        match tokens.peek() {
            Some(SpannedToken {
                token: Token::ElseIf,
                ..
            }) => {
                tokens.next();
                let cond = parse_expression(tokens)?;
                expect_token(tokens, Token::Colon)?;
                let then_b = parse_block(tokens)?;
                else_if_branch.push(IF {
                    condition: Box::new(cond),
                    body: then_b,
                });
            }
            Some(SpannedToken {
                token: Token::Else, ..
            }) => {
                tokens.next();
                expect_token(tokens, Token::Colon)?;
                let then_b = parse_block(tokens)?;
                else_branch = Some(Else { body: then_b });
            }
            _ => break,
        }
    }

    Ok(Statement::Condition {
        main: IF {
            condition: Box::new(condition),
            body: then_branch,
        },
        elif: else_if_branch,
        other: else_branch,
    })
}
