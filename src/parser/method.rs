use crate::{
    Parameter,
    lexer::Token,
    parser::{Expr, Parser, ast::Type, expressions::parse_expression},
};

pub fn parse_method(
    parser: &mut Parser,
) -> Result<(String, Vec<Parameter>, Vec<Expr>, Option<Type>), String> {
    parser.expect(&Token::Method)?;

    let name = parser.next_identifier()?.clone();

    parser.expect(&Token::LParen)?;
    parser.expect(&Token::RParen)?; // Hələlik heç bir parametr, self avtomatikdir

    // Return tipi varsa oxu
    let return_type = if parser.peek() == Some(&Token::Colon) {
        parser.next(); // ':'
        match parser.next() {
            Some(Token::TypeName(t)) => Some(t.clone()),
            other => {
                return Err(format!(
                    "Geri dönüş tipi gözlənilirdi, tapıldı: {:?}",
                    other
                ));
            }
        }
    } else {
        None
    };

    parser.expect(&Token::Newline)?;
    parser.expect(&Token::Indent)?;

    let mut body = Vec::new();
    while let Some(token) = parser.peek() {
        match token {
            Token::Dedent => {
                parser.next(); // block bağlanışı
                break;
            }
            Token::Newline => {
                parser.next();
            }
            Token::EOF => {
                parser.next();
                break;
            }
            _ => {
                let expr = parse_expression(parser, true)?;
                body.push(expr);
                if matches!(parser.peek(), Some(Token::Semicolon)) {
                    parser.next();
                }
            }
        }
    }

    // Parametr olaraq self avtomatik əlavə olunur
    let params = vec![Parameter {
        name: "self".to_string(),
        typ: Type::Any,
        is_mutable: false,
        is_pointer: false,
    }];

    Ok((name, params, body, return_type))
}
