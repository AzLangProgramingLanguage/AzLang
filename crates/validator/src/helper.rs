use std::rc::Rc;

use parser::{
    ast::{Expr, Operation},
    shared_ast::{StringEnum, Type},
};

use crate::{Validator, errors::ValidatorError, expr::validate_expr};
//TODO: List Type Definition has a problem. We must use Type::Integer instead of Type::Natural
pub fn get_type<'a>(value: &Expr, ctx: &Validator) -> Type {
    match value {
        Expr::Number(_) => Type::Integer,
        Expr::TemplateString(_) => Type::String(StringEnum::DynamicString),
        Expr::UnaryOp { op, expr } => {
            get_type(expr, ctx);
            match &*op {
                Operation::Subtract => Type::Integer,
                Operation::Not => Type::Bool,
                _ => Type::Any,
            }
        }
        Expr::Bool(_) => Type::Bool,

        Expr::Float(_) => Type::Float,
        Expr::String(_) => Type::String(StringEnum::LiteralString),
        Expr::List(items) => {
            if items.len() > 0 {
                let item_type = get_type(&items[0], ctx);
                for item in &items[1..] {
                    let t = get_type(item, ctx);
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
            return symbol.as_ref().unwrap().typ.clone();
        }
        Expr::StructInit { name, .. } => {
            Type::Any
            // if let Some((..)) = ctx.struct_defs.get(name.as_ref()) {
            //     Type::User(*name)
            // } else if let Some((..)) = ctx.union_defs.get(name.as_ref()) {
            //     Type::User(name.to_string())
            // } else {
            //     Type::Any
            // }
        }

        Expr::BuiltInCall { return_type, .. } => return_type.clone(),
        Expr::Call { returned_type, .. } => returned_type.clone().unwrap_or(Type::Any), /* TODO: Burada Any Olmamalıdır */
        Expr::BinaryOp { left, right, op } => {
            let left_type = get_type(left, ctx);
            let right_type = get_type(right, ctx);
            let last_type: Type = match *op {
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

pub fn validate_body<'a>(body: &mut Vec<Expr>, ctx: &mut Validator) -> Result<(), ValidatorError> {
    for expr in body {
        validate_expr(expr, ctx)?;
    }
    Ok(())
}

pub fn validate_bool_condition(
    condition: &mut Expr,
    ctx: &mut Validator,
) -> Result<(), ValidatorError> {
    validate_expr(condition, ctx)?;

    match get_type(condition, ctx) {
        Type::Any => Err(ValidatorError::IfConditionTypeUnknown),
        Type::Bool => Ok(()),
        other => Err(ValidatorError::IfConditionTypeMismatch(other.to_string())),
    }
}

pub fn reconcile_type(
    typ: &mut Rc<Type>,
    inferred: Type,
    name: &str,
) -> Result<(), ValidatorError> {
    match (&**typ, &inferred) {
        // Annotasiya yoxdur — nəticə çıxar
        (Type::Any, _) => {
            *typ = Rc::new(inferred);
        }
        // İnferred `Any` — heç nə etmə
        (_, Type::Any) => {}
        // String literal annotasiyaya uyğun gəlir

        // Uyğunsuzluq
        (expected, _) if inferred != **typ => {
            return Err(ValidatorError::DeclTypeMismatch {
                name: name.to_string(),
                expected: inferred.to_string(),
                found: expected.to_string(),
            });
        }
        _ => {}
    }
    Ok(())
}
