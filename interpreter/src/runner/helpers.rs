use crate::{
    parser::ast::{Expr, Type},
    runner::{Runner, eval::eval, runner::runner_interpretator},
};

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
pub fn exec_block<'a>(ctx: &mut Runner<'a>, body: Vec<Expr<'a>>) -> Option<Expr<'a>> {
    for expr in body {
        match expr {
            Expr::Return(value) => return Some(eval(&*value, ctx)),
            _ => {
                if let Some(val) = runner_interpretator(ctx, expr) {
                    return Some(val);
                }
            }
        }
    }
    None
}
