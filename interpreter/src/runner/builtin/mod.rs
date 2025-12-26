use parser::{
    ast::Expr,
    function,
    shared_ast::{BuiltInFunction, Type},
};

use crate::runner::{Runner, builtin::print::print_interpreter, runner::runner_interpretator};

mod print;
mod sum;

pub fn builthin_call_runner<'a>(
    ctx: &mut Runner<'a>,
    function: BuiltInFunction,
    mut args: Vec<Expr<'a>>,
    return_type: Type<'a>,
) -> Expr<'a> {
    match function {
        BuiltInFunction::Print => {
            println!(
                "{}",
                print_interpreter(runner_interpretator(ctx, args.remove(0)), ctx)
            );
            Expr::Void
        }
        BuiltInFunction::LastWord => {
            println!(
                "{}",
                print_interpreter(runner_interpretator(ctx, args.remove(0)), ctx)
            );
            std::process::exit(1);
            Expr::Void
        }
        BuiltInFunction::Sum => Expr::Void,
        BuiltInFunction::Input => Expr::Void,
        _ => Expr::Void,
    }
}
