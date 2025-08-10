use crate::{
    parser::ast::{Expr, Type},
    transpiler::{TranspileContext, helpers::map_type, transpile::transpile_expr},
};

pub fn transpile_decl<'a>(
    name: &'a String,
    typ: Option<&Type<'a>>,
    is_mutable: bool,
    value: &'a Expr<'a>,
    ctx: &mut TranspileContext<'a>,
) -> String {
    let type_str = map_type(typ.unwrap_or(&Type::Any), !is_mutable);

    let value_code: String = transpile_expr(value, ctx);

    let decl_code = match typ {
        Some(Type::Metn) => {
            if is_mutable {
                format!(
                    "var {}: {} =  try allocator.dupe(u8, {})",
                    name, type_str, value_code
                )
            } else {
                format!("const {}: {} = {}", name, type_str, value_code)
            }
        }
        Some(Type::Siyahi(inner)) => match value {
            Expr::List(items) => {
                let items_code: Vec<String> =
                    items.iter().map(|i| transpile_expr(i, ctx)).collect();
                let items_str = items_code.join(", ");
                if is_mutable {
                    ctx.needs_allocator = true;
                    ctx.cleanup_statements.push(format!("{}.deinit();", name));

                    let inner_code = map_type(inner, false);

                    format!(
                        r#"var {name} = try std.ArrayList({inner}).initCapacity(allocator, {cap});
try {name}.appendSlice(&[_]{inner}{{ {items} }});"#,
                        name = name,
                        inner = inner_code,
                        cap = items.len(),
                        items = items_str
                    )
                } else {
                    format!("const {} = [_]{}{{ {} }}", name, type_str, items_str)
                }
            }
            _ => todo!(),
        },
        _ => {
            if is_mutable {
                format!("var {}: {} = {}", name, type_str, value_code)
            } else {
                format!("const {}: {} = {}", name, type_str, value_code)
            }
        }
    };

    decl_code
}
