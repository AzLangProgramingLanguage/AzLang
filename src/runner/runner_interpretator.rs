use std::rc::Rc;

use crate::{
    dd,
    parser::ast::{BuiltInFunction, Expr, Symbol, Type},
    runner::{
        FunctionDef, Method, StructDef, Variable, builtin::print::print_interpreter, helpers,
    },
};

use super::Runner;

pub fn runner_interpretator<'a>(ctx: &mut Runner<'a>, expr: Expr<'a>) {
    match expr {
        Expr::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            let eval_value = {
                println!("Decl: {:?}", value);
                let value = eval(&*value, ctx);
                value
            };

            let type_ref = match typ {
                Some(rc_type) => (*rc_type).clone(),
                None => Type::Any,
            };

            ctx.variables.insert(
                name.to_string(),
                Variable {
                    value: eval_value,
                    typ: type_ref,
                    is_mutable,
                },
            );
        }
        Expr::BuiltInCall {
            function,
            args,
            return_type: _,
        } => match function {
            BuiltInFunction::Print => {
                let arg = eval(&args[0], ctx);
                print_interpreter(&arg, ctx);
            }
            BuiltInFunction::LastWord => {
                let arg = eval(&args[0], ctx);
                print_interpreter(&arg, ctx);
                std::process::exit(0);
            }
            _ => {}
        },
        Expr::Assignment {
            name,
            value,
            symbol,
        } => {
            ctx.variables.insert(
                name.to_string(),
                Variable {
                    value: eval(&*value, ctx),
                    typ: symbol
                        .map(|s| s.typ)
                        .unwrap_or_else(|| helpers::get_run_type(&value)),
                    is_mutable: true,
                },
            );
        }
        Expr::FunctionDef {
            name,
            params,
            body,
            return_type,
        } => {
            ctx.functions.insert(
                name.to_string(),
                FunctionDef {
                    params: params.into_iter().map(|p| (p.name, p.typ)).collect(),
                    body: body,
                    return_type: return_type.unwrap_or(Type::Any),
                },
            );
        }

        Expr::Call {
            target,
            name,
            args: _,
            returned_type: _,
        } => {
            if let Some(expr) = target {
                let target = eval(&*expr, ctx);
                match target {
                    /* FIXME: Burası tamamlanmayıb */
                    Expr::StructInit { name, args } => {
                        let structdef = ctx.structdefs.get(&name.to_string()).unwrap();
                        /* for (field, value) in args {

                        } */
                    }
                    _ => {}
                }

                /* runner_interpretator(ctx, *expr); */
            }
            let func = ctx.functions.get(&name.to_string()).unwrap();
            /* TODO:Burada body clone açığı var. */
            for expr in func.body.clone().into_iter() {
                runner_interpretator(ctx, expr);
            }
        }

        Expr::StructDef {
            name,
            fields,
            methods,
        } => {
            ctx.structdefs.insert(
                name.to_string(),
                StructDef {
                    name,
                    fields,
                    methods: methods
                        .into_iter()
                        .map(|method| Method {
                            name: method.name,
                            params: method
                                .params
                                .into_iter()
                                .map(|param| (param.name, param.typ))
                                .collect(),
                            body: method.body,
                            return_type: method.return_type,
                        })
                        .collect(),
                },
            );
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition = eval(&*condition, ctx);
            if let Expr::Bool(b) = condition {
                if b {
                    for expr in then_branch.into_iter() {
                        runner_interpretator(ctx, expr);
                    }
                } else {
                    for expr in else_branch.into_iter() {
                        match expr {
                            Expr::ElseIf {
                                condition,
                                then_branch,
                            } => {
                                let condition = eval(&*condition, ctx);
                                if let Expr::Bool(b) = condition {
                                    if b {
                                        for expr in then_branch.into_iter() {
                                            runner_interpretator(ctx, expr);
                                        }
                                        break;
                                    }
                                }
                            }
                            Expr::Else { then_branch } => {
                                for expr in then_branch.into_iter() {
                                    runner_interpretator(ctx, expr);
                                }
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        _ => {}
    }
}
pub fn eval<'a>(expr: &Expr<'a>, ctx: &Runner<'a>) -> Expr<'a> {
    match expr {
        Expr::Number(n) => Expr::Number(*n),
        Expr::Float(f) => Expr::Float(*f),
        Expr::Bool(b) => Expr::Bool(*b),
        Expr::Char(c) => Expr::Char(*c),
        Expr::String(s, t) => Expr::String(s, *t),
        Expr::DynamicString(s) => Expr::DynamicString(s.clone()),
        Expr::List(list) => {
            let elems: Vec<Expr> = list.iter().map(|e| eval(e, ctx)).collect();
            Expr::List(elems)
        }
        Expr::VariableRef { name, .. } => {
            if let Some(var) = ctx.variables.get(&name.to_string()) {
                eval(&var.value, ctx)
            } else {
                /* TODO: Burası Enum initilization olmalıdır amma başqa kod yazılmış Diqqet et. */
                Expr::DynamicString(Rc::new(name.to_string()))
            }
        }

        Expr::StructInit { name, args } => {
            /* TODO: Burası Best Practice Deyil. Random yazılıb */
            /*             let structdef = ctx.structdefs.get(&name.to_string()).unwrap();
             */
            Expr::StructInit {
                name: name.to_string().into(),
                args: args.to_vec(),
            }
        }

        Expr::BinaryOp { left, op, right } => {
            let left_val = eval(left, ctx);
            let right_val = eval(right, ctx);

            match (&left_val, &right_val) {
                (Expr::Number(l), Expr::Number(r)) => match *op {
                    "+" => Expr::Number(l + r),
                    "-" => Expr::Number(l - r),
                    "*" => Expr::Number(l * r),
                    "/" => Expr::Number(l / r),
                    "==" => Expr::Bool(l == r),
                    _ => Expr::Bool(false), // naməlum operator
                },

                (Expr::Bool(l), Expr::Bool(r)) => match *op {
                    "&&" => Expr::Bool(*l && *r),
                    "||" => Expr::Bool(*l || *r),
                    "==" => Expr::Bool(l == r),
                    _ => Expr::Bool(false),
                },

                _ => Expr::Bool(false),
            }
        }
        Expr::BuiltInCall {
            function,
            args,
            return_type,
        } => match function {
            BuiltInFunction::Ceil => {
                let arg = eval(&args[0], ctx);
                match arg {
                    Expr::Float(f) => Expr::Float(f.ceil()),
                    Expr::Number(n) => Expr::Float(n as f64),
                    Expr::UnaryOp { op, expr } => {
                        let expr = eval(&*expr, ctx);
                        match expr {
                            Expr::Float(f) => Expr::Float(f.ceil()),
                            Expr::Number(n) => Expr::Float(n as f64),
                            _ => Expr::Float(0.0),
                        }
                    }
                    _ => Expr::Float(0.0),
                }
            }
            _ => Expr::Number(0),
        },

        other => {
            println!(" Other {:?}", other);
            other.clone()
        }
    }
}
