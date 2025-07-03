use crate::{
    lexer::Token,
    parser::{Expr, Parser, expressions::parse_expression, statements::parse_statement},
};
pub fn parse_if_expr(parser: &mut Parser) -> Result<Expr, String> {
    let condition = parse_expression(parser, false)?;

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
                if let Some(stmt) = parse_statement(parser)? {
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
            let else_if_expr = parse_else_if_expr(parser)?;
            Some(vec![else_if_expr])
        }
        Some(Token::Else) => {
            parser.next(); // consume Else
            let else_expr = parse_else_expr(parser)?;
            Some(vec![else_expr]) // ❗ artıq `Expr::Else`-i dönürsən
        }
        _ => None,
    };

    Ok(Expr::If {
        condition: Box::new(condition),
        then_branch,
        else_branch: else_branch.unwrap_or_default(),
    })
}

pub fn parse_else_if_expr(parser: &mut Parser) -> Result<Expr, String> {
    let condition = parse_expression(parser, false)?;

    match parser.next() {
        Some(Token::Newline) => {}
        other => return Err(format!("Yeni sətir gözlənilirdi,dd  tapıldı: {:?}", other)),
    }

    match parser.next() {
        Some(Token::Indent) => {}
        other => return Err(format!("Girinti gözlənilirdi, tapıldı: {:?}", other)),
    }

    let mut then_branch = Vec::new();
    let mut indent_level = 1;

    loop {
        match parser.peek() {
            Some(Token::Dedent) => {
                indent_level -= 1;
                parser.next();
                if indent_level == 0 {
                    break;
                }
            }
            Some(Token::EOF) => {
                break;
            }
            Some(Token::Indent) => {
                indent_level += 1;
                parser.next();
            }
            Some(Token::Newline) => {
                parser.next();
            }
            Some(_) => {
                if let Some(stmt) = parse_statement(parser)? {
                    then_branch.push(stmt);
                }

                if let Some(token) = parser.peek() {
                    if token != &Token::Newline && token != &Token::Dedent {
                        let t = parser.next().unwrap();
                        return Err(format!("Yeni sətir gözlənilirdi, tapıldı: {:?}", t));
                    } else {
                        parser.next();
                    }
                }
            }
            None => break,
        }
    }

    Ok(Expr::ElseIf {
        condition: Box::new(condition),
        then_branch,
    })
}

pub fn parse_else_expr(parser: &mut Parser) -> Result<Expr, String> {
    match parser.next() {
        Some(Token::Newline) => {}
        other => return Err(format!("Yeni sətir gözlənilirdi,   tapıldı: {:?}", other)),
    }

    match parser.next() {
        Some(Token::Indent) => {}
        other => return Err(format!("Girinti gözlənilirdi,  tapıldı: {:?}", other)),
    }

    let mut then_branch = Vec::new();
    let mut indent_level = 1;

    loop {
        match parser.peek() {
            Some(Token::Dedent) => {
                indent_level -= 1;
                parser.next();
                if indent_level == 0 {
                    break;
                }
            }
            Some(Token::Indent) => {
                indent_level += 1;
                parser.next();
            }
            Some(Token::Newline) => {
                parser.next();
            }

            Some(Token::EOF) => {
                break;
            }
            Some(_) => {
                if let Some(stmt) = parse_statement(parser)? {
                    then_branch.push(stmt);
                }

                if let Some(token) = parser.peek() {
                    if token != &Token::Newline && token != &Token::Dedent && token != &Token::EOF {
                        let t = parser.next().unwrap();
                        return Err(format!("Yeni sətir gözlənilirdi, dd tapıldı: {:?}", t));
                    } else {
                        parser.next();
                    }
                }
            }
            None => break,
        }
    }

    Ok(Expr::Else { then_branch })
}
