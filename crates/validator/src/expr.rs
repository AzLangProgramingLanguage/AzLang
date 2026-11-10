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
        ParserExpr::String(s) => Ok(ValidatorExpr::String(s.to_string())),
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
        ParserExpr::VariableRef { name, .. } => {
            let s = ctx.lookup_variable_mut_with_err(&name)?;
            s.is_used = true;
            Ok(ValidatorExpr::VariableRef {
                name: name.to_string(),
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
        ParserExpr::Break => Ok(ValidatorExpr::Break),
        ParserExpr::Continue => Ok(ValidatorExpr::Continue),
        _ => todo!("This is not implemented yet"),
    }
}
