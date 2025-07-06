use super::{Expr, Parser, Token};
use crate::Parameter;
use crate::parser::ast::Type;
use crate::parser::statements::parse_statement;

pub fn parse_function_def(parser: &mut Parser) -> Result<Expr, String> {
    let name = match parser.next() {
        Some(Token::Identifier(name)) => name.clone(),
        other => return Err(format!("Funksiya adı gözlənilirdi, tapıldı: {:?}", other)),
    };

    let parent = parser.current_function.clone();
    let prev_function = parser.current_function.clone();
    parser.current_function = Some(name.clone());

    if parser.next() != Some(&Token::LParen) {
        return Err("Funksiya parametr siyahısı '(' ilə başlamalıdır".to_string());
    }

    let mut parameters: Vec<Parameter> = Vec::new();

    loop {
        match parser.peek() {
            Some(Token::ConstantDecl) | Some(Token::MutableDecl) | Some(Token::Identifier(_)) => {
                // 1️⃣ Mutability ayarla (default: sabit/const)
                let is_mutable = match parser.peek() {
                    Some(Token::MutableDecl) => {
                        parser.next();
                        true
                    }
                    Some(Token::ConstantDecl) => {
                        parser.next();
                        false
                    }
                    _ => false, // İf nothing specified → default: const
                };

                // 2️⃣ Adı götür
                let param_name = match parser.next() {
                    Some(Token::Identifier(name)) => name.clone(),
                    other => {
                        return Err(format!("Parametr adı gözlənilirdi, tapıldı: {:?}", other));
                    }
                };

                // 3️⃣ Tip varsa götür, yoxdursa default `Any`
                let param_type = if parser.peek() == Some(&Token::Colon) {
                    parser.next(); // consume ':'
                    match parser.next() {
                        Some(Token::TypeName(t)) => t.clone(),
                        other => {
                            return Err(format!(
                                "Parametr tipi gözlənilirdi, tapıldı: {:?}",
                                other
                            ));
                        }
                    }
                } else {
                    Type::Any // 🔥 Avtomatik tip təyin (later validator istifadə edəcək)
                };

                // 4️⃣ Parametri əlavə et
                parameters.push(Parameter {
                    name: param_name,
                    typ: param_type,
                    is_mutable,
                    is_pointer: false,
                });

                // 5️⃣ ',' varsa davam et, yoxsa break et
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
    let return_type = if parser.peek() == Some(&Token::Colon) {
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

    let _ = parser.expect(&Token::Indent);

    let mut body = Vec::new();
    loop {
        match parser.peek() {
            Some(Token::End) | Some(Token::Dedent) | Some(Token::EOF) => {
                parser.next(); // 'bitir' və ya `Dedent`
                break;
            }
            Some(Token::Newline) => {
                parser.next();
                continue;
            }
            Some(_) => {
                if let Some(stmt) = parse_statement(parser)? {
                    body.push(stmt);
                }
                if matches!(parser.peek(), Some(Token::Semicolon)) {
                    parser.next();
                }
            }
            None => return Err("Funksiya gövdəsi bağlanmadı".to_string()),
        }
    }

    // Tip avtomatik çıxarılırsa

    parser.current_function = prev_function;

    Ok(Expr::FunctionDef {
        name,
        params: parameters,
        body,
        return_type,
        parent,
    })
}
