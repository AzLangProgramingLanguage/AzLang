use crate::{
    lexer::Token,
    parser::{
        ast::{Expr, Type},
        expression::parse_single_expr,
        structs::parse_structs_init,
    },
};
use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;
use std::borrow::Cow;

use super::expression::parse_expression;

pub fn parse_identifier<'a, I>(tokens: &mut PeekMoreIterator<I>, s: &'a str) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = Expr::VariableRef {
        name: Cow::Borrowed(s),
        transpiled_name: None,
        symbol: None,
    };

    match tokens.peek() {
        Some(Token::ListStart) => {
            tokens.next();
            let index_expr =
                parse_single_expr(tokens).map_err(|e| eyre!("İndeks ifadəsi səhv: {}", e))?;
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
            tokens.next();
            let mut args = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Token::RParen) => {
                        tokens.next();
                        break;
                    }
                    None => break,
                    _ => {
                        let arg = parse_expression(tokens)?;
                        args.push(arg);

                        if let Some(Token::Comma) = tokens.peek() {
                            tokens.next();
                        } else {
                            if !matches!(tokens.peek(), Some(Token::RParen)) {
                                return Err(eyre!(
                                    "Arqument siyahısında ',' və ya ')' gözlənilirdi"
                                ));
                            }
                        }
                    }
                }
            }
            expr = Expr::Call {
                target: None,
                name: s,
                args,
                returned_type: None,
                is_allocator: false,
                transpiled_name: None,
            };
            Ok(expr)
        }
        Some(Token::Operator(op)) if op == "=" => {
            tokens.next();
            let value = parse_expression(tokens)?;
            /* TODO: Buraya baxarsan */
            /*   match value {
                Expr::String(s, _) => {
                    value = Expr::StructInit {
                        name: Cow::Borrowed("Yazı"),
                        transpiled_name: Some(Cow::Borrowed("azlangYazi")),
                        args: vec![("Mut", value)],
                    }
                }
                _ => {}
            } */
            Ok(Expr::Assignment {
                name: s.into(),
                value: Box::new(value),
                symbol: None,
            })
        }
        Some(Token::Dot) => {
            tokens.next();
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
                        is_allocator: false,
                        transpiled_name: None,
                    };
                }
                _ => {
                    expr = Expr::Index {
                        target: Box::new(expr),
                        index: Box::new(Expr::String(field_or_method, false)),
                        target_type: Type::Any,
                    };
                }
            }
            Ok(expr)
        }
        Some(Token::LBrace) => {
            tokens.next();
            parse_structs_init(tokens, Cow::Borrowed(s))
        }
        Some(_) => Ok(expr),
        None => Err(eyre!("İdentifikator sonrası gözlənilməz EOF")),
    }
}
