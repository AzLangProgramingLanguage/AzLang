use parser::typed_ast::{CompiledProgram, TypedExpr};

use crate::transpiler::{
    TranspileContext, helper::transpile_function_def, struct_def::transpile_struct_def,
    union_def::transpile_union_def,
};

pub fn generate_top_level_defs<'a>(
    program: &'a CompiledProgram<'a>,
    ctx: &mut TranspileContext<'a>,
) -> String {
    let mut code = String::new();

    for expr in &program.expressions {
        match expr {
            TypedExpr::FunctionDef {
                name,
                transpiled_name: _,
                params,
                body,
                return_type,
                is_allocator,
            } => {
                let def = transpile_function_def(
                    name,
                    params,
                    body,
                    return_type,
                    None,
                    ctx,
                    is_allocator,
                );
                code.push_str(&def);
                code.push_str("\n\n");
            }
            TypedExpr::UnionType {
                name,
                transpiled_name,
                fields,
                methods,
            } => {
                let union = transpile_union_def(
                    name,
                    transpiled_name.as_deref().unwrap_or(name),
                    fields,
                    methods,
                    ctx,
                );
                code.push_str(&union);
                code.push_str("\n\n");
            }
            TypedExpr::StructDef {
                name,
                transpiled_name,
                fields,
                methods,
            } => {
                let struct_def = transpile_struct_def(
                    name,
                    transpiled_name.as_deref().unwrap_or(name),
                    fields,
                    methods,
                    ctx,
                );
                code.push_str(&struct_def);
                code.push_str("\n\n");
            }
            _ => {}
        }
    }
    code
}
