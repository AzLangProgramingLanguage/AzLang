use parser::{
    ast::{Expr, Statement, Symbol},
    shared_ast::Type,
};

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
        Expr::VariableRef {
            name,
            symbol:
                Some(Symbol {
                    typ: Type::Function,
                    ..
                }),
        } => {
            if let Some(function) = ctx.functions.get(&name).cloned() {
                for (index, param) in function.params.iter().enumerate() {
                    let variable = get_primitive_value(ctx, args[index].clone(), None);
                    ctx.variables.insert(
                        param.name.clone(),
                        Variable {
                            value: variable,
                            // typ: Rc::new(param.typ.clone()),
                            // is_mutable: param.is_mutable,
                        },
                    );
                }
                for stmt in function.body.clone() {
                    match stmt {
                        Statement::Expr(Expr::Return(e)) => {
                            return get_primitive_value(ctx, *e, function.return_type);
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
