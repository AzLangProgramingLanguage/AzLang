use std::rc::Rc;

use crate::{
    dd,
    interpretator::{Method, StructDef, Variable, builtin::print::print_interpreter},
    parser::ast::{BuiltInFunction, Expr},
};

use super::InterPretator;

pub fn runner_interpretator<'a>(ctx: &mut InterPretator<'a>, expr: Expr<'a>) {
    match expr {
        Expr::Decl {
            name,
            transpiled_name: _,
            typ,
            is_mutable,
            value,
        } => {
            let eval_value = eval(&*value, ctx);
            ctx.variables.insert(
                name.to_string(),
                Variable {
                    value: Rc::new(eval_value),
                    typ: typ.unwrap(),
                    is_mutable,
                },
            );
        }
        Expr::StructDef {
            name,
            transpiled_name: _,
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
        Expr::Assignment {
            name,
            value,
            symbol,
        } => {
            /* TODO: ASSINGMENT */
            let eval_value = eval(&*value, ctx);
            dd!(symbol);
            ctx.variables.insert(
                name.to_string(),
                Variable {
                    value: Rc::new(eval_value),
                    typ: Rc::new(symbol.expect("Symbol not found").typ),
                    is_mutable: true,
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
        _ => {}
    }
}

// Çözüm 1: Reference döndürme yerine owned value döndürme
pub fn eval<'a>(expr: &Expr<'a>, ctx: &InterPretator<'a>) -> Expr<'a> {
    match expr {
        Expr::String(s, t) => Expr::String(s, *t),
        Expr::Number(n) => Expr::Number(*n),
        Expr::DynamicString(s) => Expr::DynamicString(s.clone()),
        Expr::Float(f) => Expr::Float(*f),
        Expr::Bool(b) => Expr::Bool(*b),
        Expr::Char(c) => Expr::Char(*c),
        Expr::List(list) => Expr::List(list.iter().map(|expr| eval(expr, ctx)).collect()),
        Expr::VariableRef { name, .. } => {
            if let Some(var) = ctx.variables.get(&name.to_string()) {
                match &*var.value {
                    Expr::String(s, t) => Expr::String(s, *t),
                    Expr::Number(n) => Expr::Number(*n),
                    Expr::Float(f) => Expr::Float(*f),
                    Expr::Bool(b) => Expr::Bool(*b),
                    Expr::Char(c) => Expr::Char(*c),
                    other => eval(other, ctx),
                }
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
                    _ => left_val,
                },
                _ => left_val,
            }
        }

        other => match other {
            Expr::String(s, t) => Expr::String(s, *t),
            _ => Expr::String("Bilinməyən", false),
        },
    }
}
