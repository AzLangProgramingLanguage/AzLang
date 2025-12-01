use parser::typed_ast::{CompiledProgram, TypedExpr};

use crate::transpiler::{TranspileContext, transpile::transpile_expr};

pub fn generate_main_body<'a>(
    program: &'a CompiledProgram<'a>,
    ctx: &mut TranspileContext<'a>,
) -> String {
    let mut body = String::new();

    for expr in &program.expressions {
        match expr {
            TypedExpr::FunctionDef { .. }
            | TypedExpr::UnionType { .. }
            | TypedExpr::StructDef { .. } => {
                continue;
            }
            _ => {}
        }

        let mut line = transpile_expr(expr, ctx);

        let needs_semicolon = matches!(
            expr,
            TypedExpr::Assignment { .. }
                | TypedExpr::Break
                | TypedExpr::Continue
                | TypedExpr::BuiltInCall { .. }
                | TypedExpr::Decl { .. }
                | TypedExpr::Call { .. }
                | TypedExpr::VariableRef { .. }
                | TypedExpr::Index { .. }
                | TypedExpr::BinaryOp { .. }
        );

        let line = if needs_semicolon && !line.trim_end().ends_with(';') {
            line.push(';');
            line
        } else {
            line.to_string()
        };
        body.push_str("   ");
        body.push_str(&line);
        body.push('\n');
    }

    body
}
