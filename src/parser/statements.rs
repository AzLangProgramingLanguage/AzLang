use crate::{
    context::TranspileContext,
    parser::{
        function::parse_function_def,
        if_expr::{parse_else_expr, parse_else_if_expr, parse_if_expr},
        object::parse_struct_def,
        types::get_type,
    },
};

use super::{Expr, Parser, Token}; // Token və Expr-i super (parser/mod.rs) vasitəsilə import edirik

// Dəyişən elanlarını emal edir (dəyişən a:ədəd = 10; kimi)
pub fn parse_variable_declaration(
    parser: &mut Parser,
    kind: &str,
    ctx: &mut TranspileContext,
) -> Result<Option<Expr>, String> {
    let name = match parser.next() {
        Some(Token::Identifier(name)) => name.clone(),
        other => {
            return Err(format!(
                "{} üçün dəyişən adı gözlənilirdi, tapıldı: {:?}",
                kind, other
            ));
        }
    };
    if parser.declared_variables.contains(&name) {
        return Err(format!("Dəyişən '{}' artıq əvvəl təyin olunub", name));
    }
    parser.declared_variables.insert(name.clone());

    parser.used_variables.insert(name.clone());
    let mut typ = match parser.peek() {
        Some(Token::Colon) => {
            parser.next(); // ':' consume
            Some(parser.parse_type()?)
        }
        _ => None,
    };

    match parser.next() {
        Some(Token::Operator(op)) if op == "=" => {}
        other => {
            return Err(format!(
                "{} üçün '=' operatoru gözlənilirdi, tapıldı: {:?}",
                kind, other
            ));
        }
    }

    let value_expr = parser.parse_expression(ctx)?; // Parser metodunu çağırırıq

    // ✨ Əgər istifadəçi tip göstərməyibsə, biz `value_expr`-dən onu təxminləyirik
    if typ.is_none() {
        if let Some(inferred_type) = get_type(&value_expr, &ctx) {
            typ = Some(inferred_type);
        }
    }

    match kind {
        "mutable_decl" => Ok(Some(Expr::MutableDecl {
            name,
            typ,
            value: Box::new(value_expr),
        })),
        "constant_decl" => Ok(Some(Expr::ConstantDecl {
            name,
            typ,
            value: Box::new(value_expr),
        })),
        _ => unreachable!(),
    }
}

// Bu, proqramımızdakı "sətr"lər, yəni ifadələrdir

pub fn parse_statement(
    parser: &mut Parser,
    ctx: &mut TranspileContext,
) -> Result<Option<Expr>, String> {
    while let Some(token) = parser.peek() {
        match token {
            Token::Newline | Token::Semicolon => {
                parser.next(); // Boş sətirləri və nöqtəli vergülləri keç
            }

            Token::Break => {
                parser.next(); // consume `Break`
                return Ok(Some(Expr::Break));
            }
            Token::Continue => {
                parser.next(); // consume `Continue`
                return Ok(Some(Expr::Continue));
            }
            Token::EOF => {
                break;
            }
            _ => break,
        }
    }

    match parser.peek() {
        Some(Token::MutableDecl) | Some(Token::ConstantDecl) => {
            let kind = parser.next().unwrap();
            let kind_str = match kind {
                Token::MutableDecl => "mutable_decl",
                Token::ConstantDecl => "constant_decl",
                _ => unreachable!(),
            };
            parse_variable_declaration(parser, kind_str, ctx)
        }
        Some(Token::Conditional) => {
            parser.next();
            parse_if_expr(parser, ctx).map(Some)
        }

        Some(Token::ElseIf) => {
            parser.next();
            parse_else_if_expr(parser, ctx).map(Some)
        }
        Some(Token::Else) => {
            parser.next();
            parse_else_expr(parser, ctx).map(Some)
        }

        Some(Token::FunctionDef) => {
            parser.next(); // consume FunctionDef
            parse_function_def(parser, ctx).map(Some)
        }
        Some(Token::Object) => {
            parser.next();
            let struct_def = parse_struct_def(parser, ctx)?;
            Ok(Some(struct_def))
        }
        Some(Token::EOF) => Ok(None),
        _ => parse_expression_as_statement(parser, ctx),
    }
}

// parse_expression-dan gələn dəyəri sadəcə ifadə kimi emal edir
pub fn parse_expression_as_statement(
    parser: &mut Parser,
    ctx: &mut TranspileContext,
) -> Result<Option<Expr>, String> {
    let expr = parser.parse_expression(ctx)?;
    record_variable_usage(&expr, &mut parser.used_variables);

    Ok(Some(expr))
}

/* İstifadə olunan dəyişənləri yoxlayır */
fn record_variable_usage(expr: &Expr, used: &mut std::collections::HashSet<String>) {
    match expr {
        Expr::VariableRef { name, .. } => {
            used.insert(name.clone());
        }
        Expr::BinaryOp { left, right, .. } => {
            record_variable_usage(left, used);
            record_variable_usage(right, used);
        }
        Expr::FunctionCall { args, .. } | Expr::BuiltInCall { args, .. } | Expr::List(args) => {
            for arg in args {
                record_variable_usage(arg, used);
            }
        }
        Expr::MethodCall { target, args, .. } => {
            record_variable_usage(target, used);
            for arg in args {
                record_variable_usage(arg, used);
            }
        }
        Expr::Return(inner)
        | Expr::Index { target: inner, .. }
        | Expr::MutableDecl { value: inner, .. }
        | Expr::ConstantDecl { value: inner, .. } => {
            record_variable_usage(inner, used);
        }
        _ => {}
    }
}
