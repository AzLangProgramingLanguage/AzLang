use parser::{
    ast::Expr,
    shared_ast::{BuiltInFunction, Type},
};

use crate::runner::{
    Runner,
    builtin::{input::input, sum::sum},
    runner::{Value, runner_interpretator},
};

mod input;
mod sum;

pub fn builthin_call_runner(
    function: BuiltInFunction,
    mut args: Vec<Value>,
    return_type: Type,
) -> Value {
    match function {
        BuiltInFunction::Print => {
            println!("{}", args.remove(0));
            Value::Void
        }
        BuiltInFunction::LastWord => {
            println!("{}", args.remove(0));
            std::process::exit(1);
        }
        BuiltInFunction::Len => {
            let arg = args.remove(0);

            match arg {
                Value::List(l) => Value::Number(l.len() as i64),
                Value::String(s) => Value::Number(s.len() as i64),
                _ => Value::Number(0),
            }
        }
        BuiltInFunction::Input => {
            println!("{}", args.remove(0));
            input()
        }

        BuiltInFunction::Sum => sum(args),
        _ => Value::Void,
    }
}
