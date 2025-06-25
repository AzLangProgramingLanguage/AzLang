use crate::{
    context::TranspileContext,
    lexer::Token,
    parser::{Expr, Parser, ast::TemplateChunk, expressions::parse_expression},
};

pub fn parse_template_string_expr(
    parser: &mut Parser,
    ctx: &mut TranspileContext,
) -> Result<Expr, String> {
    let mut chunks = Vec::new();

    loop {
        match parser.peek().cloned() {
            Some(Token::StringLiteral(s)) => {
                parser.next();
                chunks.push(TemplateChunk::Literal(s));
            }
            Some(Token::InterpolationStart) => {
                parser.next(); // ${ işarəsini keç

                // Xüsusi hal: Əgər birbaşa } gəlsə, boş interpolasiya kimi qəbul et
                if let Some(Token::InterpolationEnd) = parser.peek() {
                    parser.next();
                    chunks.push(TemplateChunk::Expr(Box::new(Expr::VariableRef {
                        name: "".to_string(),
                        symbol: None,
                    })));
                    continue;
                }

                // İfadəni parse et
                let expr = parse_expression(parser, false, ctx)?;
                chunks.push(TemplateChunk::Expr(Box::new(expr)));

                // } işarəsini yoxla
                match parser.next() {
                    Some(Token::InterpolationEnd) => {}
                    other => {
                        return Err(format!(
                            "Template interpolasiyası bağlanmalıdır,  gözlənilirdi: {:?}",
                            other
                        ));
                    }
                }
            }
            Some(Token::Backtick) => {
                parser.next(); // bağlanış `
                break;
            }
            Some(tok) => {
                return Err(format!("Template içində gözlənilməyən token: {:?}", tok));
            }
            None => return Err("Template string bitmədi".to_string()),
        }
    }

    Ok(Expr::TemplateString(chunks))
}
