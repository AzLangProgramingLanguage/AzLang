use parser::{
    ast::{Expr::BuiltInCall, Statement, Symbol},
    shared_ast::{BuiltInFunction, StringEnum, Type},
};
type ValidatorExpr = crate::ast::Expr;
use crate::{
    Validator,
    ast::{
        Ast::{self},
        Decl, Program,
    },
    errors::ValidatorError,
    expr::validate_expr,
    helper::{get_type, reconcile_type},
};
pub fn validate_statement(
    stmt: Statement,
    program: &mut Program,
    ctx: &mut Validator,
) -> Result<(), ValidatorError> {
    match stmt {
        Statement::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            if ctx.lookup_variable(&name).is_some() {
                return Err(ValidatorError::AlreadyDecl(name.to_string()));
            }

            let mut inferred = get_type(&value, ctx);
            inferred = reconcile_type(typ, inferred, &name)?;

            ctx.declare_variable(
                name.to_string(),
                Symbol {
                    typ: inferred.clone(),
                    is_used: false,
                    is_pointer: is_mutable,
                    is_changed: false,
                },
            );
            let val = validate_expr(*value, ctx)?;

            program.expressions.push(Ast::Decl(Decl {
                name,
                typ: inferred,
                value: Box::new(val),
            }));
        }
        Statement::Expr(expr) => match expr {
            BuiltInCall { function, mut args } => {
                let mut validated_args = Vec::new();
                while let Some(arg) = args.pop() {
                    validated_args.push(validate_expr(arg, ctx)?);
                }

                let return_type = match &function {
                    &BuiltInFunction::Print => Type::Void,
                    &BuiltInFunction::Input => Type::String(StringEnum::DynamicString),
                    &BuiltInFunction::Len => Type::Natural,
                    &BuiltInFunction::Number => Type::Integer,
                    &BuiltInFunction::Sum => Type::Integer,
                    &BuiltInFunction::Range => Type::Array(Box::new(Type::Integer)),
                    &BuiltInFunction::LastWord => Type::Void,
                    &BuiltInFunction::Timer => Type::Integer,
                    &BuiltInFunction::Max => Type::Integer,
                    &BuiltInFunction::Zig => Type::Void,
                    &BuiltInFunction::StrLower
                    | &BuiltInFunction::StrUpper
                    | &BuiltInFunction::Trim
                    | &BuiltInFunction::StrReverse
                    | &BuiltInFunction::ConvertString => Type::String(StringEnum::DynamicString),
                    &BuiltInFunction::Allocator => Type::Void,
                    &BuiltInFunction::Min => Type::Integer,
                    &BuiltInFunction::Sqrt => Type::Float,
                    &BuiltInFunction::Mod => Type::Integer,
                    &BuiltInFunction::Ceil => Type::Integer,
                    &BuiltInFunction::Floor => Type::Integer,
                    &BuiltInFunction::Round => Type::Integer,
                };

                program
                    .expressions
                    .push(Ast::Expr(ValidatorExpr::BuiltInCall {
                        function,
                        args: validated_args,
                        return_type,
                    }));
            }
            _ => todo!(),
        },
        _ => {}
    }
    Ok(())
}
