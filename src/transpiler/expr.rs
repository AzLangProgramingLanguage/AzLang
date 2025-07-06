use crate::array_methods::transpile_list_method_call;
use crate::context::TranspileContext;
use crate::declaration::transpile_constant_decl;
use crate::function::{is_semicolon_needed, transpile_function_call, transpile_function_def};
use crate::lexer::Token;
use crate::r#loop::transpile_loop;
use crate::parser::ast::{EnumDecl, Type};
use crate::parser::{Expr, ast::BuiltInFunction};
use crate::string_methods::transpile_string_method_call;
use crate::transpiler::declaration::transpile_mutable_decl;
use crate::transpiler::utils::{
    map_type, transpile_builtin_print, transpile_builtin_range, transpile_builtin_sum,
};
pub fn transpile_expr(expr: &Expr, ctx: &mut TranspileContext) -> Result<String, String> {
    match expr {
        Expr::Index { target, index } => {
            let target_code = match &**target {
                Expr::VariableRef {
                    name,
                    symbol: Some(sym),
                } => {
                    let base = if sym.is_pointer {
                        format!("{}.*", name)
                    } else {
                        name.clone()
                    };
                    if sym.is_mutable {
                        format!("{}.items", base)
                    } else {
                        base
                    }
                }
                _ => transpile_expr(target, ctx)?,
            };

            let index_code = transpile_expr(index, ctx)?;

            Ok(format!("{}[{}]", target_code, index_code))
        }

        Expr::EnumDecl(EnumDecl { name, variants }) => {
            ctx.enum_defs.insert(name.clone(), variants.clone());
            let variants_code = variants
                .iter()
                .map(|v| format!("    {},", v))
                .collect::<Vec<_>>()
                .join("\n");

            Ok(format!("const {} = enum {{\n{}\n}};", name, variants_code))
        }
        Expr::Match(match_expr) => {
            let target_code = transpile_expr(&match_expr.target, ctx)?;

            let is_enum = match &*match_expr.target {
                Expr::VariableRef {
                    name,
                    symbol: Some(sym),
                } => matches!(sym.typ, Type::Istifadeci(_)),
                _ => false,
            };

            let arms_code: Vec<String> = match_expr
                .arms
                .iter()
                .map(|(variant_token, exprs)| {
                    let variant_str = match variant_token {
                        Token::Identifier(s) => {
                            if is_enum {
                                format!(".{}", s)
                            } else if s == "_" {
                                "else".to_string()
                            } else {
                                s.clone()
                            }
                        }
                        Token::Underscore => {
                            if is_enum {
                                "_".to_string()
                            } else {
                                "else".to_string()
                            }
                        }
                        Token::Number(n) => n.to_string(),
                        Token::StringLiteral(s) => {
                            if s.len() == 1 {
                                format!("'{}'", s)
                            } else {
                                format!("\"{}\"", s)
                            }
                        }
                        _ => format!("{:?}", variant_token),
                    };

                    let block_code = exprs
                        .iter()
                        .filter_map(|e| transpile_expr(e, ctx).ok())
                        .map(|line| format!("    {};", line))
                        .collect::<Vec<_>>()
                        .join("\n");

                    format!(
                        "{variant} => {{\n{block}\n}}",
                        variant = variant_str,
                        block = block_code
                    )
                })
                .collect();

            let arms_joined = arms_code.join(",\n");

            Ok(format!("switch ({}) {{\n{}\n}}", target_code, arms_joined))
        }

        Expr::Break => Ok("break".to_string()),
        Expr::Continue => Ok("continue".to_string()),
        Expr::Assignment {
            name,
            value,
            symbol,
        } => {
            let value_code = transpile_expr(value, ctx)?;

            let left = match symbol {
                Some(sym) if sym.is_pointer => format!("{}.*", name),
                _ => name.clone(),
            };

            Ok(format!("{} = {}", left, value_code))
        }

        Expr::StructDef {
            name,
            fields,
            methods,
        } => {
            let old_struct = ctx.current_struct.clone();

            ctx.struct_defs.insert(
                name.clone(),
                fields.iter().map(|(fname, _)| fname.clone()).collect(),
            ); //  The application panicked (crashed).
            //  Message:  called `Option::unwrap()` on a `None` value

            ctx.current_struct = Some(name.clone());
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
                        .unwrap_or("void".to_string());

                    let header =
                        format!("    pub fn {}({}) {} {{", method_name, all_params, ret_type);
                    let body_lines: Vec<String> = body
                        .iter()
                        .filter_map(|expr| {
                            let line = transpile_expr(expr, ctx).ok()?;
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
            Ok(format!("const {} = struct {{\n{}\n}};", name, full_body))
        }

        Expr::StructInit { name, args } => {
            let mut field_lines = Vec::new();

            // 💡 Burada ctx-yə immutable giriş veririk və dərhal bağlayırıq
            let struct_fields = ctx
                .struct_defs
                .get(name)
                .ok_or_else(|| format!("Struct `{}` tapılmadı", name))?
                .clone();

            for (i, arg_expr) in args.iter().enumerate() {
                // 💡 İndi ctx-ni mutable kimi verə bilərik
                let value_code = transpile_expr(arg_expr, ctx)?;

                let field_name = struct_fields
                    .get(i)
                    .map(|s| s.as_str())
                    .unwrap_or("unknown");

                field_lines.push(format!(".{} = {}", field_name, value_code));
            }

            let body = field_lines.join(", ");
            Ok(format!("{}{{ {} }};", name, body))
        }

        Expr::FieldAccess { target, field, .. } => {
            let target_code = transpile_expr(target, ctx)?;
            Ok(format!("{}.{}", target_code, field))
        }

        Expr::BinaryOp { left, op, right } => {
            // Sol tərəfi transpile et (pointer yoxla)
            let left_code = match &**left {
                Expr::VariableRef { name, symbol } => {
                    if let Some(symbol) = symbol {
                        if symbol.is_pointer {
                            format!("{}.*", name)
                        } else {
                            name.clone()
                        }
                    } else {
                        transpile_expr(left, ctx)?
                    }
                }
                _ => transpile_expr(left, ctx)?,
            };

            // Sağ tərəfi transpile et (pointer yoxla)
            let right_code = match &**right {
                Expr::VariableRef { name, symbol } => {
                    if let Some(symbol) = symbol {
                        if symbol.is_pointer {
                            format!("{}.*", name)
                        } else {
                            name.clone()
                        }
                    } else {
                        transpile_expr(right, ctx)?
                    }
                }
                _ => transpile_expr(right, ctx)?,
            };

            // Operator transpilyasiyası (artıq sadədir)
            let zig_op = match op.as_str() {
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
                other => return Err(format!("Dəstəklənməyən operator: {}", other)),
            };

            Ok(format!("({} {} {})", left_code, zig_op, right_code))
        }

        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition_code = transpile_expr(condition, ctx)?;

            let then_code: Result<Vec<String>, String> = then_branch
                .iter()
                .map(|e| {
                    let code = transpile_expr(e, ctx)?;
                    if !code.ends_with(';') {
                        Ok(format!("{};", code))
                    } else {
                        Ok(code)
                    }
                })
                .collect();
            let then_code = then_code?.join("\n    ");

            let mut else_code = String::new();
            for expr in else_branch {
                let code = transpile_expr(expr, ctx)?;
                else_code.push_str(&format!("\n{}", code));
            }

            Ok(format!(
                "if ({}) {{\n    {}\n}}{}",
                condition_code, then_code, else_code
            ))
        }
        Expr::ElseIf {
            condition,
            then_branch,
        } => {
            let condition_code = transpile_expr(condition, ctx)?;

            let then_code: Result<Vec<String>, String> = then_branch
                .iter()
                .map(|e| {
                    let code = transpile_expr(e, ctx)?;
                    if !code.ends_with(';') {
                        Ok(format!("{};", code))
                    } else {
                        Ok(code)
                    }
                })
                .collect();
            let then_code = then_code?.join("\n    ");

            Ok(format!(
                "else if ({}) {{\n    {}\n}}",
                condition_code, then_code
            ))
        }

        Expr::Else { then_branch } => {
            let else_code: Result<Vec<String>, String> = then_branch
                .iter()
                .map(|e| {
                    let code = transpile_expr(e, ctx)?;
                    if !code.ends_with(';') {
                        Ok(format!("{};", code))
                    } else {
                        Ok(code)
                    }
                })
                .collect();
            let else_code = else_code?.join("\n    ");

            Ok(format!("else {{\n    {}\n}}", else_code))
        }
        Expr::TemplateString(_) => Ok("".to_string()),
        Expr::Loop {
            var_name,
            iterable,
            body,
        } => transpile_loop(var_name, iterable, body, ctx),
        Expr::BuiltInCall {
            func,
            args,
            resolved_type,
        } => match func {
            BuiltInFunction::Print => transpile_builtin_print(&args[0], &resolved_type, ctx),
            BuiltInFunction::Sum => transpile_builtin_sum(&args, ctx),
            BuiltInFunction::Timer => Ok("@intCast(std.time.milliTimestamp());".to_string()),
            BuiltInFunction::Number => {
                if args.len() != 1 {
                    return Err("Ədəd() funksiyası yalnız 1 arqument qəbul edir".to_string());
                }
                println!("Args {:?}", args);

                let inner = transpile_expr(&args[0], ctx)?;
                println!("Inner {}", inner);
                Ok(format!("try std.fmt.parseInt(usize, {}, 10)", inner))
            }

            BuiltInFunction::LastWord => {
                let print_code = transpile_builtin_print(&args[0], &resolved_type, ctx)?;
                Ok(format!("{}\n    std.process.exit(0);", print_code))
            }
            BuiltInFunction::Range => transpile_builtin_range(&args, ctx),

            BuiltInFunction::Input => {
                if args.len() != 1 {
                    return Err("giriş() yalnız 1 arqument qəbul edir".to_string());
                }

                ctx.used_input_fn = true;
                let prompt = transpile_expr(&args[0], ctx)?;
                let buf_name = "buf_temp"; // sabit buffer adı

                Ok(format!(
                    r#"(blk: {{
                var {buf}: [100]u8 = undefined;
                break :blk try input({prompt}, &{buf});
            }})"#,
                    buf = buf_name,
                    prompt = prompt
                ))
            }
            BuiltInFunction::Len => {
                let arg_code = transpile_expr(&args[0], ctx)?;
                Ok(format!("{}.len", arg_code))
            }
        },

        Expr::MutableDecl { name, typ, value } => transpile_mutable_decl(name, typ, value, ctx),
        Expr::ConstantDecl { name, typ, value } => transpile_constant_decl(name, typ, value, ctx),

        Expr::Bool(b) => Ok(b.to_string()),
        Expr::Number(n) => Ok(n.to_string()),
        Expr::Float(n) => Ok(n.to_string()),
        Expr::MethodCall {
            target,
            method,
            args,
        } => {
            let target_code = transpile_expr(target, ctx)?;
            let args_code: Result<Vec<String>, _> =
                args.iter().map(|arg| transpile_expr(arg, ctx)).collect();
            let args_code = args_code?;

            // Typeni artıq symboldan götürə bilərik
            let target_type = match &**target {
                Expr::VariableRef {
                    symbol: Some(sym), ..
                } => Some(&sym.typ),
                _ => None,
            };

            match target_type {
                Some(Type::Metn) => {
                    transpile_string_method_call(&target_code, method, &args_code, ctx)
                        .ok_or_else(|| format!("Dəstəklənməyən string metodu: {}", method))
                }
                Some(Type::Siyahi(_)) => {
                    let is_mutable = match &**target {
                        Expr::VariableRef {
                            symbol: Some(sym), ..
                        } => sym.is_mutable,
                        _ => false,
                    };
                    transpile_list_method_call(&target_code, method, &args_code, is_mutable, ctx)
                }
                Some(Type::Istifadeci(_)) => {
                    let joined_args = if args_code.is_empty() {
                        "".to_string()
                    } else {
                        format!(", {}", args_code.join(", "))
                    };

                    Ok(format!("{}.{}({})", target_code, method, joined_args))
                }
                _ => Err("MethodCall üçün naməlum və ya dəstəklənməyən tip.".to_string()),
            }
        }

        Expr::Return(e) => {
            let expr_code = transpile_expr(e, ctx)?;
            Ok(format!("return {}", expr_code))
        }
        Expr::FunctionCall {
            name,
            args,
            resolved_params,
            return_type,
        } => transpile_function_call(name, args, return_type.clone(), ctx),

        Expr::List(items) => {
            let items_code: Result<Vec<String>, String> =
                items.iter().map(|item| transpile_expr(item, ctx)).collect();
            let items_str = items_code?.join(", ");
            Ok(format!("[{}]", items_str))
        }

        Expr::String(s) => Ok(format!("\"{}\"", s.escape_default())),
        Expr::VariableRef { name, symbol } => {
            //name = "Qirmizi"

            if ctx
                .enum_defs
                .values()
                .any(|variants| variants.contains(name))
            {
                Ok(format!(".{}", name))
            } else {
                if let Some(sym) = symbol {
                    if sym.is_pointer {
                        Ok(format!("{}.*", name))
                    } else {
                        Ok(format!("{}", name))
                    }
                } else {
                    Ok(format!("{}", name))
                }
            }
        }
        Expr::FunctionDef {
            name,
            params,
            body,
            return_type,
            parent,
        } => transpile_function_def(name, params, body, return_type, parent, ctx),
    }
}

