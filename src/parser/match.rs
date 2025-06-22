use crate::{
    context::TranspileContext,
    lexer::Token,
    parser::{Expr, Parser, ast::MatchExpr, expressions::parse_expression},
};

pub fn parse_match_expr(parser: &mut Parser, ctx: &mut TranspileContext) -> Result<Expr, String> {
    parser.next(); // consume `uyğun`

    let target = Box::new(parse_expression(parser, false, ctx)?);

    match parser.next() {
        Some(Token::Newline) => {}
        other => {
            return Err(format!(
                "Match ifadəsindən sonra yeni sətir gözlənilirdi, tapıldı: {:?}",
                other
            ));
        }
    }

    match parser.next() {
        Some(Token::Indent) => {}
        other => {
            return Err(format!(
                "Match arms üçün girinti gözlənilirdi, tapıldı: {:?}",
                other
            ));
        }
    }

    let mut arms = Vec::new();
    loop {
        match parser.peek() {
            Some(Token::Identifier(_))
            | Some(Token::Number(_))
            | Some(Token::StringLiteral(_))
            | Some(Token::Underscore) => {
                let pattern_token = parser.next().cloned().unwrap();
                match parser.peek() {
                    Some(Token::Arrow) => {
                        parser.next(); // yeyilir
                    }
                    other => {
                        return Err(format!("'->' gözlənilirdi, tapıldı: {:?}", other));
                    }
                }

                let mut block = Vec::new();
                loop {
                    match parser.peek() {
                        Some(Token::Identifier(_))
                        | Some(Token::Number(_))
                        | Some(Token::StringLiteral(_))
                        | Some(Token::Underscore) => break,
                        Some(Token::Dedent) | Some(Token::End) | Some(Token::EOF) => break,
                        Some(_) => {
                            let expr = parse_expression(parser, false, ctx)?;
                            block.push(expr);
                            if let Some(Token::Newline) = parser.peek() {
                                parser.next();
                            }
                        }
                        None => break,
                    }
                }

                // Token birbaşa əlavə olunur (string çevrilmədən)
                match &pattern_token {
                    Token::Identifier(_)
                    | Token::Number(_)
                    | Token::Operator(_)
                    | Token::StringLiteral(_)
                    | Token::Underscore => {
                        arms.push((pattern_token, block));
                    }
                    other => {
                        return Err(format!(
                            "Match arm üçün qeyri-qəbul edilən token: {:?}",
                            other
                        ));
                    }
                }
            }

            Some(Token::Dedent) => {
                parser.next();
                break;
            }

            Some(Token::EOF) => break,

            Some(unexpected) => {
                return Err(format!(
                    "Match arm üçün gözlənilməz token: {:?}",
                    unexpected
                ));
            }

            None => return Err("Match ifadəsi bitmədən EOF gəldi".to_string()),
        }
    }

    Ok(Expr::Match(Box::new(MatchExpr { target, arms })))
}
