use color_eyre::eyre::{Result, eyre};

use crate::{
    parser::ast::{Expr, Symbol, Type},
    translations::validator_messages::ValidatorError,
    validator::{FunctionInfo, ValidatorContext, helpers::validate_decl},
};

pub fn validate_expr<'a>(
    expr: &'a mut Expr<'a>,
    ctx: &mut ValidatorContext<'a>,
    log: &mut dyn FnMut(&str),
) -> Result<(), ValidatorError<'a>> {
    match expr {
        Expr::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            log(&format!("✅ Declarasiya yaradılır: {name}"));
            let type_ = validate_decl(name, typ, value, is_mutable, ctx, log)?;
            *typ = Some(type_);
            // `cannot assign to `*typ` because it is borrowed
            // `*typ` is assigned to here but it was already borrowed
        }

        Expr::StructDef {
            name,
            fields,
            methods,
        } => {
            log(&format!("✅ Struktur yaradılır: {name}"));

            if ctx.struct_defs.contains_key(*name) {
                return Err(ValidatorError::DuplicateStruct(name.to_string()));
            }
            /*
                       ctx.struct_defs.insert(
                           (*name).to_string(),
                           StructDecl {
                               name: (*name).to_string(),
                               fields: fields.clone(),
                               methods: methods.clone(),
                           },
                       );
            */
            ctx.current_struct = Some((*name).to_string());
            for (method_name, params, body, _) in methods {
                log(&format!("➡️ Method yoxlanılır: {method_name}"));
                ctx.push_scope();
                for param in params {
                    /*  ctx.declare_variable(
                        param.name.to_string(),
                        /*                         Symbol::Variable(param.typ.clone()),
                         */
                    ); */
                }
                for expr in body {
                    validate_expr(expr, ctx, log)?;
                }
                ctx.pop_scope();
            }
            ctx.current_struct = None;
        }

        Expr::EnumDecl(enum_decl) => {
            let name = &enum_decl.name;
            log(&format!("✅ Enum yaradılır: {name}"));

            /*          if ctx.enum_defs.contains_key(name) {
                return Err(eyre!("❌ Enum təkrarlanır: {name}"));
            } */

            /*             ctx.enum_defs.insert(name.to_string(), enum_decl.clone());
             */
        }

        Expr::FunctionDef {
            name,
            params,
            body,
            return_type,
            /*             parent,
             */
        } => {
            log(&format!("✅ Funksiya yaradılır: {name}"));

            if ctx.functions.contains_key(*name) {
                return Err(ValidatorError::DuplicateFunction(name));
            }

            /*    ctx.functions.insert(
                           (*name).to_string(),
                           FunctionInfo {
                               name: (*name).to_string(),
                               parameters: params.clone(),
                               return_type: return_type.clone(),
                               body: Some(body.clone()),
                               scope_level: ctx.scopes.len(),
                               parent: parent.clone(),
                           },
                       );
            */
            ctx.current_function = Some((*name).to_string());
            ctx.push_scope();
            for param in params {
                /*                 ctx.declare_variable(param.name.to_string(), Symbol::Variable(param.typ.clone()));
                 */
            }
            for expr in body {
                validate_expr(expr, ctx, log)?;
            }
            ctx.pop_scope();
            ctx.current_function = None;
        }

        Expr::Assignment { name, value, .. } => {
            log(&format!("➡️ Mənimsətmə yoxlanılır: {name}"));
            if ctx.lookup_variable(name).is_none() {
                return Err(ValidatorError::UndefinedVariable(name));
            }
            validate_expr(value, ctx, log)?;
        }

        Expr::VariableRef { name, .. } => {
            if ctx.lookup_variable(name).is_none() {
                return Err(ValidatorError::UndefinedVariable(name));
            }
        }

        Expr::BinaryOp { left, right, .. } => {
            validate_expr(left, ctx, log)?;
            validate_expr(right, ctx, log)?;
        }

        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            validate_expr(condition, ctx, log)?;
            ctx.push_scope();
            for expr in then_branch {
                validate_expr(expr, ctx, log)?;
            }
            ctx.pop_scope();
            for expr in else_branch {
                validate_expr(expr, ctx, log)?;
            }
        }

        Expr::Loop {
            var_name,
            iterable,
            body,
        } => {
            validate_expr(iterable, ctx, log)?;
            ctx.push_scope();
            /*             ctx.declare_variable(var_name.to_string(), Symbol::Variable(Type::Any));
             */
            for expr in body {
                validate_expr(expr, ctx, log)?;
            }
            ctx.pop_scope();
        }

        Expr::Return(inner) => {
            validate_expr(inner, ctx, log)?;
        }

        Expr::Call {
            target, name, args, ..
        } => {
            log(&format!("➡️ Call yoxlanılır: {name}"));
            if let Some(t) = target {
                validate_expr(t, ctx, log)?;
            }
            for arg in args {
                validate_expr(arg, ctx, log)?;
            }
        }
        _ => {
            return Err(ValidatorError::UnknownExpression(expr));
        }
    }
    Ok(())
}