fn contains_self(expr: &Expr) -> bool {
    match expr {
        Expr::VariableRef { name, .. } => name == "self",

        Expr::BinaryOp { left, right, .. } => contains_self(left) || contains_self(right),

        Expr::MethodCall { target, args, .. } => {
            contains_self(target) || args.iter().any(contains_self)
        }

        Expr::FunctionCall { args, .. } => args.iter().any(contains_self),

        Expr::FieldAccess { target, .. } => contains_self(target),

        Expr::Assignment { value, .. } => contains_self(value),

        Expr::Index { target, index } => contains_self(target) || contains_self(index),

        Expr::Loop { iterable, body, .. } => {
            contains_self(iterable) || body.iter().any(contains_self)
        }

        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            contains_self(condition)
                || then_branch.iter().any(contains_self)
                || else_branch.iter().any(contains_self)
        }

        Expr::ElseIf {
            condition,
            then_branch,
        } => contains_self(condition) || then_branch.iter().any(contains_self),

        Expr::Else { then_branch } => then_branch.iter().any(contains_self),

        Expr::Return(inner) => contains_self(inner),

        Expr::List(items) => items.iter().any(contains_self),

        _ => false,
    }
}

fn indent(s: &str, level: usize) -> String {
    let indent_str = "    ".repeat(level);
    s.lines()
        .map(|line| format!("{}{}", indent_str, line))
        .collect::<Vec<_>>()
        .join("\n")
}
