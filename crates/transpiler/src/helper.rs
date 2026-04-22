use parser::{
    ast::{Expr, Operation, Statement},
    shared_ast::{StringEnum, Type},
};

use crate::{TranspileContext, transpile::transpile_stmt};

pub fn map_typ(typ: &Type) -> &'static str {
    match typ {
        Type::Natural => "i64",
        Type::String(typ) => match typ {
            StringEnum::DynamicString => "[]u8",
            StringEnum::LiteralString => "[]u8",
            StringEnum::LiteralConstString => "[]const u8",
        },
        _ => "Any",
    }
}

pub fn get_expr_type<'a>(expr: &Expr) -> Type {
    match expr {
        Expr::String(_) => Type::String(StringEnum::LiteralConstString),
        Expr::Number(_) => Type::Integer,
        Expr::Float(_) => Type::Float,
        Expr::Bool(_) => Type::Bool,
        Expr::Char(_) => Type::Char,
        Expr::UnaryOp { op, expr: _ } => match *op {
            Operation::Not => Type::Bool,
            Operation::Subtract => Type::Integer,
            Operation::Add => Type::Natural,
            other => {
                panic!("get_expr_type de bilinmeyen unaryOp tipi geldi {other:#?} ")
            }
        },

        Expr::Call {
            target,
            name,
            args,
            returned_type,
        } => returned_type.as_ref().unwrap().clone(),
        Expr::VariableRef { name: _, symbol } => symbol.as_ref().unwrap().typ.clone(),
        Expr::BuiltInCall {
            function: _,
            args: _,
            return_type,
        } => return_type.clone(),

        Expr::BinaryOp {
            left, op, right, ..
        } => {
            /*TODO:: Bu nə pis koddu belə */
            let left_type = get_expr_type(left);
            let right_type = get_expr_type(right);

            let comparison_ops = ["==", "!=", "<", "<=", ">", ">="];
            let logic_ops = ["&&", "||"];
            let arithmetic_ops = ["+", "-", "*", "/", "%"];

            /*   if comparison_ops.contains(&op) || logic_ops.contains(&op) {
                return Type::Bool;
            }

            if arithmetic_ops.contains(&op) {
                if left_type == Type::Integer && right_type == Type::Integer {
                    return Type::Integer;
                } else if left_type == Type::Natural && right_type == Type::Natural {
                    return Type::Natural;
                } else if left_type == Type::Float && right_type == Type::Float {
                    return Type::Float;
                } else if left_type == Type::Integer && right_type == Type::Natural {
                    return Type::Integer;
                } else if left_type == Type::Natural && right_type == Type::Integer {
                    return Type::Integer;
                } else {
                    return Type::Float;
                }
            } */
            Type::Any
        }
        Expr::Index {
            target: _,
            index: _,
            target_type,
        } => target_type.clone(),
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
        _ => todo!(),
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
pub fn is_semicolon_needed(stmt: &Statement) -> bool {
    matches!(stmt, Statement::Expr(..) | Statement::Decl { .. })
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
            | Expr::UnaryOp { .. }
            | Expr::List(..)
    )
}
//
pub fn transpile_body(body: Vec<Statement>, ctx: &mut TranspileContext) -> String {
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
