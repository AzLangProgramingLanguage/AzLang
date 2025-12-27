use parser::{ast::Expr, shared_ast::Type};

use crate::{TranspileContext, helper::get_expr_type, transpile::transpile_expr};

pub fn transpile_min<'a>(args: &mut Vec<Expr<'a>>, ctx: &mut TranspileContext<'a>) -> String {
    transpile_min_max(args, ctx, "min")
}

pub fn transpile_max<'a>(args: &mut Vec<Expr<'a>>, ctx: &mut TranspileContext<'a>) -> String {
    transpile_min_max(args, ctx, "max")
}

fn transpile_min_max<'a>(
    args: &mut Vec<Expr<'a>>,
    ctx: &mut TranspileContext<'a>,
    fn_name: &str,
) -> String {
    let list_expr = args.remove(0);

    let final_list_code = match list_expr {
        Expr::VariableRef {
            name,
            symbol: Some(sym),
        } => {
            let transpiled_name = name;
            if sym.is_mutable {
                format!("{transpiled_name}.items")
            } else {
                format!("&{transpiled_name}")
            }
        }
        _ => {
            let inner_type = get_expr_type(&list_expr);

            let list_code = transpile_expr(list_expr, ctx);

            let type_code = inner_typer(inner_type);

            if list_code.starts_with('[') && list_code.ends_with(']') {
                let stripped = &list_code[1..list_code.len() - 1];
                format!("&[_]{}{{ {} }}", type_code, stripped)
            } else {
                list_code
            }
        }
    };

    match fn_name {
        "min" => ctx.used_min_fn = true,
        "max" => ctx.used_max_fn = true,
        _ => {}
    }

    format!("{}( {})", fn_name, final_list_code) /*TODO: TypeCOde YOxdu */
}

pub fn inner_typer(inner_type: Type<'_>) -> &'static str {
    match inner_type {
        Type::Array(inner) => inner_typer(*inner),
        Type::Integer => "usize",
        Type::LowInteger => "u8",
        Type::BigInteger => "i128",
        _ => "/* unsupported list element type */",
    }
}
