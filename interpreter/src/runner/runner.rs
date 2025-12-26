use std::{mem, rc::Rc};

use super::Runner;
use crate::runner::{
    FunctionDef, Variable, binary_op::binary_op_runner, builtin::builthin_call_runner,
};

use parser::{
    ast::Expr,
    shared_ast::{BuiltInFunction, Type},
};

pub fn runner_interpretator<'a>(ctx: &mut Runner<'a>, expr: Expr<'a>) -> Expr<'a> {
    match expr {
        Expr::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            let new_value: Expr<'a> = runner_interpretator(ctx, *value);
            ctx.variables.insert(
                name.to_string(),
                Variable {
                    value: Rc::new(new_value),
                    typ: (*typ).clone(),
                    is_mutable: is_mutable,
                },
            );
            Expr::Void
        }

        Expr::FunctionDef {
            name,
            params,
            body,
            return_type,
        } => {
            let body_rc = Rc::new(body);
            let params_rc = Rc::new(params);
            ctx.functions.insert(
                name.to_string(),
                FunctionDef {
                    params: params_rc,
                    body: body_rc,
                    return_type: return_type.unwrap_or(Type::Any),
                },
            );
            Expr::Void
        }

        Expr::Assignment {
            name,
            value,
            symbol,
        } => {
            let new_value: Expr<'a> = runner_interpretator(ctx, *value);
            if let Some(var) = ctx.variables.get_mut(&name.to_string()) {
                var.value = Rc::new(new_value);
            }

            Expr::Void
        }
        Expr::BinaryOp {
            left,
            right,
            op,
            return_type,
        } => binary_op_runner(ctx, left, right, op, return_type),

        Expr::BuiltInCall {
            function,
            mut args,
            return_type,
        } => builthin_call_runner(ctx, function, args, return_type),
        Expr::VariableRef { name, symbol } => {
            if let Some(var) = ctx.variables.get(&name.to_string()) {
                return var.value.as_ref().clone();
            }
            Expr::Void
        }
        Expr::String(s) => Expr::String(s),
        Expr::Number(n) => Expr::Number(n),
        Expr::List(l) => Expr::List(l.clone()),
        Expr::Bool(b) => Expr::Bool(b),
        Expr::DynamicString(s) => Expr::DynamicString(s.clone()),
        Expr::Void => Expr::Void,
        _ => Expr::Void,
    }
}
