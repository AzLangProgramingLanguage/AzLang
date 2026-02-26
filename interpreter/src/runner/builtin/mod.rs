use parser::{
    ast::Expr,
    shared_ast::{BuiltInFunction, Type},
};

use crate::runner::{
    Runner,
    builtin::{input::input, print::print_interpreter, sum::sum},
    runner::runner_interpretator,
};

mod input;
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
            let output = print_interpreter(runner_interpretator(ctx, args.remove(0)), ctx);
            ctx.output.push_str(&output);

            println!("{}", output);
            Expr::Void
        }
        BuiltInFunction::LastWord => {
            let output = print_interpreter(runner_interpretator(ctx, args.remove(0)), ctx);
            ctx.output.push_str(&output);

            println!("{}", output);
            std::process::exit(1);
        }
        BuiltInFunction::Len => {
            let arg = runner_interpretator(ctx, args.remove(0));

            match arg {
                Expr::List(s) => {
                    return Expr::Number(s.len() as i64);
                }
                _ => {
                    return Expr::Number(0);
                }
            }
        }
        BuiltInFunction::Input => {
            println!(
                "{}",
                print_interpreter(runner_interpretator(ctx, args.remove(0)), ctx)
            );
            input()
        }

        BuiltInFunction::Sum => sum(args, ctx),
        _ => Expr::Void,
    }
}
