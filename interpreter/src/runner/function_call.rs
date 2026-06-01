use parser::shared_ast::Type;

use validator::ast::{Ast, Expr};

use crate::runner::{
    Runner, Variable,
    runner::{Value, get_primitive_value, runner_interpretator},
};

pub fn function_call(
    ctx: &mut Runner,
    _target: Option<Box<Expr>>,
    name: Box<Expr>,
    args: Vec<Expr>,
    _returned_type: Option<Type>,
) -> Value {
    match *name {
        Expr::VariableRef { name, symbol: _ } => {
            if let Some(function) = ctx.functions.get(&name).cloned() {
                for (index, param) in function.params.iter().enumerate() {
                    let variable = get_primitive_value(ctx, args[index].clone(), None);
                    ctx.variables.insert(
                        param.name.clone(),
                        Variable {
                            value: variable,
                        },
                    );
                }
                for stmt in function.body.clone() {
                    match stmt {
                        Ast::Expr(Expr::Return(e)) => {
                            return get_primitive_value(ctx, *e, Some(function.return_type));
                        }
                        _ => runner_interpretator(ctx, stmt),
                    }
                }
                Value::Void
            } else {
                panic!("{name} function not found")
            }
        }
        other => todo!("{other:?} not implemented yet"),
    }
}
