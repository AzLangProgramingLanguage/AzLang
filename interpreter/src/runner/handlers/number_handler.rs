use parser::ast::Expr;
use parser::shared_ast::Type;

use crate::runner::eval::eval;
use crate::runner::runner::runner_interpretator;
use crate::runner::{Runner, Variable};

pub fn handle_number_call<'a>(
    name: &str,
    s: i64,
    _args: Vec<Expr<'a>>,
    ctx: &mut Runner<'a>,
) -> Option<Expr<'a>> {
    let method_body = {
        let uniontype = ctx.uniontypes.get("Ədəd")?;
        let method = uniontype.methods.iter().find(|m| m.name == name)?;
        method.body.clone()
    };

    ctx.variables.insert(
        "self".to_string(),
        Variable {
            value: Expr::Number(s),
            typ: Type::User("Ədəd".into()),
            is_mutable: false,
        },
    );

    for expr in method_body {
        match expr {
            Expr::Return(value) => {
                let val = eval(&*value, ctx);
                ctx.variables.remove("self");
                return Some(val);
            }
            Expr::Comment(c) if c.trim() == "Burasını Sistem Qərar Versin" => match name {
                "tipi" => {
                    return Some(Expr::String("Ədəd".into()));
                }
                "yazıya_çevir" => {
                    return Some(Expr::DynamicString(s.to_string().into()));
                }

                _ => {}
            },
            _ => {
                runner_interpretator(ctx, expr.clone());
            }
        }
    }

    ctx.variables.remove("self");
    Some(Expr::Void)
}
