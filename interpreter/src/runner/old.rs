pub fn runner_interpretator<'a>(ctx: &mut Runner<'a>, expr: Expr<'a>) -> Option<Expr<'a>> {
    match expr {
        Expr::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            match &*value {
                Expr::StructInit { name, args } => {}
                _ => {}
            }
            let eval_value = {
                match *value {
                    Expr::Call {
                        target,
                        name,
                        args,
                        returned_type,
                    } => runner_interpretator(
                        ctx,
                        Expr::Call {
                            target,
                            name,
                            args,
                            returned_type,
                        },
                    )
                    .expect("Runner interpretator failed"),
                    _ => eval(&value, ctx),
                }
            };

            ctx.variables.insert(
                name.to_string(),
                Variable {
                    value: eval_value,
                    typ: (*typ).clone(), /* TODO: Yersiz Clone */
                    is_mutable,
                },
            );
            None
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
            None
        }
        Expr::BuiltInCall {
            function,
            args,
            return_type: _,
        } => match function {
            BuiltInFunction::Print => {
                let arg = {
                    match &args[0] {
                        Expr::Call {
                            target,
                            name,
                            args,
                            returned_type,
                        } => runner_interpretator(
                            ctx,
                            Expr::Call {
                                target: target.clone(),
                                name: name,
                                args: args.to_vec(),
                                returned_type: returned_type.clone(),
                            },
                        )
                        .unwrap(),
                        /* BUG: Burası runner_interpretator olmalıdır   */
                        _ => runner_interpretator(ctx, args.get(0).unwrap().clone()).unwrap(), // TODO:
                                                                                               // Buraya baxarsan
                    }
                };
                let output = print_interpreter(&arg, ctx);
                println!("{}", output);
                None
            }
            BuiltInFunction::LastWord => {
                let arg = eval(&args[0], ctx);
                let output = print_interpreter(&arg, ctx);
                println!("{}", output);
                std::process::exit(0);
            }

            _ => None,
        },

        Expr::VariableRef { name, .. } => {
            if let Some(var) = ctx.variables.get(&name.to_string()) {
                runner_interpretator(ctx, var.value.clone()) //TODO: Burada yersiz clone var
            } else {
                /* TODO: Burası Enum initilization olmalıdır amma başqa kod yazılmış Diqqet et. */
                Some(Expr::DynamicString(Rc::new(name.to_string())))
            }
        }

        Expr::Loop {
            var_name,
            iterable,
            body,
        } => {
            /* FIXME:   Burası buglu variable any tipinde olmamalı işleyir. Onu düzelt.*/
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
                                _ => {
                                    runner_interpretator(ctx, expr);
                                }
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
            Some(Expr::Void)
        }
        Expr::Assignment {
            name,
            value,
            symbol,
        } => {
            let new_value = runner_interpretator(ctx, *value).unwrap_or(Expr::Void);
            ctx.variables.insert(
                name.to_string(),
                Variable {
                    value: new_value,
                    typ: symbol.map(|s| s.typ).unwrap_or(Type::Any),
                    is_mutable: true,
                },
            );
            None
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
                    body: Rc::new(body),
                    return_type: return_type.unwrap_or(Type::Any),
                },
            );
            Some(Expr::Void)
        }

        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition = runner_interpretator(ctx, *condition).unwrap_or(Expr::Void);
            if let Expr::Bool(b) = condition {
                if b {
                    return exec_block(ctx, then_branch);
                } else {
                    for expr in else_branch.into_iter() {
                        match expr {
                            Expr::ElseIf {
                                condition,
                                then_branch,
                            } => {
                                let condition = runner_interpretator(ctx, *condition)
                                    .unwrap_or(Expr::Bool(false));
                                if let Expr::Bool(b) = condition {
                                    if b {
                                        return exec_block(ctx, then_branch);
                                    }
                                }
                            }
                            Expr::Else { then_branch } => {
                                return exec_block(ctx, then_branch);
                            }
                            _ => {}
                        }
                    }
                }
            }
            None
        }

        Expr::BinaryOp { left, right, op } => {
            let left_val = match *left {
                Expr::Call { .. } => runner_interpretator(ctx, *left).unwrap_or(Expr::Void),
                _ => eval(&left, ctx),
            };
            let right_val = match *right {
                Expr::Call { .. } => runner_interpretator(ctx, *right).unwrap_or(Expr::Void),
                _ => eval(&right, ctx),
            };
            match (&left_val, &right_val) {
                (Expr::Number(l), Expr::Number(r)) => match op {
                    "+" => Some(Expr::Number(l + r)),
                    "-" => Some(Expr::Number(l - r)),
                    "*" => Some(Expr::Number(l * r)),
                    "/" => Some(Expr::Number(l / r)),
                    "==" => Some(Expr::Bool(l == r)),
                    _ => Some(Expr::Bool(false)),
                },
                (s, r) => {
                    let left = runner_interpretator(ctx, s.clone()).expect("msg");

                    runner_interpretator(
                        ctx,
                        Expr::BinaryOp {
                            left: Box::new(left),
                            right: Box::new(r.clone()),
                            op,
                        },
                    )
                }
                (Expr::Float(l), Expr::Number(r)) => match op {
                    "+" => Some(Expr::Float(l + *r as f64)),
                    "-" => Some(Expr::Float(l - *r as f64)),
                    "/" => Some(Expr::Float(l / *r as f64)),
                    "*" => Some(Expr::Float(l * *r as f64)),
                    "==" => Some(Expr::Bool(*l == *r as f64)),
                    _ => Some(Expr::Bool(false)),
                },
                (Expr::Float(l), Expr::Float(r)) => match op {
                    "+" => Some(Expr::Float(l + r)),
                    "-" => Some(Expr::Float(l - r)),
                    "/" => Some(Expr::Float(l / r)),
                    "*" => Some(Expr::Float(l * r)),
                    "==" => Some(Expr::Bool(l == r)),
                    _ => Some(Expr::Bool(false)),
                },
                (Expr::Number(l), Expr::Float(r)) => match op {
                    "+" => Some(Expr::Float(*l as f64 + r)),
                    "-" => Some(Expr::Float(*l as f64 - r)),
                    "/" => Some(Expr::Float(*l as f64 / r)),
                    "*" => Some(Expr::Float(*l as f64 * r)),
                    "==" => Some(Expr::Bool(*l as f64 == *r)),
                    _ => Some(Expr::Bool(false)),
                },

                (Expr::Time(l), Expr::Time(r)) => match op {
                    "-" => Some(Expr::Number(l.duration_since(*r).as_millis() as i64)),
                    _ => Some(Expr::Bool(false)),
                },

                (Expr::Bool(l), Expr::Bool(r)) => match op {
                    "&&" => Some(Expr::Bool(*l && *r)),
                    "||" => Some(Expr::Bool(*l || *r)),
                    "==" => Some(Expr::Bool(l == r)),
                    _ => Some(Expr::Bool(false)),
                },
            }
        }
        Expr::Call {
            target, name, args, ..
        } => {
            let mut variable_name = None;
            if let Some(expr) = target {
                let target_val = match &*expr {
                    Expr::Call { .. } => runner_interpretator(ctx, *expr).unwrap_or(Expr::Void),
                    Expr::VariableRef { name, symbol } => {
                        variable_name = Some(name.to_string());
                        eval(&expr, ctx)
                    }
                    _ => eval(&expr, ctx),
                };

                match target_val {
                    Expr::String(s) => handle_string_call(&name, &s, args, ctx),
                    Expr::Number(n) => handle_number_call(&name, n, args, ctx),
                    Expr::List(list) => handle_list_call(&name, list, variable_name, args, ctx),
                    Expr::StructInit {
                        name: struct_name,
                        args,
                    } => {
                        let structdef = ctx.structdefs.get(&struct_name.to_string()).unwrap();
                        let method = structdef.methods.iter().find(|m| m.name == name)?;

                        // "self" əlavə et və metod bədənini icra et
                        ctx.variables.insert(
                            "self".into(),
                            Variable {
                                value: Expr::StructInit {
                                    name: struct_name.clone(),
                                    args,
                                },
                                typ: Type::User(struct_name.clone()),
                                is_mutable: false,
                            },
                        );

                        let result = exec_block(ctx, method.body.clone());
                        ctx.variables.remove("self");
                        result
                    }
                    _ => None,
                }
            } else {
                let func = ctx.functions.get(name)?;
                for (i, (param, typ)) in func.params.iter().enumerate() {
                    ctx.variables.insert(
                        param.clone(),
                        Variable {
                            value: eval(&args[i], ctx),
                            typ: typ.clone(),
                            is_mutable: false,
                        },
                    );
                }
                exec_block(ctx, func.body.to_vec()).or(Some(Expr::Void))
            }
            /* TODO:  Burada deyerler silinmelidir */
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
            Some(Expr::Void)
        }

        _ => None,
    }
}
