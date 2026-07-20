use crate::{
    FunctionDef, FunctionInfo, TranspileContext,
    helper::{is_semicolon_needed, map_typ},
    transpile::transpile_stmt,
};
use std::fmt::Write;
use validator::ast::Function;

pub fn transpile_function_def(ctx: &mut TranspileContext, func: Function, buff: &mut String) {
    write!(buff, "fn {}( ", func.name);
    for (i, param) in func.params.iter().enumerate() {
        if i > 0 {
            buff.push(',');
        }
        write!(buff, "{}: {}", param.name, map_typ(&param.typ));
    }
    write!(buff, ") {} {{ ", map_typ(&func.return_typ));
    for body in func.body {
        transpile_stmt(body, ctx);
    }
    buff.push('}');

    ctx.functions.insert(
        func.name,
        FunctionInfo {
            params: func.params,
            return_typ: func.return_typ,
        },
    );
}
