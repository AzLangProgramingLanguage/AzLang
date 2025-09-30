use crate::parser::ast::{Expr, Type};

pub fn get_run_type<'a>(value: &Expr<'_>) -> Type<'a> {
    match value {
        Expr::Number(_) => Type::Integer,
        Expr::Float(_) => Type::Float,
        Expr::Bool(_) => Type::Bool,
        Expr::Char(_) => Type::Char,
        Expr::String(_, _) => Type::Metn,
        Expr::DynamicString(_) => Type::Metn,
        Expr::VariableRef { .. } => Type::Any,
        Expr::BinaryOp { .. } => Type::Any,
        Expr::UnaryOp { .. } => Type::Any,
        Expr::Call { .. } => Type::Any,
        Expr::StructDef { .. } => Type::Any,
        Expr::FunctionDef { .. } => Type::Any,
        Expr::Assignment { .. } => Type::Any,
        Expr::If { .. } => Type::Any,
        Expr::Match { .. } => Type::Any,
        Expr::BuiltInCall { .. } => Type::Any,
        _ => Type::Any,
    }
}
