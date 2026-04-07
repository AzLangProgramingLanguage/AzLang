use crate::runner::{Runner, runner::runner_interpretator};
use parser::ast::{Expr, Statement};

/*
 *

* pub fn exec_block<'a>(ctx: &mut Runner<'a>, body: Vec<Expr<'a>>) -> Option<Expr<'a>> {
    for expr in body {
        match expr {
            Expr::Return(value) => return Some(eval(&*value, ctx)),
            _ => {
                if let Some(val) = runner_interpretator(ctx, expr) {
                    return Some(val);
                }
            }
        }
    }
    None
}
*/

pub fn run_body<'a>(ctx: &mut Runner, body: Vec<Statement>) {
    for expr in body {
        runner_interpretator(ctx, expr);
    }
}
