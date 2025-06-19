use crate::context::{FunctionInfo, Parameter, TranspileContext};
use crate::parser::ast::Type;
use crate::parser::types::get_type;

use super::expressions::parse_expression;
use super::{Expr, Parser, Token};

pub fn parse_function_def(parser: &mut Parser, ctx: &mut TranspileContext) -> Result<Expr, String> {
    let name = match parser.next() {
        Some(Token::Identifier(name)) => name.clone(),
        other => return Err(format!("Funksiya adı gözlənilirdi, tapıldı: {:?}", other)),
    };

    if parser.next() != Some(&Token::LParen) {
        return Err("Funksiya parametr siyahısı '(' ilə başlamalıdır".to_string());
    }

    let mut parameters: Vec<Parameter> = Vec::new();

    loop {
        match parser.peek() {
            Some(Token::ConstantDecl) | Some(Token::MutableDecl) | Some(Token::Identifier(_)) => {
                let is_mutable = matches!(parser.peek(), Some(Token::MutableDecl));
                if is_mutable || matches!(parser.peek(), Some(Token::ConstantDecl)) {
                    parser.next(); // consume const or mut
                }

                let param_name = match parser.next() {
                    Some(Token::Identifier(name)) => name.clone(),
                    other => {
                        return Err(format!("Parametr adı gözlənilirdi, tapıldı: {:?}", other));
                    }
                };

                if parser.next() != Some(&Token::Colon) {
                    return Err("':' gözlənilirdi".to_string());
                }

                let param_type = match parser.next() {
                    Some(Token::TypeName(t)) => t.clone(),
                    other => {
                        return Err(format!("Parametr tipi gözlənilirdi, tapıldı: {:?}", other));
                    }
                };

                parameters.push(Parameter {
                    name: param_name,
                    typ: param_type,
                    is_mutable,
                });

                match parser.peek() {
                    Some(Token::Comma) => {
                        parser.next();
                    }
                    Some(Token::RParen) => {}
                    other => {
                        return Err(format!(
                            "Parametrlər arasında ',' və ya ')' gözlənilirdi, tapıldı: {:?}",
                            other
                        ));
                    }
                }
            }
            Some(Token::RParen) => break,
            other => {
                return Err(format!(
                    "Parametr adı və ya ')' gözlənilirdi, tapıldı: {:?}",
                    other
                ));
            }
        }
    }

    if parser.next() != Some(&Token::RParen) {
        return Err("')' gözlənilirdi".to_string());
    }

    // Return tipi
    let mut return_type = if parser.peek() == Some(&Token::Colon) {
        parser.next(); // consume `:`
        match parser.next() {
            Some(Token::TypeName(t)) => Some(t.clone()),
            other => {
                return Err(format!(
                    "Geri dönüş tipi gözlənilirdi, tapıldı: {:?}",
                    other
                ));
            }
        }
    } else {
        None
    };

    // Yeni sətir və girinti
    match parser.next() {
        Some(Token::Newline) => {}
        _ => return Err("Yeni sətir gözlənilirdi".to_string()),
    }

    match parser.next() {
        Some(Token::Indent) => {}
        _ => return Err("Girinti gözlənilirdi".to_string()),
    }

    let mut body = Vec::new();
    loop {
        match parser.peek() {
            Some(Token::End) | Some(Token::Dedent) => {
                parser.next(); // 'bitir' və ya `Dedent`
                break;
            }
            Some(Token::Newline) => {
                parser.next();
                continue;
            }
            Some(_) => {
                let expr = parse_expression(parser, true, ctx)?;
                body.push(expr);
                if matches!(parser.peek(), Some(Token::Semicolon)) {
                    parser.next();
                }
            }
            None => return Err("Funksiya gövdəsi bağlanmadı".to_string()),
        }
    }

    // Tip avtomatik çıxarılırsa
    if return_type.is_none() {
        return_type = infer_function_return_type(&body, &TranspileContext::new());
    }

    // ✅ Funksiya konteksə əlavə olunur
    ctx.declare_function(FunctionInfo {
        name: name.clone(),
        return_type: return_type.clone(),
        parameters: parameters.clone(),
        body: None,
        scope_level: ctx.scopes.len(),
        is_public: false,
    });

    Ok(Expr::FunctionDef {
        name,
        params: parameters,
        body,
        return_type,
    })
}

fn infer_function_return_type(body: &[Expr], ctx: &TranspileContext) -> Option<Type> {
    let mut return_types = vec![];

    for expr in body {
        if let Expr::Return(inner) = expr {
            if let Some(t) = get_type(inner, ctx) {
                return_types.push(t);
            }
        }
    }

    if return_types.is_empty() {
        Some(Type::Void)
    } else if return_types.iter().all(|t| t == &return_types[0]) {
        Some(return_types[0].clone())
    } else {
        Some(Type::Any)
    }
}
