use crate::{
    parser::ast::{Expr, Program},
    transpiler::{
        TranspileContext, helpers::transpile_function_def, union_def::transpile_union_def,
    },
};

pub fn generate_top_level_defs<'a>(
    program: &Program<'a>,
    ctx: &mut TranspileContext<'a>,
) -> String {
    let mut code = String::new();

    for expr in &program.expressions {
        match expr {
            Expr::FunctionDef {
                name,
                params,
                body,
                return_type,
            } => {
                let def = transpile_function_def(name, params, body, return_type, None, ctx);
                code.push_str(&def);
                code.push_str("\n\n");
            }
            Expr::UnionType {
                name,
                fields,
                methods,
            } => {
                let union = transpile_union_def(name, fields, methods, ctx);
                code.push_str(&union);
                code.push_str("\n\n");
            }
            _ => {}
        }
    }
    code
}
