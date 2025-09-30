use std::rc::Rc;

use crate::{
    dd,
    parser::ast::{BuiltInFunction, Expr, Symbol, Type},
    runner::{
        FunctionDef, Method, StructDef, Variable, builtin::print::print_interpreter, helpers,
    },
};

use super::Runner;
use bumpalo::Bump;

pub fn runner_interpretator<'a>(ctx: &mut Runner<'a>, expr: Expr<'a>) {
    match expr {
        Expr::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            let eval_value = {
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
                print_interpreter(&args[0], ctx);
            }
            BuiltInFunction::LastWord => {
                print_interpreter(&args[0], ctx);
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
        /*
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

        Expr::FunctionDef {
            name,
            params,
            body,
            return_type,
        } => {
            ctx.functions.insert(
                name.to_string(),
                FunctionDef {
                    params: params
                        .into_iter()
                        .map(|param| (param.name, param.typ))
                        .collect(),
                    body: &body, /* `body` does not live long enough
                                 borrowed value does not live long enough */
                    return_type,
                },
            );
        }
        Expr::Call {
            target,
            name,
            args,
            returned_type,
        } => {
            //TODO:  Burada expr üçün  clone traitini implement etmemizi isteyir
            if let Some(func_def) = ctx.functions.remove(&name.to_string()) {
                /*    for expr in func_def.body.into_iter() {
                    runner_interpretator(ctx, expr);
                } */
            }
        }


        }, */
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
                Expr::Number(0)
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

        other => other.clone(),
    }
}
