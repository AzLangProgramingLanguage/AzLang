use parser::{
    ast::Operation,
    shared_ast::{StringEnum, Type},
};
use validator::ast::{Ast, Expr};

use crate::{TranspileContext, transpile::transpile_stmt};

pub fn map_typ(typ: &Type) -> &'static str {
    match typ {
        Type::Natural => "i64",
        Type::Integer => "i64",
        Type::String(typ) => match typ {
            StringEnum::DynamicString => "[]u8",
            StringEnum::LiteralString => "[]u8",
            StringEnum::LiteralConstString => "[]const u8",
        },
        Type::Float => "f64",
        other => todo!("Hele burası hazır deyil {other:?}"),
    }
}

pub fn get_expr_type(expr: &Expr) -> Type {
    match expr {
        Expr::String(_) => Type::String(StringEnum::LiteralConstString),
        Expr::Number(_) => Type::Integer,
        Expr::Float(_) => Type::Float,
        Expr::Bool(_) => Type::Bool,
        Expr::Char(_) => Type::Char,

        Expr::List(items) => {
            if items.is_empty() {
                return Type::Array(Box::new(Type::Any));
            }
            let item_type = get_expr_type(&items[0]);

            for item in &items[1..] {
                let t = get_expr_type(item);
                if t != item_type {
                    return Type::Array(Box::new(Type::Any));
                }
            }

            Type::Array(Box::new(item_type))
        }

        _ => Type::Any,
    }
}

pub fn get_format_str_from_type(t: &Type) -> &'static str {
    match t {
        Type::String(StringEnum::LiteralString) | Type::String(StringEnum::LiteralConstString) => {
            "{s}"
        }
        Type::Integer => "{}",
        Type::Any => "{any}",
        Type::Float => "{d}",
        other => {
            panic!("{other:?}")
        }
    }
}
//
// pub fn map_type(typ: &Type, is_const: bool) -> &'static str {
//     match typ {
//         Type::Integer => "isize",
//         Type::Natural => "usize",
//         Type::Any => "any",
//         Type::Void => "void",
//         Type::ZigFloat => "f64",
//         Type::Float => "f64",
//         Type::BigInteger => {
//             if is_const {
//                 "const i128"
//             } else {
//                 "i128"
//             }
//         }
//         Type::Char => "u8",
//         Type::LowInteger => {
//             if is_const {
//                 "const u8"
//             } else {
//                 "u8"
//             }
//         }
//         Type::String => {
//             if is_const {
//                 "[]u8"
//             } else {
//                 "[]u8"
//             }
//         }
//         Type::LiteralString => "[]const u8",
//         Type::ZigNatural => "usize",
//         Type::ZigInteger => "isize",
//         Type::LiteralConstString => "[]const u8",
//         Type::ZigArray => "[]usize",
//         Type::ZigConstArray => "[]const usize",
//         Type::Bool => "bool",
//         Type::Array(inner) => {
//             let inner_str = map_type(inner, is_const);
//             inner_str
//         }
//         Type::User(_) => "any",
//         Type::Allocator => "std.mem.Allocator",
//         Type::Function => "any",
//     }
// }
//
pub fn is_semicolon_needed(stmt: &Ast) -> bool {
    matches!(stmt, Ast::Expr(..) | Ast::Decl { .. })
}
//
pub fn is_primite_value(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Number(_)
            | Expr::Float(_)
            | Expr::Bool(_)
            | Expr::Char(_)
            | Expr::String(_)
            | Expr::List(..)
    )
}
//
pub fn transpile_body(body: Vec<Ast>, ctx: &mut TranspileContext) -> String {
    body.into_iter()
        .map(|stmt| {
            let mut s = String::new();
            if is_semicolon_needed(&stmt) {
                s.push_str(&transpile_stmt(stmt, ctx));
                s.push(';');
            }
            s
        })
        .collect()
}
