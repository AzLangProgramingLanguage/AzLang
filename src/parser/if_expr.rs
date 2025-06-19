use crate::{
    context::TranspileContext,
    lexer::Token,
    parser::{Expr, Parser, expressions::parse_expression, statements::parse_statement},
};
pub fn parse_if_expr(parser: &mut Parser, ctx: &mut TranspileContext) -> Result<Expr, String> {
    let condition = parse_expression(parser, false, ctx)?;

    // Expect newline after condition
    match parser.next() {
        Some(Token::Newline) => {}

        other => return Err(format!("Yeni sətir gözlənilirdi, tapıldı: {:?}", other)),
    }

    // Expect indent for the if block
    match parser.next() {
        Some(Token::Indent) => {}
        other => return Err(format!("Girinti gözlənilirdi, tapıldı: {:?}", other)),
    }

    let mut then_branch = Vec::new();
    let mut current_indent_level = 1;

    loop {
        match parser.peek() {
            Some(Token::Dedent) => {
                current_indent_level -= 1;
                parser.next(); // consume dedent

                if current_indent_level == 0 {
                    // Consume ALL consecutive dedents at base level
                    while matches!(parser.peek(), Some(Token::Dedent)) {
                        parser.next();
                    }
                    break;
                }
            }
            Some(Token::Indent) => {
                current_indent_level += 1;
                parser.next();
            }
            Some(Token::Newline) => {
                parser.next();
            }
            Some(Token::EOF) => {
                break;
            }
            Some(_) => {
                if let Some(stmt) = parse_statement(parser, ctx)? {
                    then_branch.push(stmt);
                }

                // Handle optional newline after statement
                match parser.peek() {
                    Some(Token::Dedent) | None => {}
                    _ => {
                        if let Some(token) = parser.next() {
                            if token != &Token::Newline {
                                return Err(format!(
                                    "Yeni sətir gözlənilirdi, tapıldı: {:?}",
                                    token
                                ));
                            }
                        }
                    }
                }
            }
            None => break,
        }
    }

    // Now check for else/elseif
    let else_branch = match parser.peek() {
        Some(Token::ElseIf) => {
            parser.next(); // consume ElseIf
            let else_if_expr = parse_if_expr(parser, ctx)?;
            // Represent elseif as an If expression in the else branch
            Some(vec![else_if_expr])
        }
        Some(Token::Else) => {
            parser.next(); // consume Else

            // Expect newline after else
            match parser.next() {
                Some(Token::Newline) => {}
                other => return Err(format!("Yeni sətir gözlənilirdi, tapıldı: {:?}", other)),
            }

            // Expect indent for else block
            match parser.next() {
                Some(Token::Indent) => {}
                other => return Err(format!("Girinti gözlənilirdi, tapıldı: {:?}", other)),
            }

            let mut else_branch = Vec::new();
            let mut current_indent_level = 1;

            loop {
                match parser.peek() {
                    Some(Token::Dedent) => {
                        current_indent_level -= 1;
                        parser.next();
                        if current_indent_level == 0 {
                            break;
                        }
                    }
                    Some(Token::Indent) => {
                        current_indent_level += 1;
                        parser.next();
                    }
                    Some(Token::Newline) => {
                        parser.next();
                    }
                    Some(_) => {
                        if let Some(stmt) = parse_statement(parser, ctx)? {
                            else_branch.push(stmt);
                        }

                        match parser.peek() {
                            Some(Token::Dedent) | None => {}
                            _ => {
                                if let Some(token) = parser.next() {
                                    if token != &Token::Newline {
                                        return Err(format!(
                                            "Yeni sətir gözlənilirdi, tapıldı: {:?}",
                                            token
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    None => break,
                }
            }

            Some(else_branch)
        }
        _ => None,
    };

    Ok(Expr::If {
        condition: Box::new(condition),
        then_branch,
        else_branch,
    })
}
