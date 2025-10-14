use crate::{
    Runner, dd,
    parser::ast::{Expr, Type},
    runner::{Variable, eval::eval, runner_interpretator::runner_interpretator},
};

pub fn handle_number_call<'a>(name: &str, s: i64, ctx: &mut Runner<'a>) -> Option<Expr<'a>> {
    let method_body = {
        let uniontype = ctx.uniontypes.get("Ədəd")?;
        let method = uniontype.methods.iter().find(|m| m.name == name)?.clone();
        method.body.clone()
    };

    ctx.variables.insert(
        "self".to_string(),
        Variable {
            value: Expr::Number(s),
            typ: Type::Istifadeci("Ədəd".into()),
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
                "TipVer" => {
                    return Some(Expr::String("Ədəd".into(), false));
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
