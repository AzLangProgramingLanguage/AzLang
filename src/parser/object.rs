use crate::{
    context::TranspileContext,
    lexer::Token,
    parser::{Expr, Parser, types::parse_type},
};

pub fn parse_struct_def(parser: &mut Parser, ctx: &mut TranspileContext) -> Result<Expr, String> {
    let name = match parser.next() {
        Some(Token::Identifier(n)) => n.clone(),
        other => return Err(format!("Struktur adı gözlənilirdi, tapıldı: {:?}", other)),
    };

    match parser.next() {
        Some(Token::Newline) => {}
        other => return Err(format!("Yeni sətir gözlənilirdi, tapıldı: {:?}", other)),
    }

    match parser.next() {
        Some(Token::Indent) => {}
        other => {
            return Err(format!(
                "Girinti (indent) gözlənilirdi, tapıldı: {:?}",
                other
            ));
        }
    }

    let mut fields = Vec::new();

    while let Some(Token::Identifier(_)) = parser.peek() {
        let field_name = match parser.next() {
            Some(Token::Identifier(n)) => n.clone(),
            other => return Err(format!("Sahə adı gözlənilirdi, tapıldı: {:?}", other)),
        };

        parser.expect(&Token::Colon)?;

        let field_type = parse_type(parser)?;
        fields.push((field_name, field_type));

        while matches!(parser.peek(), Some(Token::Newline)) {
            parser.next();
        }
    }

    if let Some(Token::Dedent) = parser.peek() {
        parser.next();
    }

    // 🟢 Struct-u `ctx`-ə qeyd et
    /*     ctx.struct_defs.insert(name.clone(), fields.clone());
     */
    Ok(Expr::StructDef { name, fields })
}
