use crate::{
    parser::ast::{Expr, Program},
    transpiler::{TranspileContext, helpers::transpile_function_def},
};

pub fn generate_top_level_defs<'a>(
    program: &Program<'a>,
    ctx: &mut TranspileContext<'a>,
) -> String {
    let mut code = String::new();

    for expr in &program.expressions {
        if let Expr::FunctionDef {
            name,
            params,
            body,
            return_type,
        } = expr
        {
            let def = transpile_function_def(name, params, body, return_type, None, ctx);
            code.push_str(&def);
            code.push_str("\n\n");
        }
    }

    code
}
