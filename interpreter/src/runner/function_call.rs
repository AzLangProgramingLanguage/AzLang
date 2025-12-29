use std::rc::Rc;

use parser::{ast::Expr, shared_ast::Type};

use crate::runner::{Runner, Variable, runner::runner_interpretator};

pub fn function_call<'a>(
    ctx: &mut Runner<'a>,
    target: Option<Box<Expr<'a>>>,
    name: &'a str,
    args: Vec<Expr<'a>>,
    returned_type: Option<Type<'a>>,
) -> Expr<'a> {
    match target {
        Some(expr) => {
            println!("{expr:?}");
            std::process::exit(1);
        }
        None => {
            if let Some(function) = ctx.functions.get(name) {
                let body_rc = Rc::clone(&function.body);
                let params = Rc::clone(&function.params);
                for i in 0..params.len() {
                    ctx.variables.insert(
                        params[i].name.clone(),
                        Variable {
                            value: Rc::new(args[i].clone()),
                            typ: Rc::new(params[i].typ.clone()),
                            is_mutable: params[i].is_mutable,
                        },
                    );
                }
                for i in 0..body_rc.len() {
                    let expr = body_rc[i].clone();
                    match expr {
                        Expr::Return(return_expr) => {
                            return runner_interpretator(ctx, *return_expr);
                        }
                        _ => {
                            runner_interpretator(ctx, expr);
                        }
                    }
                } //TODO: Burada Mütleq deyerleri temizlemek lazımdır. 
            }
        }
    }
    Expr::Void
}
