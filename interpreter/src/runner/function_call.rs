use std::{os::unix::process, rc::Rc};

use parser::{
    ast::{Expr, Parameter},
    shared_ast::Type,
};

use crate::runner::{Runner, Variable, runner::runner_interpretator};

pub fn function_call(
    ctx: &mut Runner,
    target: Option<Box<Expr>>,
    name: Box<Expr>,
    args: Vec<Expr>,
    returned_type: Option<Type>,
) -> Expr {
    match target {
        Some(expr) => {
            println!("{expr:?}");
            panic!(" Burası hazır deyil {expr:?}");
        }
        None => {
            if let Some(function) = ctx.functions.get(&name) {
                let body_rc: Rc<Vec<Expr>> = Rc::clone(&function.body);
                let params: Rc<Vec<Parameter>> = Rc::clone(&function.params);
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
                        Expr::Return(s) => {
                            return runner_interpretator(ctx, *s);
                        }
                        _ => {
                            runner_interpretator(ctx, expr);
                        }
                    }
                } //TODO: Burada Mütleq deyerleri temizlemek lazımdır.
            } else {
                dbg!(2); //TODO: Burası funksiyada yok
                std::process::exit(1);
            }
        }
    }
    ctx.current_return.clone()
}
