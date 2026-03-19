use std::{os::unix::process, rc::Rc};

use parser::{
    ast::{Expr, Parameter, Symbol},
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
            panic!(" Burası hazır deyil {expr:?}");
        }
        None => {
            match &*name {
                Expr::VariableRef {
                    name: func_name,
                    symbol,
                } => {
                    if let Some(function) = ctx.functions.get(func_name) {
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
                                    let result = runner_interpretator(ctx, *s);
                                    ctx.current_return = result.clone();
                                    return result;
                                }
                                _ => {
                                    runner_interpretator(ctx, expr);
                                }
                            }
                        } //TODO: Burada Mütleq deyerleri temizlemek lazımdır.
                    } else {
                        panic!("Bele bir funksiya yoxdur. ");
                    }
                }
                Expr::Call {
                    target,
                    name,
                    args: inner_args,
                    returned_type: return_type,
                } => {
                    let expr = function_call(
                        ctx,
                        target.clone(),
                        name.clone(),
                        inner_args.clone(),
                        returned_type.clone(),
                    );
                    match expr.clone() {
                        Expr::VariableRef {
                            name,
                            symbol:
                                Some(Symbol {
                                    typ: Type::Function,
                                    ..
                                }),
                        } => {
                            function_call(ctx, None, Box::new(expr), args.clone(), returned_type);
                        }
                        _ => {
                            return expr;
                        }
                    }
                }
                _ => {
                    panic!("Buraya gəlməməliydi.")
                }
            }
        }
    }

    ctx.current_return.clone()
}
