use crate::{
    context::TranspileContext,
    lexer::Token,
    parser::{Expr, Parser, expressions::parse_expression, statements::parse_statement},
};
pub fn parse_loop(parser: &mut Parser, ctx: &mut TranspileContext) -> Result<Expr, String> {
    parser.next(); // consume `gəz`

    // Əvvəl iterable gəlməlidir (məs: ədədlər)
    let iterable = parse_expression(parser, false, ctx)?;

    // Sonra "içində" açar sözü
    if parser.next() != Some(&Token::In) {
        return Err("'içində' açar sözü gözlənilirdi".to_string());
    }

    // Sonra dəyişənin adı (məs: ədəd)
    let var_name = if let Some(Token::Identifier(name)) = parser.next() {
        name.clone()
    } else {
        return Err("Dəyişən adı gözlənilirdi".to_string());
    };

    // Qalan hissə dəyişmir
    match parser.next() {
        Some(Token::Newline) => {}
        other => return Err(format!("Yeni sətir gözlənilirdi, tapıldı: {:?}", other)),
    }

    match parser.next() {
        Some(Token::Indent) => {}
        other => return Err(format!("Girinti gözlənilirdi, tapıldı: {:?}", other)),
    }

    let mut body = Vec::new();
    loop {
        match parser.peek() {
            Some(Token::Dedent) | Some(Token::End) => {
                parser.next(); // consume `Dedent` or `Bitir`
                break;
            }
            Some(Token::Newline) => {
                parser.next();
            }
            Some(_) => {
                let stmt = parse_statement(parser, ctx)?;

                if let Some(stmt_expr) = stmt {
                    body.push(stmt_expr);
                } else {
                    break;
                }

                match parser.peek() {
                    Some(Token::End) => {}
                    Some(Token::Newline) => {
                        parser.next();
                    }

                    Some(Token::EOF) => {
                        break;
                    }

                    Some(_) => {}
                    None => {
                        break;
                    }
                }
            }
            None => {
                break;
            }
        }
    }

    Ok(Expr::Loop {
        var_name,
        iterable: Box::new(iterable),
        body,
    })
}
