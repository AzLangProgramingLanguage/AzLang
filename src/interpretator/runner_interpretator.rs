use crate::{
    dd,
    interpretator::{Variable, builtin},
    parser::ast::{BuiltInFunction, Expr},
};

use super::InterPretator;

pub fn runner_interpreator<'a>(ctx: &mut InterPretator<'a>, expr: Expr<'a>) {
    match expr {
        Expr::Decl {
            name,
            transpiled_name,
            typ,
            is_mutable,
            value,
        } => {
            if let Some(typ) = typ {
                ctx.variables.insert(
                    name.to_string(),
                    Variable {
                        value,
                        typ,
                        is_mutable,
                    },
                );
            }
        }
        Expr::BuiltInCall {
            function,
            args,
            return_type,
        } => match function {
            BuiltInFunction::Print => {
                builtin::print::print_interpreter(&args[0], ctx);
            }
            BuiltInFunction::Input => {
                builtin::print::exporter(&args[0], ctx);
                std::io::stdin().read_line(&mut String::new()).unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}
