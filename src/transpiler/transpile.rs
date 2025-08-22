use std::borrow::Cow;

use crate::{
    parser::ast::{BuiltInFunction, EnumDecl, Expr, Symbol, TemplateChunk, Type},
    transpiler::{
        TranspileContext,
        builtinfunctions::{
            min_max::{transpile_max, transpile_min},
            print::transpile_print,
            sum::transpile_sum,
        },
        decl::transpile_decl,
        helpers::{get_expr_type, is_semicolon_needed, map_type, transpile_function_def},
        struct_def::transpile_struct_def,
    },
};

use super::union_def::transpile_union_def;

pub fn transpile_expr<'a>(expr: &'a Expr<'a>, ctx: &mut TranspileContext<'a>) -> String {
    match expr {
        Expr::String(s, b) => {
            if *b {
                format!("try allocator.dupe(u8, \"{}\")", s.escape_default())
            } else {
                format!("\"{}\"", s.escape_default())
            }
        }

        Expr::Number(n) => n.to_string(),
        Expr::Float(n) => n.to_string(),
        Expr::Bool(b) => b.to_string(),
        Expr::Break => "break".to_string(),
        Expr::Continue => "continue".to_string(),
        Expr::Decl {
            name: _,
            transpiled_name,
            typ,
            is_mutable,
            value,
        } => transpile_decl(
            transpiled_name.as_ref().unwrap(),
            typ.as_deref(),
            *is_mutable,
            value,
            ctx,
        ),
        Expr::Return(expr) => {
            let arg_code = transpile_expr(expr, ctx);
            format!("return {arg_code}")
        }
        Expr::VariableRef {
            name,
            transpiled_name,
            symbol,
        } => {
            if name == "self" {
                ctx.is_used_self = true;
            }

            let transpiled_name: Cow<str> = match transpiled_name {
                Some(transled_name) => Cow::Borrowed(transled_name),
                None => Cow::Owned(name.to_string()),
            };

            if ctx
                .enum_defs
                .values()
                .any(|variants| variants.contains(&name))
            {
                format!(".{transpiled_name}")
            } else if let Some(sym) = symbol {
                if sym.is_pointer {
                    format!("{transpiled_name}.*")
                } else {
                    format!("{transpiled_name}")
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
                else_code.push_str(&format!("\n{code}"));
            }

            format!("if ({condition_code}) {{\n    {then_code}\n}}{else_code}",)
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
                        format!("{code};")
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
                Expr::VariableRef {
                    name,
                    transpiled_name,
                    symbol,
                } => {
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

            let right_code = match &**right {
                Expr::VariableRef {
                    name,
                    transpiled_name,
                    symbol,
                } => {
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
            match zig_op {
                "/" => format!("(@divTrunc({left_code}.deyer,{right_code}.deyer))"),
                "%" => format!("(@mod({left_code}.deyer,{right_code}.deyer))"),
                _ => format!("({}.deyer {} {}.deyer)", left_code, zig_op, right_code),
            }
        }

        Expr::StructDef {
            name,
            transpiled_name,
            fields,
            methods,
        } => transpile_struct_def(
            name,
            transpiled_name.as_deref().unwrap(),
            fields,
            methods,
            ctx,
        ),
        Expr::UnionType {
            name,
            transpiled_name,
            fields,
            methods,
        } => {
            let new_name = transpiled_name.as_deref().unwrap();
            transpile_union_def(name, new_name, fields, methods, ctx)
        }
        Expr::TemplateString(template) => {
            let mut lines = Vec::new();
            for part in template {
                match part {
                    TemplateChunk::Literal(lit) => lines.push(format!("{}", lit)),
                    TemplateChunk::Expr(expr) => {
                        lines.push(format!("{}", transpile_expr(expr, ctx)))
                    }
                }
            }
            lines.join("\n")
        }

        Expr::FunctionDef {
            name,
            transpiled_name: _,
            params,
            body,
            return_type,
            is_allocator,
        } => transpile_function_def(name, params, body, return_type, None, ctx, is_allocator),
        Expr::BuiltInCall {
            function,
            args,
            return_type: _,
        } => match function {
            BuiltInFunction::Print => transpile_print(&args[0], ctx),
            BuiltInFunction::Max => transpile_max(&args, ctx),
            BuiltInFunction::Sum => transpile_sum(&args, ctx),
            BuiltInFunction::ConvertString => {
                let arg_code = transpile_expr(&args[0], ctx);
                format!("try convert_string(allocator, {},false)", arg_code)
            }
            BuiltInFunction::Min => transpile_min(&args, ctx),
            BuiltInFunction::Number => {
                let arg_code = transpile_expr(&args[0], ctx);
                format!("try std.fmt.parseInt(isize, {}, 10)", arg_code)
            }
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
            BuiltInFunction::Zig => {
                let code = transpile_expr(&args[0], ctx);
                format!("{}", code)
            }
            BuiltInFunction::StrLower => {
                ctx.is_used_allocator = true;
                let code = transpile_expr(&args[0], ctx);
                format!("try str_lowercase(allocator, {},false)", code)
            }
            BuiltInFunction::StrUpper => {
                ctx.is_used_allocator = true;
                let code = transpile_expr(&args[0], ctx);
                format!("try str_uppercase(allocator, {},false)", code)
            }
            BuiltInFunction::Input => {
                ctx.used_input_fn = true;
                let prompt = transpile_expr(&args[0], ctx);
                let buf_name = "buf_temp";

                format!(
                    r#"(blk: {{
                    var {buf}: [100]u8 = undefined;
                    break :blk try input({prompt}, &{buf});
                }})"#,
                    buf = buf_name,
                    prompt = prompt
                )
            }
            BuiltInFunction::LastWord => {
                let print_code = transpile_print(&args[0], ctx);
                format!("{};\n    std.process.exit(0)", print_code)
            }
            BuiltInFunction::StrReverse => {
                ctx.is_used_allocator = true;
                let code = transpile_expr(&args[0], ctx);
                format!("try str_reverse(allocator, {},false)", code)
            }
            _ => todo!(),
        },
        Expr::Call {
            target,
            name: _,
            args,
            returned_type: _,
            is_allocator,
            transpiled_name,
        } => {
            // Argümanları dönüştür
            let mut args_code: Vec<String> = args
                .iter()
                .map(|arg| match arg {
                    Expr::VariableRef {
                        transpiled_name,
                        symbol: Some(sym),
                        ..
                    } => {
                        let new_name = transpiled_name.as_ref().unwrap();
                        if sym.is_pointer {
                            format!("&{}", new_name)
                        } else {
                            new_name.to_string()
                        }
                    }

                    _ => transpile_expr(arg, ctx),
                })
                .collect();

            ctx.is_used_allocator = true;

            // Eğer allocator gerekiyorsa ekle
            if *is_allocator {
                ctx.needs_allocator = true;
                args_code.push("allocator".to_string());
            }

            // Fonksiyon adı
            let func_name = transpiled_name.as_ref().unwrap();

            match target.as_deref() {
                Some(Expr::VariableRef {
                    name: target_name, ..
                }) => {
                    if *is_allocator {
                        format!(
                            "(try {}.{} ({}))",
                            target_name,
                            func_name,
                            args_code.join(", ")
                        )
                    } else {
                        format!("{}.{} ({})", target_name, func_name, args_code.join(", "))
                    }
                }
                Some(Expr::Number(n)) => {
                    format!(
                        "azlangEded.Yeni({}).{}({})",
                        n,
                        func_name,
                        args_code.join(", ")
                    )
                }
                Some(Expr::String(s, _)) => {
                    format!(
                        "azlangYazi.Yeni(azlangYazi{{.Const = \"{}\"}}).{}({})",
                        s,
                        func_name,
                        args_code.join(", ")
                    )
                }
                _ => format!("{}({})", func_name, args_code.join(", ")),
            }
        }

        Expr::StructInit {
            name,
            transpiled_name,
            args,
        } => {
            let mut field_lines: Vec<String> = Vec::new();
            let transpiled_name = transpiled_name.as_ref().unwrap();
            for (i, arg_expr) in args.iter().enumerate() {
                let value_code = transpile_expr(&arg_expr.1, ctx);
                let field_name = arg_expr.0;
                field_lines.push(format!(".{} = {}", field_name, value_code));
            }
            let body = field_lines.join(", ");
            format!("{}{{ {} }};", transpiled_name, body)
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
            target_type: _,
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
        Expr::Match { target, arms } => {
            let target_code = transpile_expr(target, ctx);
            let mut arms_code = Vec::new();
            for arm in arms {
                let pattern_code = transpile_expr(&arm.0, ctx);
                let mut expr_code = String::new();
                for expr in &arm.1 {
                    expr_code.push_str(&transpile_expr(&expr, ctx));
                }
                expr_code.push(',');

                arms_code.push(format!(".{} => {}", pattern_code, expr_code));
            }
            format!("switch ({}) {{\n{}\n}}", target_code, arms_code.join("\n"))
        }
        Expr::Assignment {
            name,
            value,
            symbol: _,
        } => {
            let mut value_code = String::new();
            match &**value {
                Expr::StructInit {
                    name: _,
                    transpiled_name,
                    args,
                } => {
                    if transpiled_name.as_ref().map(|t| t.as_ref()) == Some("azlangYazi") {
                        // args[1].1 -> ikinci argümanın değeri
                        value_code = format!(
                            "azlangYazi{{ .Mut = try allocator.dupe(u8,  {} )  }}",
                            transpile_expr(&args[0].1, ctx)
                        );
                    } else {
                        value_code = transpile_expr(value, ctx);
                    }
                }
                _ => {
                    value_code = transpile_expr(value, ctx);
                }
            }

            format!("{} = {}", name, value_code)
        }

        _ => {
            println!("not yet implemented");
            println!("{:?}", expr);
            todo!()
        }
    }
}
