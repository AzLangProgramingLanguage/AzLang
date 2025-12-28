use std::rc::Rc;

use parser::{ast::Expr, shared_ast::Type};

use crate::runner::{Runner, runner::runner_interpretator};

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
                }
            }
        }
    }
    Expr::Void
}
