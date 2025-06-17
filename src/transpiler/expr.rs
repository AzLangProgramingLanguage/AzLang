use super::context::TranspileContext;
use crate::array_methods::transpile_list_method_call;
use crate::declaration::transpile_constant_decl;
use crate::function::{transpile_function_call, transpile_function_def};
use crate::r#loop::transpile_loop;
use crate::parser::ast::Type;
use crate::parser::{Expr, ast::BuiltInFunction};
use crate::string_methods::transpile_string_method_call;
use crate::transpiler::declaration::transpile_mutable_decl;
use crate::transpiler::utils::{
    get_expr_type, transpile_builtin_print, transpile_builtin_range, transpile_builtin_sum,
};
pub fn transpile_expr(expr: &Expr, ctx: &mut TranspileContext) -> Result<String, String> {
    match expr {
        Expr::Index { target, index } => {
            let target_code = transpile_expr(target, ctx)?;
            let index_code = transpile_expr(index, ctx)?;
            Ok(format!("{}[{}]", target_code, index_code))
        }
        Expr::Break => Ok("break;".to_string()),
        Expr::Continue => Ok("continue;".to_string()),
        Expr::BinaryOp { left, op, right } => {
            let left_code = transpile_expr(left, ctx)?;
            let right_code = transpile_expr(right, ctx)?;

            if op == "=" {
                Ok(format!("{} = {};", left_code, right_code))
            } else {
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
                    // Add semicolon for each statement in then branch
                    if !code.ends_with(';') {
                        Ok(format!("{}", code))
                    } else {
                        Ok(code)
                    }
                })
                .collect();
            let then_code = then_code?.join("\n    "); // Indent with 4 spaces

            let else_code = if let Some(branch) = else_branch {
                let else_lines: Result<Vec<String>, String> = branch
                    .iter()
                    .map(|e| {
                        let code = transpile_expr(e, ctx)?;
                        // Add semicolon for each statement in else branch
                        if code.ends_with(';') {
                            Ok(code)
                        } else {
                            Ok(format!("{}", code))
                        }
                    })
                    .collect();
                let else_code = else_lines?.join("\n    "); // Indent with 4 spaces
                format!("\nelse {{\n    {}\n}}", else_code)
            } else {
                "".to_string()
            };

            Ok(format!(
                "if ({}) {{\n    {}\n}}{}",
                condition_code, then_code, else_code
            ))
        }
        Expr::Loop {
            var_name,
            iterable,
            body,
        } => transpile_loop(var_name, iterable, body, ctx),
        Expr::BuiltInCall { func, args } => match func {
            BuiltInFunction::Print => transpile_builtin_print(&args[0], ctx),
            BuiltInFunction::Sum => transpile_builtin_sum(&args, ctx),
            BuiltInFunction::Number => match &args[0] {
                Expr::String(s) => Ok(s.clone()),
                Expr::FunctionCall { .. } | Expr::VariableRef(_) => {
                    let inner = transpile_expr(&args[0], ctx)?;
                    Ok(format!("{}.parse::<i32>().unwrap()", inner))
                }
                _ => Ok("0".to_string()),
            },
            BuiltInFunction::LastWord => {
                let print_code = transpile_builtin_print(&args[0], ctx)?;
                Ok(format!("{}\n    std.process.exit(0);", print_code))
            }
            BuiltInFunction::Range => transpile_builtin_range(&args, ctx),

            BuiltInFunction::Input => Ok("".to_string()),

            BuiltInFunction::Len => {
                let arg_code = transpile_expr(&args[0], ctx)?;
                Ok(format!("{}.len", arg_code))
            }
        },

        Expr::MutableDecl { name, typ, value } => transpile_mutable_decl(name, typ, value, ctx),
        Expr::ConstantDecl { name, typ, value } => transpile_constant_decl(name, typ, value, ctx),

        Expr::Bool(b) => Ok(b.to_string()),
        Expr::Number(n) => Ok(n.to_string()),
        Expr::MethodCall {
            target,
            method,
            args,
        } => {
            let target_code = transpile_expr(target, ctx)?;
            let args_code: Result<Vec<String>, String> =
                args.iter().map(|arg| transpile_expr(arg, ctx)).collect();
            let args_code = args_code?;
            let target_type = get_expr_type(target, ctx);

            match target_type {
                Some(Type::Metn) => {
                    transpile_string_method_call(&target_code, method, &args_code, ctx)
                        .ok_or_else(|| format!("Dəstəklənməyən string metodu: {}", method))
                }

                Some(Type::Siyahi(_)) => {
                    let is_mutable = ctx.mutable_symbols.contains(&target_code);
                    let code =
                        transpile_list_method_call(&target_code, method, &args_code, is_mutable)?;
                    Ok(code)
                }

                _ => Err(format!(
                    "MethodCall üçün dəstəklənməyən və ya məlum olmayan target tipi: {:?}",
                    target_type
                )),
            }
        }
        Expr::Return(e) => {
            let expr_code = transpile_expr(e, ctx)?;
            Ok(format!("return {}", expr_code))
        }
        Expr::FunctionCall { name, args } => transpile_function_call(name, args, ctx),

        Expr::List(items) => {
            let items_code: Result<Vec<String>, String> =
                items.iter().map(|item| transpile_expr(item, ctx)).collect();
            let items_str = items_code?.join(", ");
            Ok(format!("[{}]", items_str))
        }

        Expr::String(s) => Ok(format!("\"{}\"", s.escape_default())),
        Expr::VariableRef(name) => Ok(name.clone()),

        Expr::FunctionDef { name, params, body } => transpile_function_def(name, params, body, ctx),
    }
}
