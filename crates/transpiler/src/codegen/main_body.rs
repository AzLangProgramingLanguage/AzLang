use parser::ast::{Expr, Program};

use crate::{TranspileContext, transpile::transpile_expr};

pub fn generate_main_body<'a>(program: &'a Program<'a>, ctx: &mut TranspileContext<'a>) -> String {
    let mut body = String::new();

    for expr in &program.expressions {
        match expr {
            Expr::FunctionDef { .. } | Expr::UnionType { .. } | Expr::StructDef { .. } => {
                continue;
            }
            _ => {}
        }

        let mut line = transpile_expr(expr.clone(), ctx); /* TODO: Yersiz Clone */

        let needs_semicolon = matches!(
            expr,
            Expr::Assignment { .. }
                | Expr::Break
                | Expr::Continue
                | Expr::BuiltInCall { .. }
                | Expr::Decl { .. }
                | Expr::Call { .. }
                | Expr::VariableRef { .. }
                | Expr::Index { .. }
                | Expr::BinaryOp { .. }
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
