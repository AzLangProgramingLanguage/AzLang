use parser::ast::{Expr, Program};

use crate::transpiler::{
    TranspileContext,
    definition::{
        function_def::transpile_function_def, struct_def::transpile_struct_def,
        union_def::transpile_union_def,
    },
};

pub fn generate_top_level_defs<'a>(
    program: &'a Program<'a>,
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
                let def =
                    transpile_function_def(name, params, body, return_type, None, ctx, &false);
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
            Expr::StructDef {
                name,
                fields,
                methods,
            } => {
                let struct_def = transpile_struct_def(name, fields, methods, ctx);
                code.push_str(&struct_def);
                code.push_str("\n\n");
            }
            _ => {}
        }
    }
    code
}
