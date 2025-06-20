// string_methods.rs

use crate::context::TranspileContext;
pub fn transpile_string_method_call(
    target_code: &str,
    method: &str,
    args_code: &[String],
    ctx: &mut TranspileContext,
) -> Option<String> {
    match method {
        "böyüt" => {
            ctx.needs_allocator = true;
            Some(format!(
                r#"
    for ({target}) |*ch| {{
        ch.* = std.ascii.toUpper(ch.*);
    }}
    "#,
                target = target_code
            ))
        }

        "böl" => {
            if args_code.len() == 1 {
                let delimiter = &args_code[0];
                if target_code.starts_with('"') && target_code.ends_with('"') {
                    ctx.used_split_n_fn = true;
                    println!("{}", target_code);
                    let max_parts_expr = format!("{}", target_code.len());
                    Some(format!(
                        r#"({{
    var result = splitN({}, {}, {});
    result.parts[0..result.len];
}})"#,
                        target_code, delimiter, max_parts_expr
                    ))
                } else {
                    ctx.used_split_auto_fn = true;
                    ctx.needs_allocator = true;
                    let delimiter = delimiter.replace("\"", "\'");
                    Some(format!(
                        r#"try splitAuto(allocator, {}, {})"#,
                        target_code, delimiter
                    ))
                }
            } else {
                None
            }
        }

        "kiçilt" => {
            ctx.needs_allocator = true;
            Some(format!(
                r#"
    for ({target}) |*ch| {{
        ch.* = std.ascii.toLower(ch.*);
    }}
    "#,
                target = target_code
            ))
        }

        "kənar_təmizlə" => Some(format!("{}.trim()", target_code)),

        "uzunluq" => Some(format!("{}.len()", target_code)),

        "boşdur" => Some(format!("{}.is_empty()", target_code)),

        "sırala" => {
            if args_code.is_empty() {
                Some(format!(
                    r#"std.mem.sort(u8, {0}.items, {}, std.sort.asc(u8));"#,
                    target_code
                ))
            } else {
                None
            }
        }

        "əvəzlə" => {
            if args_code.len() == 2 {
                ctx.needs_allocator = true;
                Some(format!(
                    r#"
    {target} = try std.mem.replaceOwned(u8,allocator, {target}, {old}, {new});
    "#,
                    target = target_code,
                    old = args_code[0],
                    new = args_code[1]
                ))
            } else {
                None
            }
        }

        "kəs" => {
            if args_code.len() == 2 {
                ctx.needs_allocator = true;
                Some(format!(
                    r#"
    {target} = try allocator.dupe(u8, {target}[{start}..{end}]);
    "#,
                    target = target_code,
                    start = args_code[0],
                    end = args_code[1]
                ))
            } else {
                None
            }
        }

        "birləşdir" => {
            ctx.needs_allocator = true;
            Some(format!(
                r#"
    {target} = try std.mem.concat(allocator, u8, &[_][]const u8{{ {target}, {arg} }});
    "#,
                target = target_code,
                arg = args_code[0]
            ))
        }

        _ => None,
    }
}
