use crate::{
    lexer::Token,
    parser::{Expr, Parser, ast::EnumDecl},
};

pub fn parse_enum_decl(parser: &mut Parser) -> Result<Expr, String> {
    parser.next(); // consume `tip`

    let name = match parser.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return Err("tip-dən sonra ad gözlənilirdi".to_string()),
    };

    // Variantları toplayırıq
    match parser.next() {
        Some(Token::Newline) => {}
        _ => return Err("Yeni sətir gözlənilirdi".to_string()),
    }

    let mut variants = Vec::new();

    loop {
        match parser.peek() {
            Some(Token::Identifier(name)) => {
                variants.push(name.clone());
                parser.next(); // consume name
            }
            Some(Token::Newline) => {
                parser.next(); // boş sətir, keç
            }
            Some(Token::Dedent) | Some(Token::End) => {
                parser.next(); // block bitdi
                break;
            }
            Some(Token::Indent) => {
                parser.next();
            } //TODO: Buna Baxarsan
            Some(Token::EOF) => break,
            Some(unexpected) => {
                return Err(format!(
                    "Enum variantında gözlənilməz token: {:?}",
                    unexpected
                ));
            }
            None => {
                return Err("Enum tərifi gözlənilmədən bitdi".to_string());
            }
        }
    }

    Ok(Expr::EnumDecl(EnumDecl { name, variants }))
}
