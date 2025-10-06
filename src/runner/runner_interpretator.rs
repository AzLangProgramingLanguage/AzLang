use std::rc::Rc;

use crate::{
    dd,
    parser::ast::{BuiltInFunction, Expr, Symbol, Type},
    runner::{
        FunctionDef, Method, StructDef, UnionType, Variable, builtin::print::print_interpreter,
        eval::eval, helpers,
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
            match &*value {
                Expr::StructInit { name, args } => {
                    println!("StructInit: {}", name);
                    println!("Args: {:?}", args);
                }
                _ => {}
            }
            let eval_value = eval(&*value, ctx);

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
        Expr::UnionType {
            name,
            fields,
            methods,
        } => {
            ctx.uniontypes.insert(
                name.to_string(),
                UnionType {
                    name,
                    fields: fields,
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
        Expr::BuiltInCall {
            function,
            args,
            return_type: _,
        } => match function {
            BuiltInFunction::Print => {
                let arg = eval(&args[0], ctx);
                let output = print_interpreter(&arg, ctx);
                println!("{}", output);
            }
            BuiltInFunction::LastWord => {
                let arg = eval(&args[0], ctx);
                let output = print_interpreter(&arg, ctx);
                println!("{}", output);
                std::process::exit(0);
            }

            _ => {}
        },
        Expr::Loop {
            var_name,
            iterable,
            body,
        } => {
            /* FIXME   Burası buglu variable any tipinde olmamalı işleyir. Onu düzelt.*/
            let iterable = eval(&*iterable, ctx);
            match iterable {
                Expr::List(list) => {
                    let mut if_break = false;
                    let mut if_continue = false;
                    for item in list {
                        let item = eval(&item, ctx);
                        ctx.variables.insert(
                            var_name.to_string(),
                            Variable {
                                value: item,
                                typ: Type::Any,
                                is_mutable: false,
                            },
                        );
                        if if_break {
                            break;
                        }
                        if if_continue {
                            if_continue = false;
                            continue;
                        }
                        for expr in body.clone().into_iter() {
                            match expr {
                                Expr::Break => if_break = true,
                                Expr::Continue => if_continue = true,
                                _ => runner_interpretator(ctx, expr),
                            }
                            if if_break {
                                break;
                            }
                            if if_continue {
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
            ctx.variables.remove(var_name);
        }
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
