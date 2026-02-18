use std::borrow::Cow;

use parser::{
    ast::{Expr, Operation},
    shared_ast::Type,
};

use crate::{ValidatorContext, errors::ValidatorError, validate::validate_expr};

pub fn get_type<'a>(
    value: &Expr<'a>,
    ctx: &mut ValidatorContext<'a>,
    typ: Option<&Type<'a>>,
) -> Type<'a> {
    match value {
        Expr::Number(_) => Type::Integer,
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
        Expr::BinaryOp {
            left,
            right,
            op,
            return_type,
        } => {
            let left_type = get_type(left, ctx, typ);
            let right_type = get_type(right, ctx, typ);
            let last_type: Type<'_> = match *op {
                Operation::Equal
                | Operation::NotEqual
                | Operation::Less
                | Operation::LessEqual
                | Operation::Greater
                | Operation::GreaterEqual => {
                    if left_type != right_type {
                        return Type::Bool;
                    }
                    Type::Bool
                }
                Operation::And | Operation::Or => {
                    if left_type != Type::Bool || right_type != Type::Bool {
                        return Type::Bool;
                    }
                    Type::Bool
                }
                Operation::Add
                | Operation::Subtract
                | Operation::Multiply
                | Operation::Divide
                | Operation::Modulo => match (left_type, right_type) {
                    (Type::Integer, Type::Integer) => Type::Integer,
                    (Type::Natural, Type::Natural) => Type::Natural,
                    (Type::Float, Type::Float) => Type::Float,
                    (Type::Integer, Type::Float) => Type::Float,
                    (Type::Float, Type::Integer) => Type::Float,
                    _ => Type::Any,
                },
                _ => Type::Any,
            };
            last_type
        }
        _ => Type::Any,
    }
}

pub fn validate_body<'a>(
    body: &mut Vec<Expr<'a>>,
    ctx: &mut ValidatorContext<'a>,
) -> Result<(), ValidatorError> {
    for expr in body {
        validate_expr(expr, ctx)?;
    }
    Ok(())
}

pub fn validate_bool_condition<'a>(
    condition: &mut Expr<'a>,
    ctx: &mut ValidatorContext<'a>,
) -> Result<(), ValidatorError> {
    validate_expr(condition, ctx)?;

    match get_type(condition, ctx, None) {
        Type::Any => Err(ValidatorError::IfConditionTypeUnknown),
        Type::Bool => Ok(()),
        other => Err(ValidatorError::IfConditionTypeMismatch(other.to_string())),
    }
}
