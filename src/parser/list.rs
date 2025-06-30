use super::expressions::parse_expression;
use super::{Expr, Parser, Token};

pub fn parse_list(parser: &mut Parser) -> Result<Expr, String> {
    let mut elements = Vec::new();

    loop {
        match parser.peek() {
            Some(Token::ListEnd) => {
                parser.next();
                break;
            }
            Some(_) => {
                let expr = parse_expression(parser, false)?;
                elements.push(expr);

                match parser.peek() {
                    Some(Token::Comma) => {
                        parser.next();
                    }
                    Some(Token::ListEnd) => continue,
                    other => {
                        return Err(format!(
                            "Siyahı üçün ',' və ya ']' gözlənilirdi, tapıldı: {:?}",
                            other
                        ));
                    }
                }
            }
            None => return Err("Siyahı bağlanmadı".to_string()),
        }
    }

    Ok(Expr::List(elements))
}
