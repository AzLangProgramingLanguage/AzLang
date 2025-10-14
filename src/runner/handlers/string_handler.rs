use crate::{
    Runner, dd,
    parser::ast::{Expr, Type},
    runner::{Variable, eval::eval, runner_interpretator::runner_interpretator},
};

pub fn handle_string_call<'a>(name: &str, s: &'a str, ctx: &mut Runner<'a>) -> Option<Expr<'a>> {
    let method_body = {
        let uniontype = ctx.uniontypes.get("Yazı")?;
        let method = uniontype.methods.iter().find(|m| m.name == name)?.clone();
        method.body.clone()
    };

    ctx.variables.insert(
        "self".to_string(),
        Variable {
            value: Expr::String(s, false),
            typ: Type::Istifadeci("Yazı".into()),
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
                "tərs" => {
                    let strr = s.chars().rev().collect::<String>();
                    return Some(Expr::DynamicString(strr.into()));
                }
                "böyüt" => {
                    let strr = s.to_uppercase();
                    return Some(Expr::DynamicString(strr.into()));
                }
                "kiçilt" => {
                    let strr = s.to_lowercase();
                    return Some(Expr::DynamicString(strr.into()));
                }
                "qırx" => {
                    let strr = s.trim();
                    return Some(Expr::String(strr.into(), false));
                }
                "uzunluq" => {
                    let strr = s.len();
                    return Some(Expr::Number(strr.try_into().unwrap()));
                }
                "TipVer" => {
                    return Some(Expr::String("Yazı".into(), false));
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
