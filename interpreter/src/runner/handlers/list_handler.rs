use parser::ast::{Expr, Type};

use crate::runner::eval::eval;
use crate::runner::runner::runner_interpretator;
use crate::runner::{Runner, Variable};

pub fn handle_list_call<'a>(
    name: &str,
    s: Vec<Expr<'a>>,
    variable_name: Option<String>,
    args: Vec<Expr<'a>>,
    ctx: &mut Runner<'a>,
) -> Option<Expr<'a>> {
    let method_body = {
        let uniontype = ctx.uniontypes.get("Siyahı")?;
        let method = uniontype.methods.iter().find(|m| m.name == name)?;
        method.body.clone()
    };

    ctx.variables.insert(
        "self".to_string(),
        Variable {
            value: Expr::List(s.clone()),
            typ: Type::Istifadeci("Siyahı".into()),
            is_mutable: false,
        },
    );

    for expr in method_body {
        match expr {
            Expr::Return(value) => {
                let val = eval(&value, ctx);
                ctx.variables.remove("self");
                return Some(val);
            }
            Expr::Comment(c) if c.trim() == "Burasını Sistem Qərar Versin" => match name {
                "uzunluq" => {
                    let strr = s.len();
                    return Some(Expr::Number(strr.try_into().unwrap()));
                }
                "tipi" => {
                    return Some(Expr::String("Siyahı".into(), false));
                }
                "sil" => match args.get(0) {
                    Some(arg) => {
                        let arg = eval(&*arg, ctx);
                        let arg = match arg {
                            Expr::Number(n) => n,
                            _ => return None,
                        };
                        let mut list = s;
                        list.remove(arg.try_into().unwrap());
                        return None;
                    }
                    None => return None,
                },
                "sondansil" => {
                    let mut list = s;
                    list.remove(list.len() - 1);
                    match variable_name {
                        Some(name) => {
                            ctx.variables.insert(
                                name,
                                Variable {
                                    value: Expr::List(list),
                                    typ: Type::Istifadeci("Siyahı".into()),
                                    is_mutable: false,
                                },
                            );
                        }
                        None => {}
                    }
                    return None;
                }
                "əlavə_et" => {
                    /* TOOD: Burası tamamlanmayıb. */
                    let mut list = ctx.variables.get("self").unwrap().value.clone();
                    return Some(list);
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
