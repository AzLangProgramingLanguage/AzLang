use std::borrow::Cow;

use crate::{
    parser::ast::{Expr, Parameter, Type},
    transpiler::{
        helpers::{is_semicolon_needed, map_type},
        transpile::transpile_expr,
    },
};

use super::TranspileContext;
type MethodType<'a> = Vec<(&'a str, Vec<Parameter<'a>>, Vec<Expr<'a>>, Option<Type<'a>>)>;
pub fn transpile_union_def<'a>(
    name: &'a str,
    fields: &Vec<(&str, Type<'_>)>,
    methods: &MethodType<'a>,
    ctx: &mut TranspileContext<'a>,
) -> String {
    let old_struct = ctx.current_struct.clone();

    // ctx.struct_defsc
    // .insert(Cow::Borrowed(name), Cow::Owned(fields.clone()));

    ctx.current_struct = Some(name);
    let field_lines: Vec<String> = fields
        .iter()
        .map(|(fname, ftype)| {
            let zig_type = map_type(ftype, true);
            format!("    {fname}: {zig_type},")
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
                format!("{self_prefix}{params_zig}")
            };

            let ret_type = return_type
                .as_ref()
                .map(|t| map_type(t, true))
                .unwrap_or(Cow::Borrowed("void"));

            let header = format!("pub fn {method_name}({all_params})  {ret_type} {{");
            let body_lines: Vec<String> = body
                .iter()
                .filter_map(|expr| {
                    /*                     let old_struct = ctx.current_struct.clone();
                     */
                    let line = transpile_expr(expr, ctx);
                    if is_semicolon_needed(expr) && !line.trim_start().starts_with("//") {
                        Some(format!("{line};"))
                    } else {
                        Some(line)
                    }
                })
                .map(|line| format!("        {line}"))
                .collect();
            format!("{header}\n{}\n    }}", body_lines.join("\n"))
        })
        .collect::<Vec<_>>();

    let mut all_lines = field_lines;
    all_lines.push("".to_string()); // boş sətr
    all_lines.extend(method_lines);
    let full_body = all_lines.join("\n");
    ctx.current_struct = old_struct;

    format!("const {name} = union(enum) {{\n{full_body}\n}};")
}
