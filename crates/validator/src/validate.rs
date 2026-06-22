use std::{collections::HashMap, rc::Rc};

use parser::{
    ast::{Statement, Symbol},
    shared_ast::{StringEnum, Type},
};
type ValidatorExpr = crate::ast::Expr;
use crate::{
    decl, Validator,
    ast::{self, Ast, Else, Function, IF},
    errors::ValidatorError,
    expr::validate_expr,
    helper::{get_type, type_checking},
};
pub fn validate_statement(stmt: Statement, ctx: &mut Validator) -> Result<Ast, ValidatorError> {
    match stmt {
        Statement::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => decl::validate_decl(name, typ, is_mutable, *value, ctx),
        Statement::Assignment { name, value } => {
            let inferred = get_type(&value, ctx)?;
            let symbol = ctx.lookup_variable_mut_with_err(name.as_ref())?;
            symbol.is_changed = true;
            if !symbol.is_mutable {
                return Err(ValidatorError::AssignmentToImmutableVariable(
                    name.to_string(),
                ));
            }

            type_checking(symbol.typ.clone(), inferred)?;

            let val = validate_expr(*value, ctx)?;

            Ok(Ast::Assignment {
                name: name.to_string(),
                value: Box::new(val),
            })
        }

        Statement::Condition { main, elif, other } => {
            let condition = validate_expr(*main.condition, ctx)?;
            let mut validated_body = Vec::new();
            for stmt in main.body {
                validated_body.push(validate_statement(stmt, ctx)?);
            }
            let validated_main = IF {
                condition: Box::new(condition),
                body: validated_body,
            };

            let mut validated_elif = Vec::new();
            for branch in elif {
                let branch_condition = validate_expr(*branch.condition, ctx)?;
                let mut branch_body = Vec::new();
                for stmt in branch.body {
                    branch_body.push(validate_statement(stmt, ctx)?);
                }
                validated_elif.push(IF {
                    condition: Box::new(branch_condition),
                    body: branch_body,
                });
            }

            let validated_other = other
                .map(|o| {
                    let mut body = Vec::new();
                    for stmt in o.body {
                        body.push(validate_statement(stmt, ctx)?);
                    }
                    Ok::<Else, ValidatorError>(Else { body })
                })
                .transpose()?;

            Ok(Ast::Condition {
                main: validated_main,
                elif: validated_elif,
                other: validated_other,
            })
        }
        Statement::While { condition, body } => {
            let condition_type = get_type(&condition, ctx)?;
            if condition_type != Type::Bool {
                return Err(ValidatorError::TypeMismatch {
                    expected: Type::Bool,
                    found: condition_type,
                });
            }
            let condition = validate_expr(*condition, ctx)?;
            let mut validated_body = Vec::new();
            for stmt in body {
                validated_body.push(validate_statement(stmt, ctx)?);
            }
            Ok(Ast::While {
                condition: Box::new(condition),
                body: validated_body,
            })
        }
        Statement::Expr(expr) => {
            let expr = validate_expr(expr, ctx)?;
            Ok(Ast::Expr(expr))
        }
        Statement::EnumDecl { .. }
        | Statement::FunctionDef { .. }
        | Statement::StructDef { .. }
        | Statement::UnionType { .. }
        | Statement::Match { .. }
        | Statement::Loop { .. }
        | Statement::ExternalFunctionDef { .. } => Ok(Ast::Expr(ValidatorExpr::Void)),
    }
}
