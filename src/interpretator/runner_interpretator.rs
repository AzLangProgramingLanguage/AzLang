use crate::{
    dd,
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
                ctx.variables.insert(name.to_string(), (typ, value));
            }
        }
        Expr::BuiltInCall {
            function,
            args,
            return_type,
        } => match function {
            BuiltInFunction::Print => match args[0] {
                Expr::String(s, _) => println!("{}", s),
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }
}
