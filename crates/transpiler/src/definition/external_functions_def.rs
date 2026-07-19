use parser::{ast::Parameter, shared_ast::Type};
use std::fmt::Write;
use validator::ast::ExternalFunctionDef;

use crate::TranspileContext;
pub fn transpile_external_functions(
    ctx: &mut TranspileContext,
    externalfn: &ExternalFunctionDef,
    buff: &mut String,
) {
    write!(buff, "extern fn {}", externalfn.name).unwrap();
    transpile_params_for_external_functions(ctx, &externalfn.params, buff);
    write!(buff, " {};", c_type_formatter(ctx, &externalfn.return_typ)).unwrap();
}
fn transpile_params_for_external_functions(
    ctx: &mut TranspileContext,
    args: &Vec<Parameter>,
    buff: &mut String,
) {
    write!(buff, "(").unwrap();
    for arg in args {
        write!(buff, "{}:", arg.name).unwrap();

        write!(buff, "{}", c_type_formatter(ctx, &arg.typ)).unwrap();
        buff.push(',');
    }
    buff.pop();
    buff.push(')');
}
fn c_type_formatter(ctx: &mut TranspileContext, typ: &Type) -> &'static str {
    match typ {
        Type::Integer => "c_int",
        Type::Natural => "c_int",

        Type::Bool => "c_bool",
        _ => {
            ctx.has_external_any = true;
            "*const ValueType"
        }
    }
}
