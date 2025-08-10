use crate::{
    parser::ast::Expr,
    transpiler::{
        TranspileContext, builtinfunctions::min_max::inner_typer, helpers::get_expr_type,
        transpile::transpile_expr,
    },
};

pub fn transpile_sum<'a>(args: &'a [Expr<'a>], ctx: &mut TranspileContext<'a>) -> String {
    ctx.used_sum_fn = true;

    let expr_type = get_expr_type(&args[0]);
    let list_code = {
        if args.len() > 1 {
            let mut final_list_code = String::from("&[_]usize{");
            for arg in args {
                final_list_code.push_str(&transpile_expr(arg, ctx));
                final_list_code.push_str(", ");
            }
            final_list_code.pop();
            final_list_code.pop();
            final_list_code.push('}');
            final_list_code
        } else {
            transpile_expr(&args[0], ctx)
        }
    };
    let inner_type = inner_typer(expr_type);
    let final_list_code = match &args[0] {
        Expr::VariableRef {
            name,
            transpiled_name: _,
            symbol: Some(sym),
        } => {
            if sym.is_mutable {
                format!("{}.items", name)
            } else {
                format!("&{}", name)
            }
        }
        _ => {
            if list_code.starts_with('[') && list_code.ends_with(']') {
                let stripped = &list_code[1..list_code.len() - 1];
                format!("&[_]{}{{ {} }}", inner_type, stripped)
            } else {
                list_code
            }
        }
    };
    format!("sum({}, {})", inner_type, final_list_code)
}
