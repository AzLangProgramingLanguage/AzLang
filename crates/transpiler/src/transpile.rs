use parser::ast::Statement;

use crate::{
    TranspileContext,
    helper::{map_typ, transpile_body},
    transpile_expr,
};

pub fn transpile_stmt(stmt: Statement, ctx: &mut TranspileContext) -> String {
    match stmt {
        Statement::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            let typ_str = map_typ(&typ);
            let is_mutable_str = if is_mutable { "var" } else { "const" };

            format!(
                "{is_mutable_str} {name}: {typ_str} = {}",
                transpile_expr(*value, ctx)
            )
        }
        Statement::Condition { main, elif, other } => {
            let mut result = String::from("if(");
            result.push_str(&format!(
                "{}
) {{ {} }}      
      ",
                transpile_expr(*main.condition, ctx),
                transpile_body(main.body, ctx)
            ));

            for el in elif {
                result.push_str(&format!(
                    "else if ({}) {{ {} }} ",
                    transpile_expr(*el.condition, ctx),
                    transpile_body(el.body, ctx)
                ));
            }

            if let Some(_else_) = other {
                result.push_str(&format!("else {{ {} }} ", transpile_body(_else_.body, ctx)));
            }
            result
        }
        Statement::Assignment { name, value } => {
            format!("{name} = {}; ", transpile_expr(*value, ctx))
        }
        Statement::Expr(expr) => transpile_expr(expr, ctx),
        other => todo!("Burası hele hazır deyil {other:?}"),
    }
}

