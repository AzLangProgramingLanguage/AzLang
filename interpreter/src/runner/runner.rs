use std::rc::Rc;

use super::Runner;
use crate::runner::{
    FunctionDef, Method, StructDef, UnionType, Variable,
    builtin::print::print_interpreter,
    eval::eval,
    handlers::{
        list_handler::handle_list_call, number_handler::handle_number_call,
        string_handler::handle_string_call,
    },
    helpers::exec_block,
};
use parser::{
    ast::Expr,
    shared_ast::{BuiltInFunction, Type},
};

pub fn runner_interpretator<'a>(ctx: &mut Runner<'a>, expr: Expr<'a>) -> Expr<'a> {
    match expr {
        Expr::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            ctx.variables.insert(
                name.to_string(),
                Variable {
                    value: eval(&value, ctx),
                    typ: (*typ).clone(), /* TODO: Burada cloneye eytiyac yoxdur/ */
                    is_mutable,
                },
            );
            Expr::Void
        }
        Expr::BuiltInCall {
            function,
            args,
            return_type,
        } => match function {
            BuiltInFunction::Print => {
                let arg = runner_interpretator(ctx, args[0].clone());
                let output = print_interpreter(&arg, ctx);
                println!("{}", output);
                Expr::Void
            }
            _ => Expr::Void,
        },
        Expr::String(s) => Expr::String(s),
        Expr::Number(n) => Expr::Number(n),
        Expr::List(l) => Expr::List(l),
        Expr::Bool(b) => Expr::Bool(b),
        Expr::DynamicString(s) => Expr::DynamicString(s),
        Expr::Void => Expr::Void,
        _ => Expr::Void,
    }
}
