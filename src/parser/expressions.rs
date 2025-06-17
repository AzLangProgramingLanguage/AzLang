use crate::builtin::match_builtin;
use crate::parser::loop_expr::parse_loop;

use super::{Expr, Parser, Token};
use super::{call::parse_function_call, list::parse_list};

pub fn parse_expression(parser: &mut Parser, inside_function: bool) -> Result<Expr, String> {
    while parser.peek() == Some(&Token::Newline) {
        parser.next();
    }
    parse_binary_op_expression(parser, inside_function, 1)
}

fn parse_primary_expression(parser: &mut Parser, inside_function: bool) -> Result<Expr, String> {
    let mut expr = match parser.peek() {
        Some(Token::True) => {
            parser.next();
            Expr::Bool(true)
        }
        Some(Token::False) => {
            parser.next();
            Expr::Bool(false)
        }
        Some(Token::StringLiteral(_)) => {
            if let Some(Token::StringLiteral(s)) = parser.next() {
                Expr::String(s.clone())
            } else {
                return Err("StringLiteral gözlənilirdi".to_string());
            }
        }
        Some(Token::Number(_)) => {
            if let Some(Token::Number(val)) = parser.next() {
                Expr::FunctionCall {
                    name: "number".to_string(),
                    args: vec![Expr::String(val.to_string())],
                }
            } else {
                return Err("Ədəd gözlənilirdi".to_string());
            }
        }

        Some(Token::ListStart) => {
            parser.next(); // consume '['
            return parse_list(parser);
        }
        Some(Token::Loop) => {
            let loop_expr = parse_loop(parser)?;
            return Ok(loop_expr);
        }
        Some(Token::Identifier(_)) => {
            if let Some(Token::Identifier(id)) = parser.next().cloned() {
                let next_token = parser.peek();

                if let Some(Token::LParen) = next_token {
                    if match_builtin(&id).is_some() {
                        return parse_function_call(parser, &id);
                    }

                    parser.next(); // consume '('
                    let mut args = Vec::new();
                    loop {
                        match parser.peek() {
                            Some(Token::RParen) => {
                                parser.next(); // consume ')'
                                break;
                            }
                            Some(_) => {
                                let arg = parse_expression(parser, false)?;
                                args.push(arg);
                                if let Some(Token::Comma) = parser.peek() {
                                    parser.next();
                                }
                            }
                            None => return Err("Funksiya çağırışı bağlanmadı".to_string()),
                        }
                    }

                    return Ok(Expr::FunctionCall {
                        name: id.clone(),
                        args,
                    });
                } else {
                    let mut expr = Expr::VariableRef(id.clone());

                    loop {
                        match parser.peek() {
                            Some(Token::ListStart) => {
                                parser.next(); // consume '['
                                let index_expr = parse_expression(parser, inside_function)?;
                                match parser.peek() {
                                    Some(Token::ListEnd) => {
                                        parser.next(); // consume ']'
                                        expr = Expr::Index {
                                            target: Box::new(expr),
                                            index: Box::new(index_expr),
                                        };
                                    }
                                    other => {
                                        return Err(format!(
                                            "Bağlanış ']' gözlənilirdi, tapıldı: {:?}",
                                            other
                                        ));
                                    }
                                }
                            }
                            _ => break,
                        }
                    }

                    expr
                }
            } else {
                return Err("Tanıtıcı gözlənilirdi".to_string());
            }
        }

        Some(Token::Newline) => {
            parser.next();
            return parse_primary_expression(parser, inside_function);
        }

        Some(Token::Semicolon) => {
            parser.next();
            return parse_primary_expression(parser, inside_function);
        }

        Some(Token::RBrace) => {
            // Blok sonu: normal halda xəta deyil, sadəcə yuxarıya ötür
            parser.next();
            return parse_primary_expression(parser, inside_function);
        }
        other => return Err(format!("Dəyər gözlənilirdi, tapıldı: {:?}", other)),
    };

    // İndi expr üzərində loop ilə .method() çağırışlarını parse edək
    loop {
        match parser.peek() {
            Some(Token::Operator(op)) if op == "." => {
                parser.next(); // consume Operator(".")
                // Method adı gəlməlidir
                let method_name = if let Some(Token::Identifier(name)) = parser.next() {
                    name.clone()
                } else {
                    return Err("Method adı gözlənilirdi".to_string());
                };

                // Açıq mötərizə gəlməlidir
                if let Some(Token::LParen) = parser.next() {
                    let mut args = Vec::new();
                    loop {
                        match parser.peek() {
                            Some(Token::RParen) => {
                                parser.next(); // consume ')'
                                break;
                            }
                            Some(_) => {
                                let arg = parse_expression(parser, false)?;
                                args.push(arg);
                                if let Some(Token::Comma) = parser.peek() {
                                    parser.next();
                                }
                            }
                            None => return Err("Funksiya çağırışı bağlanmadı".to_string()),
                        }
                    }

                    expr = Expr::MethodCall {
                        target: Box::new(expr),
                        method: method_name,
                        args,
                    };
                } else {
                    return Err("Funksiya çağırışı üçün '(' gözlənilirdi".to_string());
                }
            }
            _ => break,
        }
    }

    Ok(expr)
}

pub fn parse_binary_op_expression(
    parser: &mut Parser,
    inside_function: bool,
    min_prec: u8,
) -> Result<Expr, String> {
    let mut left = parse_primary_expression(parser, inside_function)?;

    loop {
        let op_token = match parser.peek() {
            Some(Token::Operator(op)) => op.clone(),
            _ => break,
        };

        let prec = get_precedence(&op_token);
        if prec < min_prec {
            break;
        }

        parser.next(); // consume operator

        let right = parse_binary_op_expression(parser, inside_function, prec + 1)?;

        left = Expr::BinaryOp {
            left: Box::new(left),
            op: op_token,
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn get_precedence(op: &str) -> u8 {
    match op {
        "=" => 1, // <-- Əvvəl 0 idi, indi 1 oldu!
        "||" => 2,
        "&&" => 3,
        "==" | "!=" => 4,
        "<" | ">" | "<=" | ">=" => 5,
        "+" | "-" => 6,
        "*" | "/" => 7,
        _ => 0,
    }
}
