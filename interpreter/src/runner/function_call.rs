use std::rc::Rc;

use parser::{ast::Expr, shared_ast::Type};

use crate::runner::{Runner, Variable, runner::runner_interpretator};

pub fn function_call<'a>(
    ctx: &mut Runner<'a>,
    target: Option<Box<Expr<'a>>>,
    name: String,
    args: Vec<Expr<'a>>,
    returned_type: Option<Type<'a>>,
) -> Expr<'a> {
    match target {
        Some(expr) => {
            println!("{expr:?}");
            std::process::exit(1); /*TODO: Burası method calldir */
        }
        None => {
            if let Some(function) = ctx.functions.get(&name) {
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

                    runner_interpretator(ctx, expr);
                } //TODO: Burada Mütleq deyerleri temizlemek lazımdır.
            }
            else{
                dbg!(2);//TODO: Burası funksiyada yok
                std::process::exit(1);
            }
        }
    }
    ctx.current_return.clone()
}
