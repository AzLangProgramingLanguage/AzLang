use std::rc::Rc;

use parser::{
    ast::{
        Expr::{self, VariableRef},
        Operation, Statement,
    },
    shared_ast::{StringEnum, Type},
};

use crate::{Validator, ast::Ast, errors::ValidatorError, validate::validate_statement};
pub fn get_type(value: &Expr, ctx: &Validator) -> Result<Type, ValidatorError> {
    match value {
        Expr::Number(_) => Ok(Type::Integer),
        Expr::TemplateString(_) => Ok(Type::String(StringEnum::DynamicString)),
        Expr::UnaryOp { op, expr } => {
            get_type(expr, ctx)?;
            match &*op {
                Operation::Subtract => Ok(Type::Integer),
                Operation::Not => Ok(Type::Bool),
                _ => Err(ValidatorError::UnknownType(format!("unary op {op:?}"))),
            }
        }
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Float(_) => Ok(Type::Float),
        Expr::String(_) => Ok(Type::String(StringEnum::LiteralString)),
        Expr::List(items) => {
            if items.is_empty() {
                return Err(ValidatorError::UnknownType("empty list".to_string()));
            }
            let item_type = get_type(&items[0], ctx)?;
            for item in &items[1..] {
                let t = get_type(item, ctx)?;
                if t != item_type {
                    return Err(ValidatorError::TypeMismatch {
                        expected: item_type.clone(),
                        found: t,
                    });
                }
            }
            Ok(Type::Array(Box::new(item_type)))
        }
        Expr::Index {
            target: _,
            index: _,
            target_type,
        } => Ok(target_type.clone()),
        Expr::VariableRef { name, symbol } => {
            if let Some(s) = symbol {
                return Ok(s.typ.clone());
            }
            if let Some(s) = ctx.lookup_variable(name) {
                return Ok(s.typ.clone());
            }
            if ctx.functions.contains_key(name) {
                return Ok(Type::Function);
            }
            Err(ValidatorError::UndefinedVariable(name.clone()))
        }
        Expr::StructInit { name, .. } => Err(ValidatorError::UnknownStruct(name.clone())),
        Expr::Return(e) => get_type(e, ctx),
        Expr::Call { name, .. } => match &**name {
            VariableRef { name, symbol } => {
                if let Some(func) = ctx.functions.get(name) {
                    Ok(func.return_type.clone())
                } else {
                    Err(ValidatorError::FunctionNotFound(name.clone()))
                }
            }
            _ => Err(ValidatorError::InvalidFunctionCall(format!(" {name:?}"))),
        },
        Expr::BinaryOp { left, right, op } => {
            let left_type = get_type(left, ctx)?;
            let right_type = get_type(right, ctx)?;

            match *op {
                // ── Müqayisə əməliyyatları ────────────────────────────────────────
                Operation::Equal
                | Operation::NotEqual
                | Operation::Less
                | Operation::LessEqual
                | Operation::Greater
                | Operation::GreaterEqual => Ok(Type::Bool),

                // ── Məntiqi əməliyyatlar ──────────────────────────────────────────
                Operation::And | Operation::Or => {
                    expect_type(Type::Bool, &left_type)?;
                    expect_type(Type::Bool, &right_type)?;
                    Ok(Type::Bool)
                }

                // ── Riyazi əməliyyatlar ───────────────────────────────────────────
                Operation::Add
                | Operation::Subtract
                | Operation::Multiply
                | Operation::Divide
                | Operation::Modulo => resolve_arithmetic_type(&left_type, &right_type, *op),

                _ => Err(ValidatorError::UnknownType(format!(
                    "unknown binary op {op:?}"
                ))),
            }
        }
        Expr::Void => Ok(Type::Void),
        Expr::Char(_) => Ok(Type::Char),
        Expr::DynamicString(_) => Ok(Type::String(StringEnum::DynamicString)),
        Expr::Time(_) => Ok(Type::Void),
        Expr::Comment(_) => Ok(Type::Void),
        Expr::Break | Expr::Continue => Ok(Type::Void),
        _ => Err(ValidatorError::UnknownType(format!(
            "unknown expr {value:?}"
        ))),
    }
}

pub fn validate_body<'a>(
    body: Vec<Statement>,
    ctx: &mut Validator,
) -> Result<Vec<Ast>, ValidatorError> {
    let mut result = Vec::new();
    for expr in body {
        result.push(validate_statement(expr, ctx)?);
    }
    Ok(result)
}
pub fn type_checking(left: Type, right: Type) -> Result<(), ValidatorError> {
    match (left, right) {
        (Type::Any, _) => Ok(()),
        (_, Type::Any) => Ok(()),
        (Type::String(StringEnum::LiteralConstString), Type::String(StringEnum::LiteralString)) => {
            Ok(())
        }

        (expected, other) if expected != other => Err(ValidatorError::AssignmentTypeMismatch {
            name: other.to_string(),
            expected: expected.to_string(),
            found: other.to_string(),
        }),
        _ => Ok(()),
    }
}
pub fn reconcile_type(typ: Rc<Type>, inferred: Type, name: &str) -> Result<Type, ValidatorError> {
    match (&*typ, &inferred) {
        (Type::Any, _) => Ok(inferred),
        (other, Type::Any) => Ok(other.clone()),
        (Type::String(StringEnum::LiteralConstString), Type::String(StringEnum::LiteralString)) => {
            Ok(Type::String(StringEnum::LiteralConstString))
        }

        (expected, other) if inferred != *other => Err(ValidatorError::DeclTypeMismatch {
            name: name.to_string(),
            expected: inferred.to_string(),
            found: expected.to_string(),
        }),
        other => Ok(other.0.clone()),
    }
}
fn resolve_arithmetic_type(
    left: &Type,
    right: &Type,
    op: Operation,
) -> Result<Type, ValidatorError> {
    match (left, right) {
        // String birləşməsi yalnız Add üçün
        (Type::String(_), Type::String(_)) if op == Operation::Add => {
            Ok(Type::String(StringEnum::DynamicString))
        }

        // String-ə digər riyazi əməliyyatlar qadağandır
        (Type::String(_), _) | (_, Type::String(_)) => Err(ValidatorError::InvalidOperation {
            op,
            left: left.clone(),
            right: right.clone(),
        }),

        // Eyni tiplər
        (Type::Integer, Type::Integer) => Ok(Type::Integer),
        (Type::Natural, Type::Natural) => Ok(Type::Natural),
        (Type::Float, Type::Float) => Ok(Type::Float),

        // Float + Integer qarışığı → Float
        (Type::Float, Type::Integer) | (Type::Integer, Type::Float) => Ok(Type::Float),

        // Hər şey digər hal — tip uyğunsuzluğu
        (l, r) => Err(ValidatorError::TypeMismatch {
            expected: l.clone(),
            found: r.clone(),
        }),
    }
}

/// Tipin gözlənilən tipə uyğun olmasını yoxlayır.
#[inline]
fn expect_type(expected: Type, found: &Type) -> Result<(), ValidatorError> {
    if *found != expected {
        Err(ValidatorError::TypeMismatch {
            expected,
            found: found.clone(),
        })
    } else {
        Ok(())
    }
}
