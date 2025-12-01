use std::borrow::Cow;

use parser::{
    shared_ast::Type,
    typed_ast::{MethodTypeTyped, TypedExpr},
};

use crate::transpiler::{
    TranspileContext,
    helper::{is_semicolon_needed, map_type},
    transpile::transpile_expr,
};

pub fn transpile_struct_def<'a>(
    name: &'a str,
    transpiled_name: &'a str,
    fields: &'a Vec<(&str, Type<'a>, Option<TypedExpr<'a>>)>,
    methods: &'a Vec<MethodTypeTyped<'a>>,
    ctx: &mut TranspileContext<'a>,
) -> String {
    let old_struct = ctx.current_struct.clone();

    ctx.current_struct = Some(transpiled_name);
    let field_lines: Vec<String> = fields
        .iter()
        .map(|(fname, ftype, value)| {
            let zig_type = map_type(ftype, true);
            if let Some(val) = value {
                let transpiled = transpile_expr(val, ctx);
                format!("    {}: {}={},", fname, zig_type, transpiled)
            } else {
                format!("    {}: {},", fname, zig_type)
            }
        })
        .collect();

    let method_lines: Vec<String> = methods
        .iter()
        .map(|method| {
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
            let param_list: Vec<String> = method
                .params
                .iter()
                .filter(|p| p.name != "self") // self ayrıca işlənəcək
                .map(|p| format!("{}: {}", p.name, map_type(&p.typ, true)))
                .collect();
            let uses_self = ctx.is_used_self;

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
            let ret_type = method
                .return_type
                .as_ref()
                .map(|t| map_type(t, true))
                .unwrap_or(Cow::Borrowed("void"));
            let header = format!("pub fn {}({all_params}) {} {{ ", method.name, ret_type);
            ctx.is_used_self = false;
            format!("{header}\n{}\n    }}", body_lines.join("\n"))
        })
        .collect::<Vec<_>>();

    let mut all_lines = field_lines;
    all_lines.push("".to_string());
    all_lines.extend(method_lines);
    let full_body = all_lines.join("\n");
    ctx.current_struct = old_struct;
    let t = transpiled_name;
    format!("const {t} = struct {{\n{full_body}\n}};")
}
