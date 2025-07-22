use std::borrow::Cow;

use crate::{
    parser::ast::{BuiltInFunction, EnumDecl, Expr, Parameter, Type},
    transpiler::{
        TranspileContext,
        builtinfunctions::{
            min_max::{transpile_max, transpile_min},
            print::transpile_print,
            sum::transpile_sum,
        },
        decl::transpile_decl,
        helpers::{get_expr_type, is_semicolon_needed, map_type, transpile_function_def},
    },
};

pub fn transpile_expr<'a>(expr: &Expr<'a>, ctx: &mut TranspileContext<'a>) -> String {
    match expr {
        Expr::String(s) => format!("\"{}\"", s.escape_default()),
        Expr::Number(n) => n.to_string(),
        Expr::Float(n) => n.to_string(),
        Expr::Bool(b) => b.to_string(),
        Expr::Break => "break".to_string(),
        Expr::Continue => "continue".to_string(),
        Expr::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => transpile_decl(name, typ.as_ref(), *is_mutable, value, ctx),
        Expr::Return(expr) => {
            let arg_code = transpile_expr(expr, ctx);
            format!("return {arg_code}")
        }
        Expr::VariableRef { name, symbol } => {
            if ctx
                .enum_defs
                .values()
                .any(|variants| variants.contains(&name))
            {
                format!(".{name}")
            } else if let Some(sym) = symbol {
                if sym.is_pointer {
                    format!("{name}.*")
                } else {
                    format!("{name}")
                }
            } else {
                format!("{name}")
            }
        }
        Expr::List(items) => {
            let items_code: Vec<String> =
                items.iter().map(|item| transpile_expr(item, ctx)).collect();
            let items_str = items_code.join(", ");
            format!("[{items_str}]")
        }

        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition_code = transpile_expr(condition, ctx);

            let then_code: Vec<String> = then_branch
                .iter()
                .map(|e| {
                    let code = transpile_expr(e, ctx);
                    if is_semicolon_needed(e) {
                        format!("{};", code)
                    } else {
                        code
                    }
                })
                .collect();
            let then_code = then_code.join("\n    ");

            let mut else_code = String::new();
            for expr in else_branch {
                let code = transpile_expr(expr, ctx);
                else_code.push_str(&format!("\n{}", code));
            }

            format!(
                "if ({}) {{\n    {}\n}}{}",
                condition_code, then_code, else_code
            )
        }
        Expr::ElseIf {
            condition,
            then_branch,
        } => {
            let condition_code = transpile_expr(condition, ctx);

            let then_code: Vec<String> = then_branch
                .iter()
                .map(|e| {
                    let code = transpile_expr(e, ctx);
                    if !code.ends_with(';') {
                        format!("{};", code)
                    } else {
                        code
                    }
                })
                .collect();
            let then_code = then_code.join("\n    ");

            format!("else if ({}) {{\n    {}\n}}", condition_code, then_code)
        }

        Expr::Else { then_branch } => {
            let else_code: Vec<String> = then_branch
                .iter()
                .map(|e| {
                    let code = transpile_expr(e, ctx);

                    if is_semicolon_needed(e) && !code.trim_start().starts_with("//") {
                        format!("{};", code)
                    } else {
                        code
                    }
                }) //Bu iterator neden Result tipinde birşey döndürüyor
                .collect();
            let else_code = else_code.join("\n    ");

            format!("else {{\n    {}\n}}", else_code)
        }

        Expr::EnumDecl(EnumDecl { name, variants }) => {
            ctx.enum_defs.insert(name.clone(), variants.clone());
            let variants_code = variants
                .iter()
                .map(|v| format!("    {},", v))
                .collect::<Vec<_>>()
                .join("\n");

            format!("const {} = enum {{\n{}\n}};", name, variants_code)
        }
        Expr::BinaryOp { left, op, right } => {
            // Sol tərəfi transpile et (pointer yoxla)
            let left_code = match &**left {
                Expr::VariableRef { name, symbol } => {
                    if let Some(symbol) = symbol {
                        if symbol.is_pointer {
                            format!("{}.*", name)
                        } else {
                            name.to_string()
                        }
                    } else {
                        transpile_expr(left, ctx)
                    }
                }
                _ => transpile_expr(left, ctx),
            };

            // Sağ tərəfi transpile et (pointer yoxla)
            let right_code = match &**right {
                Expr::VariableRef { name, symbol } => {
                    if let Some(symbol) = symbol {
                        if symbol.is_pointer {
                            format!("{}.*", name)
                        } else {
                            name.to_string()
                        }
                    } else {
                        transpile_expr(right, ctx)
                    }
                }
                _ => transpile_expr(right, ctx),
            };

            let zig_op = match *op {
                "&&" => "and",
                "||" => "or",
                "==" => "==",
                "!=" => "!=",
                "+" => "+",
                "-" => "-",
                "*" => "*",
                "/" => "/",
                "<" => "<",
                ">" => ">",
                "<=" => "<=",
                ">=" => ">=",
                other => other,
            };

            format!("({} {} {})", left_code, zig_op, right_code)
        }

        Expr::StructDef {
            name,
            fields,
            methods,
        } => {
            let old_struct = ctx.current_struct.clone();

            ctx.struct_defs
                .insert(Cow::Borrowed(name), Cow::Owned(fields.clone()));

            ctx.current_struct = Some(name);
            let field_lines: Vec<String> = fields
                .iter()
                .map(|(fname, ftype)| {
                    let zig_type = map_type(ftype, true);
                    format!("    {}: {},", fname, zig_type)
                })
                .collect();

            let method_lines: Vec<String> = methods
                .iter()
                .map(|(method_name, params, body, return_type)| {
                    let uses_self = true;
                    let param_list: Vec<String> = params
                        .iter()
                        .filter(|p| p.name != "self") // self ayrıca işlənəcək
                        .map(|p| format!("{}: {}", p.name, map_type(&p.typ, true)))
                        .collect();

                    let self_prefix = if uses_self { "self: @This()" } else { "" };

                    let params_zig = if !param_list.is_empty() {
                        if uses_self {
                            format!(", {}", param_list.join(", "))
                        } else {
                            param_list.join(", ")
                        }
                    } else {
                        "".to_string()
                    };

                    let all_params = if self_prefix.is_empty() {
                        params_zig
                    } else if params_zig.is_empty() {
                        self_prefix.to_string()
                    } else {
                        format!("{}{}", self_prefix, params_zig)
                    };

                    let ret_type = return_type
                        .as_ref()
                        .map(|t| map_type(t, true))
                        .unwrap_or(Cow::Borrowed("void"));

                    let header =
                        format!("    pub fn {}({}) {} {{", method_name, all_params, ret_type);
                    let body_lines: Vec<String> = body
                        .iter()
                        .filter_map(|expr| {
                            let line = transpile_expr(expr, ctx);
                            if is_semicolon_needed(expr) && !line.trim_start().starts_with("//") {
                                Some(format!("{};", line))
                            } else {
                                Some(line)
                            }
                        })
                        .map(|line| format!("        {}", line))
                        .collect();
                    format!("{}\n{}\n    }}", header, body_lines.join("\n"))
                })
                .collect::<Vec<_>>();

            let mut all_lines = field_lines;
            all_lines.push("".to_string()); // boş sətr
            all_lines.extend(method_lines);
            let full_body = all_lines.join("\n");
            ctx.current_struct = old_struct;

            format!("const {} = struct {{\n{}\n}};", name, full_body)
        }
        Expr::FunctionDef {
            name,
            params,
            body,
            return_type,
        } => transpile_function_def(name, params, body, return_type, None, ctx),
        Expr::BuiltInCall {
            function,
            args,
            return_type,
        } => match function {
            BuiltInFunction::Print => transpile_print(&args[0], ctx),
            BuiltInFunction::Max => transpile_max(&args, ctx),
            BuiltInFunction::Sum => transpile_sum(&args, ctx),
            BuiltInFunction::Min => transpile_min(&args, ctx),
            BuiltInFunction::Range => {
                let start_code = transpile_expr(&args[0], ctx);
                let end_code = transpile_expr(&args[1], ctx);
                format!("{}..{}", start_code, end_code)
            }
            BuiltInFunction::Timer => {
                format!("@intCast(std.time.milliTimestamp())")
            }
            BuiltInFunction::Sqrt => {
                format!("@sqrt({}.0)", transpile_expr(&args[0], ctx))
            }
            BuiltInFunction::Round => {
                format!("@round({})", transpile_expr(&args[0], ctx))
            }
            BuiltInFunction::Floor => {
                format!("@floor({})", transpile_expr(&args[0], ctx))
            }
            BuiltInFunction::Ceil => {
                format!("@ceil({})", transpile_expr(&args[0], ctx))
            }
            BuiltInFunction::Mod => {
                format!("@abs({})", transpile_expr(&args[0], ctx))
            }
            _ => todo!(),
        },
        Expr::Call {
            target,
            name,
            args,
            returned_type,
        } => {
            let mut args_code = vec![];

            for arg in args {
                match arg {
                    Expr::VariableRef {
                        name,
                        symbol: Some(sym),
                    } => {
                        if sym.is_pointer {
                            args_code.push(format!("&{}", name));
                        } else {
                            args_code.push(name.to_string());
                        }
                    }
                    _ => {
                        // Digər hallarda transpile_expr çağır
                        let code = transpile_expr(arg, ctx);
                        args_code.push(code);
                    }
                }
            }

            format!("{}({})", name, args_code.join(", "))
        }

        Expr::StructInit { name, args } => {
            let mut field_lines: Vec<String> = Vec::new();
            let struct_fields = ctx.struct_defs.get(*name).unwrap().clone();

            for (i, arg_expr) in args.iter().enumerate() {
                let value_code = transpile_expr(arg_expr, ctx);
                let field_name = struct_fields.get(i).map(|s| s).unwrap();
                field_lines.push(format!(".{} = {}", field_name.0, value_code));
            }
            let body = field_lines.join(", ");
            format!("{}{{ {} }};", name, body)
        }

        Expr::Loop {
            var_name,
            iterable,
            body,
        } => {
            let iterable_code = transpile_expr(iterable, ctx);

            let mut body_lines = Vec::new();
            for expr in body {
                let mut line = transpile_expr(expr, ctx);
                if is_semicolon_needed(expr) && !line.trim_start().starts_with("//") {
                    line.push(';');
                }
                body_lines.push(format!("    {}", line));
            }
            let body_code = body_lines.join("\n");

            let loop_expr = match &**iterable {
                Expr::VariableRef {
                    symbol: Some(sym), ..
                } => {
                    if sym.is_mutable {
                        format!("{}.items", iterable_code)
                    } else {
                        iterable_code.clone()
                    }
                }
                _ => iterable_code.clone(),
            };

            format!("for ({}) |{}| {{\n{}\n}}", loop_expr, var_name, body_code)
        }
        Expr::UnaryOp { op, expr } => {
            let expr_code = transpile_expr(expr, ctx);
            format!("{}{}", op, expr_code)
        }
        Expr::Index {
            target,
            index,
            target_type,
        } => {
            let target_code = transpile_expr(target, ctx);
            let index_code = transpile_expr(index, ctx);

            let index_type_expr = get_expr_type(index);
            match index_type_expr {
                Type::Metn => {
                    format!("{}.{}", target_code, index_code.trim_matches('"'))
                }
                _ => {
                    format!("{}[{}]", target_code, index_code)
                }
            }
        }
        _ => {
            println!("not yet implemented");
            println!("{:?}", expr);
            todo!()
        }
    }
}
