use parser::{
    ast::Symbol,
    binary_op,
    shared_ast::{StringEnum, Type},
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
    let return_type = get_type(&expr, ctx)?;
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
                    parser::ast::TemplateChunk::Expr(e) => Ok(crate::ast::TemplateChunk::Expr(
                        Box::new(validate_expr(*e, ctx)?),
                    )),
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
                symbol: s.clone(),
            })
        }
        ParserExpr::BinaryOp { left, right, op } => {
            let left = validate_expr(*left, ctx)?;
            let right = validate_expr(*right, ctx)?;
            Ok(ValidatorExpr::BinaryOp {
                left: Box::new(left),
                right: Box::new(right),
                op,
                return_type,
            })
        }
        ParserExpr::Call { target, name, args } => {
            let target = target
                .map(|t| validate_expr(*t, ctx))
                .transpose()?
                .map(Box::new);

            let func_name = match *name {
                ParserExpr::VariableRef { name: nam, .. } => {
                    ctx.functions
                        .get(&nam)
                        .ok_or_else(|| ValidatorError::FunctionNotFound(nam.clone()))?;
                    nam
                }
                _ => {
                    return Err(ValidatorError::FunctionNotFound(format!(
                        "{name:?} bu bir funksiya deyil"
                    )));
                }
            };

            let func_info = ctx.functions.get(&func_name).unwrap();
            let params = &func_info.parameters;

            if args.len() != params.len() {
                return Err(ValidatorError::InvalidArgumentCount {
                    name: func_name.clone(),
                    expected: params.len(),
                    found: args.len(),
                });
            }

            for (arg, param) in args.iter().zip(params.iter()) {
                let arg_type = get_type(arg, ctx)?;
                let expected = &param.typ;
                match (expected, &arg_type) {
                    (Type::Any, _) | (_, Type::Any) => {}
                    (
                        Type::String(StringEnum::LiteralConstString),
                        Type::String(StringEnum::LiteralString),
                    ) => {}
                    (exp, found) if exp != found => {
                        return Err(ValidatorError::InvalidArgumentType {
                            name: func_name.clone(),
                            expected: exp.to_string(),
                            found: found.to_string(),
                        });
                    }
                    _ => {}
                }
            }

            let new_name = ValidatorExpr::VariableRef {
                name: func_name,
                symbol: Symbol {
                    typ: Type::Function,
                    is_mutable: false,
                    is_used: false,
                    is_changed: false,
                },
            };

            let validated_args = args
                .into_iter()
                .map(|arg| validate_expr(arg, ctx))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(ValidatorExpr::Call {
                target,
                name: Box::new(new_name),
                args: validated_args,
                returned_type: return_type,
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
