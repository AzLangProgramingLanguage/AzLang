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
                parser.next();
                let expr = parse_expression(parser, false, ctx)?;
                chunks.push(TemplateChunk::Expr(Box::new(expr)));

                match parser.next().cloned() {
                    Some(Token::InterpolationEnd) => {}
                    other => {
                        return Err(format!(
                            "Template interpolasiyası bağlanmalıdır: {:?}",
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
