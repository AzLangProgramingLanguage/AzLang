use crate::{
    context::TranspileContext,
    expr::transpile_expr,
    parser::{Expr, ast::Type},
};
pub fn transpile_builtin_sum(args: &[Expr], ctx: &mut TranspileContext) -> Result<String, String> {
    let list_expr = &args[0];
    let list_code = transpile_expr(list_expr, ctx)?;

    // Siyahının tipini AST-dən oxuyuruq
    let inner_type = match list_expr {
        Expr::VariableRef {
            symbol: Some(sym), ..
        } => match &sym.typ {
            Type::Siyahi(boxed) => boxed.clone(),
            _ => return Err("sum() yalnız siyahılar üçün keçərlidir".to_string()),
        },
        Expr::BuiltInCall {
            resolved_type: Some(Type::Siyahi(boxed)),
            ..
        } => boxed.clone(),
        _ => {
            return Err(
                "sum() üçün siyahı tipi təyin edilə bilmədi və ya düzgün AST verilməyib"
                    .to_string(),
            );
        }
    };

    // İcazə verilən tip kodları
    let type_code = match *inner_type {
        Type::Integer => "usize",
        Type::LowInteger => "u8",
        Type::BigInteger => "i128",
        _ => return Err("sum() yalnız rəqəm siyahıları üçün işləyir".to_string()),
    };

    ctx.used_sum_fn = true;

    // Kod çıxarışı
    let final_list_code = match list_expr {
        Expr::VariableRef {
            name,
            symbol: Some(sym),
        } => {
            if sym.is_mutable {
                format!("{}.items", name)
            } else {
                name.clone()
            }
        }
        _ => {
            if list_code.starts_with('[') && list_code.ends_with(']') {
                let stripped = &list_code[1..list_code.len() - 1];
                format!("&[_]{}{{ {} }}", type_code, stripped)
            } else {
                list_code.clone()
            }
        }
    };

    Ok(format!("sum({}, {})", type_code, final_list_code))
}
