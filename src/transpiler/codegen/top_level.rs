use crate::{
    parser::ast::{Expr, Program},
    transpiler::TranspileContext,
};

pub fn generate_top_level_defs(program: &Program, ctx: &mut TranspileContext) -> String {
    let mut code = String::new();

    for expr in &program.expressions {
        if let Expr::FunctionDef {
            name,
            params,
            body,
            return_type,
        } = expr
        {
            /*    let def = transpile_function_def(name, params, body, return_type, parent, ctx)?;
            code.push_str(&def);
            code.push_str("\n\n"); */
        }
    }

    code
}
