use std::f32::consts::E;

use crate::context::TranspileContext;
use crate::parser::builtin::match_builtin;
use crate::parser::loop_expr::parse_loop;
use crate::parser::object::parse_struct_def;

use super::{Expr, Parser, Token};
use super::{call::parse_function_call, list::parse_list};

pub fn parse_expression(
    parser: &mut Parser,
    inside_function: bool,
    ctx: &mut TranspileContext,
) -> Result<Expr, String> {
    while parser.peek() == Some(&Token::Newline) {
        parser.next();
    }

    parse_binary_op_expression(parser, inside_function, 1, ctx)
}

fn parse_primary_expression(
    parser: &mut Parser,
    inside_function: bool,
    ctx: &mut TranspileContext,
) -> Result<Expr, String> {
    let mut expr = match parser.peek() {
        Some(Token::True) => {
            parser.next();
            Expr::Bool(true)
        }
        Some(Token::Return) => {
            parser.next(); // consume `qaytar`
            let expr = parse_expression(parser, true, ctx)?;
            Expr::Return(Box::new(expr))
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
        Some(Token::Number(val)) => {
            let val = val.clone();
            parser.next();
            Expr::Number(val)
        }

        Some(Token::ListStart) => {
            parser.next(); // consume '['
            return parse_list(parser, ctx);
        }
        Some(Token::Loop) => {
            let loop_expr = parse_loop(parser, ctx)?;
            return Ok(loop_expr);
        }

        Some(Token::Identifier(_)) => {
            if let Some(Token::Identifier(id)) = parser.next().cloned() {
                let next_token = parser.peek();

                let mut expr = match next_token {
                    Some(Token::LParen) => {
                        // Function call
                        parser.next(); // consume '('
                        let mut args = Vec::new();
                        loop {
                            match parser.peek() {
                                Some(Token::RParen) => {
                                    parser.next();
                                    break;
                                }
                                Some(_) => {
                                    let arg = parse_expression(parser, false, ctx)?;
                                    args.push(arg);
                                    if let Some(Token::Comma) = parser.peek() {
                                        parser.next();
                                    }
                                }
                                None => return Err("Funksiya çağırışı bağlanmadı".to_string()),
                            }
                        }
                        Expr::FunctionCall { name: id, args }
                    }

                    Some(Token::LBrace) => {
                        // Struct init
                        parser.next(); // consume '{'
                        let mut args = Vec::new();
                        loop {
                            match parser.peek() {
                                Some(Token::RBrace) => {
                                    parser.next();
                                    break;
                                }
                                Some(_) => {
                                    let arg = parse_expression(parser, false, ctx)?;
                                    args.push(arg);
                                    if let Some(Token::Comma) = parser.peek() {
                                        parser.next();
                                    }
                                }
                                None => return Err("Struct yaratma bağlanmadı".to_string()),
                            }
                        }
                        Expr::StructInit { name: id, args }
                    }

                    _ => {
                        // VariableRef və index
                        let mut expr = Expr::VariableRef(id.clone());

                        loop {
                            match parser.peek() {
                                Some(Token::ListStart) => {
                                    parser.next(); // consume '['
                                    let index_expr =
                                        parse_expression(parser, inside_function, ctx)?;
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
                };

                // Dot operator chain
                loop {
                    match parser.peek() {
                        Some(Token::Dot) => {
                            parser.next(); // consume '.'

                            let field_or_method =
                                if let Some(Token::Identifier(name)) = parser.next() {
                                    name.clone()
                                } else {
                                    return Err("Sahə və ya metod adı gözlənilirdi".to_string());
                                };

                            match parser.peek() {
                                Some(Token::LParen) => {
                                    // Method call
                                    parser.next(); // consume '('
                                    let mut args = Vec::new();
                                    loop {
                                        match parser.peek() {
                                            Some(Token::RParen) => {
                                                parser.next();
                                                break;
                                            }
                                            Some(_) => {
                                                let arg = parse_expression(parser, false, ctx)?;
                                                args.push(arg);
                                                if let Some(Token::Comma) = parser.peek() {
                                                    parser.next();
                                                }
                                            }
                                            None => {
                                                return Err("Metod çağırışı bağlanmadı".to_string());
                                            }
                                        }
                                    }

                                    expr = Expr::MethodCall {
                                        target: Box::new(expr),
                                        method: field_or_method,
                                        args,
                                    };
                                }
                                _ => {
                                    // Field access
                                    expr = Expr::FieldAccess {
                                        target: Box::new(expr),
                                        field: field_or_method,
                                    };
                                }
                            }
                        }
                        _ => break,
                    }
                }

                return Ok(expr);
            } else {
                return Err("Tanıtıcı gözlənilirdi".to_string());
            }
        }

        Some(Token::Newline) => {
            parser.next();
            return parse_primary_expression(parser, inside_function, ctx);
        }

        Some(Token::Semicolon) => {
            parser.next();
            return parse_primary_expression(parser, inside_function, ctx);
        }

        Some(Token::RBrace) => {
            // Blok sonu: normal halda xəta deyil, sadəcə yuxarıya ötür
            parser.next();
            return parse_primary_expression(parser, inside_function, ctx);
        }
        other => return Err(format!("Dəyər gözlənilirdi, tapıldı: {:?}", other)),
    };

    // İndi expr üzərində loop ilə .method() çağırışlarını parse edək
    loop {
        println!("Dot operator gözlənilirdi"); //İsleyir 

        match parser.peek() {
            Some(Token::Dot) => {
                parser.next(); // consume '.'
                println!("Dot operator tapıldı"); //İslemir.
                // .sonra Identifier olmalıdır
                let field_or_method = if let Some(Token::Identifier(name)) = parser.next() {
                    name.clone()
                } else {
                    return Err("Sahə və ya metod adı gözlənilirdi".to_string());
                };

                match parser.peek() {
                    Some(Token::LParen) => {
                        parser.next(); // consume '('
                        let mut args = Vec::new();
                        loop {
                            match parser.peek() {
                                Some(Token::RParen) => {
                                    parser.next();
                                    break;
                                }
                                Some(_) => {
                                    let arg = parse_expression(parser, false, ctx)?;
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
                            method: field_or_method,
                            args,
                        };
                    }
                    _ => {
                        expr = Expr::FieldAccess {
                            target: Box::new(expr),
                            field: field_or_method,
                        };
                    }
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
    ctx: &mut TranspileContext,
) -> Result<Expr, String> {
    let mut left = parse_primary_expression(parser, inside_function, ctx)?;

    loop {
        // 🛑 "." operatorunu burda parse ETMƏ!
        let op_token = match parser.peek() {
            Some(Token::Operator(op)) if op != "." => op.clone(),
            _ => break,
        };

        let prec = get_precedence(&op_token);
        if prec < min_prec {
            break;
        }

        parser.next(); // consume operator

        let right = parse_binary_op_expression(parser, inside_function, prec + 1, ctx)?;

        if op_token == "=" {
            if let Expr::VariableRef(name) = left {
                return Ok(Expr::Assignment {
                    name,
                    value: Box::new(right),
                });
            } else {
                return Err("Mənimsətmənin sol tərəfində dəyişən olmalıdır".to_string());
            }
        }

        left = Expr::BinaryOp {
            left: Box::new(left),
            op: op_token,
            right: Box::new(right),
        };
    }

    Ok(left)
}
pub fn get_precedence(op: &str) -> u8 {
    match op {
        "=" => 1,
        "və" | "vəya" => 2,
        "==" | "!=" | "<" | "<=" | ">" | ">=" => 3,
        "+" | "-" => 4,
        "*" | "/" | "%" => 5,
        _ => 0, // default
    }
}
