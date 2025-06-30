use crate::parser::builtin::match_builtin;

use super::expressions::parse_expression;
use super::{Expr, Parser, Token};
pub fn parse_function_call(parser: &mut Parser, name: &str) -> Result<Expr, String> {
    let args = parse_call_arguments(parser)?;

    if let Some((builtin, typ)) = match_builtin(name) {
        Ok(Expr::BuiltInCall {
            func: builtin,
            args,
            resolved_type: Some(typ),
        })
    } else {
        Ok(Expr::FunctionCall {
            name: name.to_string(),
            args,
            resolved_params: Vec::new(),
            return_type: None,
        })
    }
}

pub fn parse_call_arguments(parser: &mut Parser) -> Result<Vec<Expr>, String> {
    if parser.next() != Some(&Token::LParen) {
        return Err("Çağırış üçün '(' gözlənilirdi".to_string());
    }

    let mut args = Vec::new();

    match parser.peek() {
        Some(Token::RParen) => {
            parser.next(); // ()
        }
        Some(_) => loop {
            let arg = parse_expression(parser, false)?;
            args.push(arg);

            match parser.peek() {
                Some(Token::Comma) => {
                    parser.next();
                }
                Some(Token::RParen) => {
                    parser.next();
                    break;
                }
                _ => {
                    return Err("Çağırış üçüd ',' və ya ')' gözlənilirdi".to_string());
                }
            }
        },
        None => return Err("Funksiya/metod çağırışı bağlanmadı".to_string()),
    }

    Ok(args)
}
