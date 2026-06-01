use parser::{
    ast::Symbol,
    shared_ast::{BuiltInFunction, StringEnum, Type},
};

use crate::{
    Validator,
    ast::{self, Ast},
    errors::ValidatorError,
    helper::get_type,
};
type ParserExpr = parser::ast::Expr;
type ValidatorExpr = ast::Expr;

pub fn validate_expr(
    expr: ParserExpr,
    ctx: &mut Validator,
) -> Result<ValidatorExpr, ValidatorError> {
    match expr {
        ParserExpr::String(s) => Ok(ValidatorExpr::String(s)),
        ParserExpr::Number(n) => Ok(ValidatorExpr::Number(n)),
        ParserExpr::Float(f) => Ok(ValidatorExpr::Float(f)),
        ParserExpr::Bool(b) => Ok(ValidatorExpr::Bool(b)),
        ParserExpr::Char(c) => Ok(ValidatorExpr::Char(c)),
        ParserExpr::Void => Ok(ValidatorExpr::Void),
        ParserExpr::TemplateString(chunks) => {
            let validated: Result<Vec<crate::ast::TemplateChunk>, ValidatorError> = chunks
                .into_iter()
                .map(|chunk| match chunk {
                    parser::ast::TemplateChunk::Literal(l) => {
                        Ok(crate::ast::TemplateChunk::Literal(l))
                    }
                    parser::ast::TemplateChunk::Expr(e) => {
                        Ok(crate::ast::TemplateChunk::Expr(Box::new(validate_expr(
                            *e, ctx,
                        )?)))
                    }
                })
                .collect();
            Ok(ValidatorExpr::TemplateString(validated?))
        }
        ParserExpr::List(items) => {
            let validated: Result<Vec<ValidatorExpr>, ValidatorError> =
                items.into_iter().map(|x| validate_expr(x, ctx)).collect();
            Ok(ValidatorExpr::List(validated?))
        }
        ParserExpr::Return(e) => {
            let validated = validate_expr(*e, ctx)?;
            Ok(ValidatorExpr::Return(Box::new(validated)))
        }
        ParserExpr::VariableRef { name, symbol } => {
            let s = ctx.lookup_variable_mut_with_err(&name)?;
            s.is_used = true;
            Ok(ValidatorExpr::VariableRef {
                name,
                symbol,
            })
        }
        ParserExpr::BinaryOp { left, right, op } => {
            let left = validate_expr(*left, ctx)?;
            let right = validate_expr(*right, ctx)?;
            let return_type = get_type(
                &ParserExpr::BinaryOp {
                    left: Box::new(ParserExpr::Void),
                    right: Box::new(ParserExpr::Void),
                    op,
                },
                ctx,
            );
            Ok(ValidatorExpr::BinaryOp {
                left: Box::new(left),
                right: Box::new(right),
                op,
                return_type,
            })
        }
        ParserExpr::Call {
            target,
            name,
            args,
        } => {
            let target = {
                match target {
                    Some(t) => Some(Box::new(validate_expr(*t, ctx)?)),
                    None => None,
                }
            };
            let name = Box::new(validate_expr(*name, ctx)?);
            let mut validated_args = Vec::new();
            for arg in args {
                validated_args.push(validate_expr(arg, ctx)?);
            }
            let returned_type = match &*name {
                ValidatorExpr::VariableRef { name: func_name, .. } => {
                    ctx.functions
                        .get(func_name)
                        .map(|f| f.return_type.clone())
                        .unwrap_or(Type::Any)
                }
                _ => Type::Any,
            };
            Ok(ValidatorExpr::Call {
                target,
                name,
                args: validated_args,
                returned_type,
            })
        }
        ParserExpr::BuiltInCall { function, args } => {
            let mut validated_args = Vec::new();
            for arg in args {
                validated_args.push(validate_expr(arg, ctx)?);
            }

            let return_type = match &function {
                BuiltInFunction::Print => Type::Void,
                BuiltInFunction::Input => Type::String(StringEnum::DynamicString),
                BuiltInFunction::Len => Type::Natural,
                BuiltInFunction::Number => Type::Integer,
                BuiltInFunction::Sum => Type::Integer,
                BuiltInFunction::Range => Type::Array(Box::new(Type::Integer)),
                BuiltInFunction::LastWord => Type::Void,
                BuiltInFunction::Timer => Type::Integer,
                BuiltInFunction::Max => Type::Integer,
                BuiltInFunction::Zig => Type::Void,
                BuiltInFunction::StrLower
                | BuiltInFunction::StrUpper
                | BuiltInFunction::Trim
                | BuiltInFunction::StrReverse
                | BuiltInFunction::ConvertString => Type::String(StringEnum::DynamicString),
                BuiltInFunction::Allocator => Type::Void,
                BuiltInFunction::Min => Type::Integer,
                BuiltInFunction::Sqrt => Type::Float,
                BuiltInFunction::Mod => Type::Integer,
                BuiltInFunction::Ceil => Type::Integer,
                BuiltInFunction::Floor => Type::Integer,
                BuiltInFunction::Round => Type::Integer,
            };

            Ok(ValidatorExpr::BuiltInCall {
                function,
                args: validated_args,
                return_type,
            })
        }
        ParserExpr::UnaryOp { op, expr } => {
            let expr = validate_expr(*expr, ctx)?;
            let return_type = match &op {
                parser::ast::Operation::Subtract => Type::Integer,
                parser::ast::Operation::Not => Type::Bool,
                _ => Type::Any,
            };
            Ok(ValidatorExpr::BinaryOp {
                left: Box::new(ValidatorExpr::Void),
                right: Box::new(expr),
                op,
                return_type,
            })
        }
        ParserExpr::Index {
            target,
            index,
            target_type,
        } => {
            let target = validate_expr(*target, ctx)?;
            let index = validate_expr(*index, ctx)?;
            Ok(ValidatorExpr::BinaryOp {
                left: Box::new(target),
                right: Box::new(index),
                op: parser::ast::Operation::Equal,
                return_type: target_type,
            })
        }
        ParserExpr::DynamicString(_)
        | ParserExpr::Time(_)
        | ParserExpr::Comment(_)
        | ParserExpr::StructInit { .. }
        | ParserExpr::Break
        | ParserExpr::Continue => Ok(ValidatorExpr::Void),
    }
}
