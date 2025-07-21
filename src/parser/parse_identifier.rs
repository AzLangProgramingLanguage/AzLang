use crate::{
    lexer::{Token, token},
    parser::{
        ast::{Expr, Type},
        expression::parse_single_expr,
        structs::parse_structs_init,
    },
};
use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;
use std::borrow::Cow;

pub fn parse_identifier<'a, I>(tokens: &mut PeekMoreIterator<I>, s: &'a str) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = Expr::VariableRef {
        name: Cow::Borrowed(s),
        symbol: None,
    };

    match tokens.nth(1) {
        Some(Token::ListStart) => {
            let index_expr =
                parse_single_expr(tokens).map_err(|e| eyre!("İndeks ifadəsi səhv: {}", e))?;
            tokens.next();
            if matches!(tokens.next(), Some(Token::ListEnd)) {
                Ok(Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(index_expr),
                    target_type: Type::Any,
                })
            } else {
                Err(eyre!("Siyahı düzgün bağlanılmadı: ']' gözlənilirdi"))
            }
        }
        Some(Token::LParen) => {
            tokens.next(); // '(' yey
            let mut args = Vec::new();

            loop {
                match tokens.peek() {
                    Some(Token::RParen) => {
                        tokens.next();
                        break;
                    }
                    None => break,
                    _ => {
                        // Arqumenti parse et
                        let arg = parse_single_expr(tokens)?;
                        args.push(arg);
                        tokens.next();
                        // Vergül varsa yey, yoxdursa dayan
                        // Düzəliş: Token növünü birbaşa yoxla
                        if let Some(Token::Comma) = tokens.peek() {
                            tokens.next(); // Vergülü yey
                        } else {
                            // Növbəti token ')' deyilsə xəta
                            if !matches!(tokens.peek(), Some(Token::RParen)) {
                                return Err(eyre!(
                                    "Arqument siyahısında ',' və ya ')' gözlənilirdi"
                                ));
                            }
                        }
                    }
                }
            }
            tokens.next();
            expr = Expr::Call {
                target: None,
                name: s,
                args,
                returned_type: None,
            };
            Ok(expr)
        }
        Some(Token::Dot) => {
            let field_or_method = match tokens.next() {
                Some(Token::Identifier(name)) => (*name).as_str(),
                _ => return Err(eyre!("Metod və ya sahə adı gözlənilirdi")),
            };

            match tokens.peek() {
                Some(Token::LParen) => {
                    tokens.next(); // consume '('
                    let mut args = Vec::new();

                    while let Some(token) = tokens.peek() {
                        match token {
                            Token::RParen => {
                                tokens.next(); // consume ')'
                                break;
                            }
                            Token::Comma => {
                                tokens.next();
                            }
                            _ => {
                                let arg = parse_single_expr(tokens)?;
                                args.push(arg);
                            }
                        }
                    }

                    expr = Expr::Call {
                        target: Some(Box::new(expr)),
                        name: field_or_method,
                        args,
                        returned_type: Some(Type::Any),
                    };
                }
                _ => {
                    expr = Expr::Index {
                        target: Box::new(expr),
                        index: Box::new(Expr::String(field_or_method)),
                        target_type: Type::Any,
                    };
                }
            }
            Ok(expr)
        }
        Some(Token::LBrace) => parse_structs_init(tokens, s),
        Some(_) => Ok(expr),
        None => Err(eyre!("İdentifikator sonrası gözlənilməz EOF")),
    }
}
