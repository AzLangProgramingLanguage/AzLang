use crate::{
    parser::ast::{Expr, Type},
    transpiler::{TranspileContext, helpers::get_expr_type, transpile::transpile_expr},
};

pub fn transpile_min<'a>(args: &'a [Expr<'a>], ctx: &mut TranspileContext<'a>) -> String {
    transpile_min_max(args, ctx, "min")
}

pub fn transpile_max<'a>(args: &'a [Expr<'a>], ctx: &mut TranspileContext<'a>) -> String {
    transpile_min_max(args, ctx, "max")
}

fn transpile_min_max<'a>(
    args: &'a [Expr<'a>],
    ctx: &mut TranspileContext<'a>,
    fn_name: &str,
) -> String {
    let list_expr = &args[0];
    let list_code = transpile_expr(list_expr, ctx);

    let inner_type = get_expr_type(list_expr);
    let type_code = inner_typer(inner_type);

    // flag-ları düzəldir
    match fn_name {
        "min" => ctx.used_min_fn = true,
        "max" => ctx.used_max_fn = true,
        _ => {}
    }

    let final_list_code = match list_expr {
        Expr::VariableRef {
            name: _,
            transpiled_name,
            symbol: Some(sym),
        } => {
            let transpiled_name = transpiled_name.as_ref().unwrap();
            if sym.is_mutable {
                format!("{transpiled_name}.items")
            } else {
                format!("&{transpiled_name}")
            }
        }
        _ => {
            if list_code.starts_with('[') && list_code.ends_with(']') {
                let stripped = &list_code[1..list_code.len() - 1];
                format!("&[_]{}{{ {} }}", type_code, stripped)
            } else {
                list_code
            }
        }
    };

    format!("{}({}, {})", fn_name, type_code, final_list_code)
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
