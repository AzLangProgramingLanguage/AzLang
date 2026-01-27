use super::Runner;
use crate::runner::{
    FunctionDef, Variable, binary_op::binary_op_runner, builtin::builthin_call_runner,
    function_call::function_call, helpers::run_body,
};
use std::rc::Rc;

use parser::{ast::Expr, shared_ast::Type};

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
                    typ,
                    is_mutable,
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
                name,
                FunctionDef {
                    params: params_rc,
                    body: body_rc,
                    return_type: return_type.unwrap_or(Type::Any),
                },
            );
    
            Expr::Void
        }
        Expr::Return(value) => {
            ctx.current_return = runner_interpretator(ctx, *value);
            Expr::Void
        }
        Expr::Call {
            target,
            name,
            args,
            returned_type,
        } => function_call(ctx, target, name, args, returned_type),
        Expr::Assignment { name, value, .. } => {
            let new_value: Expr<'a> = runner_interpretator(ctx, *value);
            if let Some(var) = ctx.variables.get_mut(&name.to_string()) {
                var.value = Rc::new(new_value);
            }

            Expr::Void
        }
        Expr::Condition { main, elif, other } => {
            if matches!(runner_interpretator(ctx, *main.condition), Expr::Bool(true)) {
                run_body(ctx, main.body);
                return Expr::Void;
            }
            for branch in elif {
                if matches!(
                    runner_interpretator(ctx, *branch.condition),
                    Expr::Bool(true)
                ) {
                    run_body(ctx, branch.body);
                    return Expr::Void;
                }
            }
            if let Some(other) = other {
                run_body(ctx, other.body);
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
            args,
            return_type,
        } => builthin_call_runner(ctx, function, args, return_type),
        Expr::VariableRef { name, .. } => {
            if let Some(var) = ctx.variables.get(&name.to_string()) {
                return var.value.as_ref().clone();
            }
            Expr::Void
        }
        other => other,
    }
}
