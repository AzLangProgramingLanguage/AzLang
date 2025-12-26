use std::rc::Rc;

use crate::runner::{FunctionDef, Variable, builtin::print::print_interpreter};

use super::Runner;

use parser::{
    ast::Expr,
    shared_ast::{BuiltInFunction, Type},
};

pub fn runner_interpretator<'a>(ctx: &mut Runner<'a>, expr: &Expr<'a>) -> Expr<'a> {
    match expr {
        Expr::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            let new_value: Expr<'a> = runner_interpretator(ctx, value);
            ctx.variables.insert(
                name.to_string(),
                Variable {
                    value: Rc::new(new_value),
                    typ: (**typ).clone(),
                    is_mutable: *is_mutable,
                },
            );
            Expr::Void
        }

        Expr::FunctionDef {
            name,
            params,
            body,
            return_type,
        } => {
            let body_rc = Rc::new(*body);
            ctx.functions.insert(
                name.to_string(),
                FunctionDef {
                    params: params.into_iter().map(|e| e.name).collect(),
                    body: body_rc,
                    return_type: return_type.unwrap_or(Type::Any),
                },
            );
            Expr::Void
        }

        Expr::Assignment {
            name,
            value,
            symbol,
        } => {
            let new_value: Expr<'a> = runner_interpretator(ctx, value);
            if let Some(var) = ctx.variables.get_mut(&name.to_string()) {
                var.value = Rc::new(new_value);
            }

            Expr::Void
        }

        Expr::BinaryOp {
            left,
            right,
            op,
            return_type,
        } => {
            let left = runner_interpretator(ctx, left);
            let right = runner_interpretator(ctx, right);
            let result = match *op {
                "+" => {
                    if let Type::Natural = *return_type {
                        let left = left.as_number().unwrap();
                        let right = right.as_number().unwrap();
                        Expr::Number(left + right)
                    } else {
                        println!("other type: {:?}", *return_type);
                        let left = left.as_float().unwrap();
                        let right = right.as_float().unwrap();
                        Expr::Float(left + right)
                    }
                }
                "-" => {
                    if let Type::Natural = *return_type {
                        let left = left.as_number().unwrap();
                        let right = right.as_number().unwrap();
                        Expr::Number(left - right)
                    } else {
                        println!("other type: {:?}", *return_type);

                        let left = left.as_float().unwrap();
                        let right = right.as_float().unwrap();
                        Expr::Float(left - right)
                    }
                }
                "*" => {
                    if let Type::Natural = *return_type {
                        let left = left.as_number().unwrap();
                        let right = right.as_number().unwrap();
                        Expr::Number(left * right)
                    } else {
                        let left = left.as_float().unwrap();
                        let right = right.as_float().unwrap();
                        Expr::Float(left * right)
                    }
                }
                "/" => {
                    if let Type::Natural = *return_type {
                        let left = left.as_number().unwrap();
                        let right = right.as_number().unwrap();
                        Expr::Number(left / right)
                    } else {
                        let left = left.as_float().unwrap();
                        let right = right.as_float().unwrap();
                        Expr::Float(left / right)
                    }
                }
                "%" => {
                    if let Type::Natural = *return_type {
                        let left = left.as_number().unwrap();
                        let right = right.as_number().unwrap();
                        Expr::Number(left % right)
                    } else {
                        let left = left.as_float().unwrap();
                        let right = right.as_float().unwrap();
                        Expr::Float(left % right)
                    }
                }
                /*  "==" => {
                    if left == right {
                        Expr::Bool(true)
                    } else {
                        Expr::Bool(false)
                    }
                }
                "!=" => {
                    if left != right {
                        Expr::Bool(true)
                    } else {
                        Expr::Bool(false)
                    }
                } */
                _ => Expr::Bool(false),
            };

            result
        }
        Expr::BuiltInCall {
            function,
            args,
            return_type,
        } => match function {
            BuiltInFunction::Print => {
                let arg = runner_interpretator(ctx, &args[0]);
                let output = print_interpreter(&arg, ctx);
                println!("{}", output);
                Expr::Void
            }
            _ => Expr::Void,
        },
        Expr::VariableRef { name, symbol } => {
            if let Some(var) = ctx.variables.get(&name.to_string()) {
                return var.value.as_ref().clone();
            }
            Expr::Void
        }
        Expr::String(s) => Expr::String(s),
        Expr::Number(n) => Expr::Number(*n),
        Expr::List(l) => Expr::List(l.clone()),
        Expr::Bool(b) => Expr::Bool(*b),
        Expr::DynamicString(s) => Expr::DynamicString(s.clone()),
        Expr::Void => Expr::Void,
        _ => Expr::Void,
    }
}
