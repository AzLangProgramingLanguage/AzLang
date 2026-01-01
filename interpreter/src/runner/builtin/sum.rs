use crate::runner::Runner;
use crate::runner::runner::runner_interpretator;
use parser::ast::Expr;

pub fn sum<'a>(args: Vec<Expr<'a>>, ctx: &mut Runner<'a>) -> Expr<'a> {
    let mut result = Expr::Number(0);
    for arg in args {
        let runned_arg = runner_interpretator(ctx, arg);
        match runned_arg {
            Expr::Number(n) => {
                result = match result {
                    Expr::Number(c) => Expr::Number(n + c),
                    Expr::Float(c) => Expr::Float(n as f64 + c),
                    _ => result,
                }
            }
            Expr::Float(n) => {
                result = match result {
                    Expr::Number(c) => Expr::Float(n + c as f64),
                    Expr::Float(c) => Expr::Float(n + c),
                    _ => result,
                }
            }
            _ => {}
        }
    }
    result
}
