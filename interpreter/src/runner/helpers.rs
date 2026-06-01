use crate::runner::{Runner, runner::runner_interpretator};
use validator::ast::Ast;

pub fn run_body(ctx: &mut Runner, body: Vec<Ast>) {
    for expr in body {
        runner_interpretator(ctx, expr);
    }
}
