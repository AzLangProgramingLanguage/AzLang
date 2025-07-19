use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;

use crate::{
    lexer::Token,
    parser::{
        ast::{Expr, MatchExpr},
        expression::parse_single_expr,
    },
};

pub fn parse_match<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    tokens.next();
    let target = Box::new(parse_single_expr(tokens)?);
    match tokens.peek() {
        Some(Token::Newline) => {
            tokens.next();
        }
        other => {
            return Err(eyre!("Match parsing xətası: {:?}", other));
        }
    }

    match tokens.next() {
        Some(Token::Indent) => {}

        other => {
            return Err(eyre!(
                "Match arms üçün girinti gözlənilirdi, tapıldı: {:?}",
                other
            ));
        }
    }
    let mut arms = Vec::new();

    while let Some(token) = tokens.peek() {
        match token {
            Token::StringLiteral(_) | Token::Number(_) | Token::Underscore => {
                let pattern = (*token).clone();
                tokens.next();

                match tokens.next() {
                    Some(Token::Arrow) => {}
                    other => {
                        return Err(eyre!(
                            "Match arm üçün '->' gözlənilirdi, tapıldı: {:?}",
                            other
                        ));
                    }
                }

                // Sağ tərəfdə ifadə gözlənilir
                let expr = parse_single_expr(tokens)?;

                arms.push((pattern, vec![expr]));

                // İfadədən sonra mütləq newline gəlməlidir
                match tokens.next() {
                    Some(Token::Newline) => {}
                    other => return Err(eyre!("Newline gözlənilirdi, tapıldı: {:?}", other)),
                }
            }
            Token::Dedent => {
                tokens.next();
                break;
            }

            Token::Newline => {
                tokens.next();
            }

            Token::Eof => break,
            unexpected => {
                return Err(eyre!("Gözlənilməz token match armında: {:?}", unexpected));
            }
        }
    }

    Ok(Expr::Match(Box::new(MatchExpr {
        target: target,
        arms: arms,
    })))
}
