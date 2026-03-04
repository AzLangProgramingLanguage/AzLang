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

pub fn builthin_call_runner(
    ctx: &mut Runner,
    function: BuiltInFunction,
    mut args: Vec<Expr>,
    return_type: Type,
) -> Expr {
    match function {
        BuiltInFunction::Print => {
            let output = print_interpreter(runner_interpretator(ctx, args.remove(0)), ctx);

            println!("{}", output);
            Expr::Void
        }
        BuiltInFunction::LastWord => {
            let output = print_interpreter(runner_interpretator(ctx, args.remove(0)), ctx);

            println!("{}", output);
            std::process::exit(1);
        }
        BuiltInFunction::Len => {
            let mut arg = runner_interpretator(ctx, args.remove(0));
            //TODO: Berbat bir kod  burada bunun yerine Value ENumu yarat
            match &arg {
                Expr::VariableRef { name, symbol } => {
                    arg = runner_interpretator(ctx, arg);
                }
                _ => {}
            }

            match arg {
                Expr::List(s) => {
                    return Expr::Number(s.len() as i64);
                }
                _ => {
                    println!("Salam {:?}", arg);
                    std::process::exit(1);

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
