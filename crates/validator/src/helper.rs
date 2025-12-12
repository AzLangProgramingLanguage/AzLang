use std::borrow::Cow;

use parser::{ast::Expr, shared_ast::Type};

use crate::{ValidatorContext, validate::validate_expr};

pub fn get_type<'a>(
    value: &Expr<'a>,
    ctx: &ValidatorContext<'a>,
    typ: Option<&Type<'a>>,
) -> Type<'a> {
    match value {
        Expr::Number(_) => Type::Natural,
        Expr::UnaryOp { op, expr } => {
            get_type(expr, ctx, typ);
            match &**op {
                "-" => Type::Integer,
                "!" => Type::Bool,
                _ => Type::Any,
            }
        }
        Expr::Bool(_) => Type::Bool,

        Expr::Float(_) => Type::Float,
        Expr::String(_) => Type::LiteralString,
        Expr::List(items) => {
            if items.len() > 0 {
                let item_type = get_type(&items[0], ctx, typ);
                for item in &items[1..] {
                    let t = get_type(item, ctx, typ);
                    if t != item_type {
                        return Type::Array(Box::new(Type::Any));
                    }
                }

                Type::Array(Box::new(item_type))
            } else {
                Type::Any
            }
        }
        Expr::Index {
            target: _,
            index: _,
            target_type,
        } => target_type.clone(),
        Expr::VariableRef { name, symbol } => {
            if let Some(s) = ctx.lookup_variable(name) {
                return s.typ.clone();
            }

            if let Some(t) = typ {
                if let Type::User(enum_name) = t {
                    if let Some(variants) = ctx.enum_defs.get(enum_name) {
                        if variants.contains(name) {
                            return t.clone();
                        }
                    }
                }
            }
            return symbol.as_ref().unwrap().typ.clone();
        }
        Expr::StructInit { name, .. } => {
            if let Some((..)) = ctx.struct_defs.get(name.as_ref()) {
                Type::User(Cow::Owned(name.to_string()))
            } else if let Some((..)) = ctx.union_defs.get(name.as_ref()) {
                Type::User(Cow::Owned(name.to_string()))
            } else {
                Type::Any
            }
        }

        Expr::BuiltInCall { return_type, .. } => return_type.clone(),
        Expr::Call { returned_type, .. } => returned_type.clone().unwrap_or(Type::Any), /* TODO: Burada Any Olmamalıdır */
        Expr::BinaryOp { variables, op } => {
            //BUG::  Burada Ciddi bug var tamamlanmayıb
            let mut last_type = Type::Any;
            for variable in variables {
                let variable_type = get_type(variable, ctx, typ);
                last_type = match (last_type, &variable_type) {
                    (Type::Any, _) => variable_type,
                    (Type::Integer, Type::Integer) => Type::Any,
                    (_, _) => Type::Any,
                }
            }
            let comparison_ops = ["==", "!=", "<", "<=", ">", ">="];
            let logic_ops = ["&&", "||"];
            let arithmetic_ops = ["+", "-", "*", "/", "%"];

            last_type
        }
        _ => Type::Any,
    }
}
