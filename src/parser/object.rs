use crate::{
    lexer::Token,
    parser::{Expr, Parser, method::parse_method, types::parse_type},
};

pub fn parse_struct_def(parser: &mut Parser) -> Result<Expr, String> {
    let name = match parser.next() {
        Some(Token::Identifier(n)) => n.clone(),
        other => return Err(format!("Struktur adı gözlənilirdi, tapıldı: {:?}", other)),
    };

    parser.expect(&Token::Newline)?;

    let mut fields = Vec::new();
    let mut methods = Vec::new();

    while let Some(token) = parser.peek() {
        match token {
            Token::Identifier(_) => {
                let field_name = parser.next_identifier()?.clone();
                parser.expect(&Token::Colon)?;
                let field_type = parse_type(parser)?;
                fields.push((field_name, field_type));

                while matches!(parser.peek(), Some(Token::Newline)) {
                    parser.next();
                }
            }

            Token::Method => {
                let method = parse_method(parser)?;
                methods.push(method);

                while matches!(parser.peek(), Some(Token::Newline)) {
                    parser.next();
                }
            }
            Token::Indent => {
                parser.next();
            }

            Token::Dedent => {
                parser.next(); // bağlanış
                break;
            }
            Token::EOF => {
                break;
            }

            other => {
                return Err(format!(
                    "Obyekt daxilində sahə və ya method gözlənilirdi, tapıldı: {:?}",
                    other
                ));
            }
        }
    }

    Ok(Expr::StructDef {
        name,
        fields,
        methods,
    })
}
