use std::rc::Rc;

use parser::{ast::Expr, shared_ast::Type};

use crate::{TranspileContext, helper::map_type, transpile::transpile_expr};
pub fn transpile_decl<'a>(
    name: &String,
    typ: Rc<Type<'a>>,
    is_mutable: bool,
    value: Expr<'a>,
    ctx: &mut TranspileContext<'a>,
) -> String {
    let type_str = map_type(&typ, !is_mutable);
let value_code: String = transpile_expr(value, ctx);

    let decl_code = if is_mutable {
        format!("var {}: {} = {}", name, type_str, value_code)
    } else {
        format!("const {}: {} = {}", name, type_str, value_code)
    };

    decl_code
}

/*   let decl_code = match typ {
    Type::String => match value {
        /*             Expr::String(_) => {
                       if is_mutable {
                           format!(
                               "var {}: {} = azlangYazi.Yeni(azlangYazi{{.Mut=try allocator.dupe(u8, {}) }});",
                               name, type_str, value_code
                           )
                       } else {
                           format!(
                               "const {}: {} = azlangYazi.Yeni(azlangYazi{{.Const={}}});",
                               name, type_str, value_code
                           )
                       }
                   }
        */
        Expr::BuiltInCall {
            function,
            args: s,
            return_type: _,
        } => match function {
            BuiltInFunction::StrReverse => {
                let arg_code = transpile_expr(&s[0], ctx);
                if is_mutable {
                    format!(
                        "var {}: {} = try str_reverse(allocator, {}, true)",
                        name, type_str, arg_code
                    )
                } else {
                    format!(
                        "const {}: {} = try str_reverse(allocator, {}, false)",
                        name, type_str, arg_code
                    )
                }
            }
            BuiltInFunction::StrLower => {
                let arg_code = transpile_expr(&s[0], ctx);
                if is_mutable {
                    format!(
                        "var {}: {} = try str_lowercase(allocator, {}, true)",
                        name, type_str, arg_code
                    )
                } else {
                    format!(
                        "const {}: {} = try str_lowercase(allocator, {}, false)",
                        name, type_str, arg_code
                    )
                }
            }
            BuiltInFunction::StrUpper => {
                let arg_code = transpile_expr(&s[0], ctx);
                if is_mutable {
                    format!(
                        "var {}: {} = try str_uppercase(allocator, {}, true)",
                        name, type_str, arg_code
                    )
                } else {
                    format!(
                        "const {}: {} = try str_uppercase(allocator, {}, false)",
                        name, type_str, arg_code
                    )
                }
            }
            _ => {
                if is_mutable {
                    format!("var {}: {} = {}", name, type_str, value_code)
                } else {
                    format!("const {}: {} = {}", name, type_str, value_code)
                }
            }
        },

        _ => {
            if is_mutable {
                format!("var {}: {} = {}", name, type_str, value_code)
            } else {
                format!("const {}: {} = {}", name, type_str, value_code)
            }
        }
    },
    Type::Natural | Type::Integer | Type::Float => match value {
        Expr::Number(_) | Expr::UnaryOp { op: _, expr: _ } | Expr::Float(_) => {
            let var_code = if is_mutable { "var" } else { "const" };
            match typ {
                Type::Natural => {
                    format!(
                        "{} {}: {} = azlangEded.Yeni(azlangEded{{.natural = {}}});",
                        var_code, name, type_str, value_code
                    )
                }
                Type::Integer => {
                    format!(
                        "{} {}: {} = azlangEded.Yeni(azlangEded{{.integer = {}}});",
                        var_code, name, type_str, value_code
                    )
                }
                Type::Float => {
                    format!(
                        "{} {}: {} = azlangEded.Yeni(azlangEded{{.float = {}}});",
                        var_code, name, type_str, value_code
                    )
                }
                _ => todo!(),
            }
        }
        Expr::BuiltInCall {
            function,
            args: s,
            return_type: _,
        } => match function {
            BuiltInFunction::Timer => {
                let value_code = transpile_expr(value, ctx);
                let var_code = if is_mutable { "var" } else { "const" };
                match typ {
                    Type::Natural => {
                        format!(
                            "{} {}: {} = azlangEded.Yeni(azlangEded{{.natural = {}}});",
                            var_code, name, type_str, value_code
                        )
                    }
                    Type::Integer => {
                        format!(
                            "{} {}: {} = azlangEded.Yeni(azlangEded{{.integer = {}}});",
                            var_code, name, type_str, value_code
                        )
                    }
                    Type::Float => {
                        format!(
                            "{} {}: {} = azlangEded.Yeni(azlangEded{{.float = {}}});",
                            var_code, name, type_str, value_code
                        )
                    }
                    _ => todo!("Buraya çatdım"),
                }
            }
            _ => todo!("Burası neresi"),
        },
        _ => {
            if is_mutable {
                format!("var {}: {} = {}", name, type_str, value_code)
            } else {
                format!("const {}: {} = {}", name, type_str, value_code)
            }
        }
    },

    Type::Array(inner) => match value {
        Expr::List(items) => {
            let items_code: Vec<String> =
                items.iter().map(|i| transpile_expr(i, ctx)).collect();
            let items_str = items_code.join(", ");
            if is_mutable {
                /*    ctx.needs_allocator = true;
                ctx.cleanup_statements.push(format!("{}.deinit();", name)); */

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
}; */
