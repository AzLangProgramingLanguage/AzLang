use std::borrow::Cow;

use parser::{shared_ast::Type, typed_ast::MethodTypeTyped};

use crate::transpiler::{
    helper::{is_semicolon_needed, map_type},
    transpile::transpile_expr,
};

use super::TranspileContext;

pub fn transpile_union_def<'a>(
    name: &'a str,
    transpiled_name: &'a str,
    fields: &'a Vec<(&str, Type<'_>)>,
    methods: &'a Vec<MethodTypeTyped<'a>>,
    ctx: &mut TranspileContext<'a>,
) -> String {
    let old_union = ctx.current_union.clone();

    ctx.current_union = Some(transpiled_name);
    let field_lines: Vec<String> = fields
        .iter()
        .map(|(fname, ftype)| {
            let zig_type = map_type(ftype, true);
            format!("    {fname}: {zig_type},")
        })
        .collect();

    let method_lines: Vec<String> = methods
        .iter()
        .map(|method| {
            let uses_self = true;
            let param_list: Vec<String> = method
                .params
                .iter()
                .filter(|p| p.name != "self") // self ayrıca işlənəcək
                .map(|p| format!("{}: {}", p.name, map_type(&p.typ, true)))
                .collect();
            let is_allocator_used = method.is_allocator;
            let mut prefix = String::new();
            if uses_self {
                prefix.push_str("self: @This(),");
            }
            if is_allocator_used {
                prefix.push_str("allocator: std.mem.Allocator");
            }

            let params_zig = if !param_list.is_empty() {
                if uses_self {
                    format!(", {}", param_list.join(", "))
                } else {
                    param_list.join(", ")
                }
            } else {
                "".to_string()
            };

            let all_params = if prefix.is_empty() {
                params_zig
            } else if params_zig.is_empty() {
                prefix.to_string()
            } else {
                format!("{prefix}{params_zig}")
            };

            let ret_type = method
                .return_type
                .as_ref()
                .map(|t| map_type(t, true))
                .unwrap_or(Cow::Borrowed("void"));

            let body_lines: Vec<String> = method
                .body
                .iter()
                .filter_map(|expr| {
                    let line = transpile_expr(expr, ctx);
                    if is_semicolon_needed(expr) && !line.trim_start().starts_with("//") {
                        Some(format!("{line};"))
                    } else {
                        Some(line)
                    }
                })
                .map(|line| format!("        {line}"))
                .collect();
            ctx.is_used_self = false;
            let header;
            if ctx.is_used_allocator {
                header = format!(
                    "pub fn {}({all_params})  !{ret_type} {{",
                    method.transpiled_name.as_ref().unwrap()
                );
            } else {
                header = format!(
                    "pub fn {}({all_params})  {ret_type} {{",
                    method.transpiled_name.as_ref().unwrap()
                );
            }
            ctx.is_used_allocator = false;

            format!("{header}\n{}\n    }}", body_lines.join("\n"))
        })
        .collect::<Vec<_>>();

    let mut all_lines = field_lines;
    all_lines.push("".to_string()); // boş sətr
    all_lines.extend(method_lines);
    let full_body = all_lines.join("\n");
    ctx.current_union = old_union;

    let new_name = transpiled_name;
    format!("const {new_name} = union(enum) {{\n{full_body}\n}};")
}
