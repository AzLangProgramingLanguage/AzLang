use crate::{
    lexer::Token,
    parser::{Expr, Parser, ast::MatchExpr, expressions::parse_expression},
};

pub fn parse_match_expr(parser: &mut Parser) -> Result<Expr, String> {
    parser.next(); // consume `uyğun`

    let target = Box::new(parse_expression(parser, false)?);

    match parser.peek() {
        Some(Token::Newline) => {
            parser.next();
        }
        other => {
            return Err(format!(
                "Match ifadəsindən sonra yeni sətir gözlənilirdi, tapıldı: {:?}",
                other
            ));
        }
    }
    let next = parser.next();
    match next {
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
            Some(Token::StringLiteral(_))
            | Some(Token::Number(_))
            | Some(Token::Identifier(_))
            | Some(Token::Underscore) => {
                // ⚠️ Peek ikinci token: Arrow varmı?
                let pattern = parser.peek_n(1);
                if pattern != Some(&Token::Arrow) {
                    break; // pattern deyil
                }

                let pattern_token = parser.next().cloned().unwrap();
                parser.next(); // consume Arrow

                let mut block = Vec::new();

                let expr = parse_expression(parser, false)?;
                block.push(expr);

                if let Some(Token::Newline) = parser.peek() {
                    parser.next();
                }

                arms.push((pattern_token, block));
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

            None => {
                return Err("Match ifadəsi bitmədən EOF gəldi".to_string());
            }
        }
    }

    Ok(Expr::Match(Box::new(MatchExpr { target, arms })))
}
