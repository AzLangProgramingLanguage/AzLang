use crate::{
    parser::ast::{Expr, Parameter, Type},
    transpiler::{TranspileContext, transpile::transpile_expr},
};

pub fn get_expr_type<'a>(expr: &Expr<'a>) -> Type<'a> {
    match expr {
        Expr::String(_) => Type::Metn,
        Expr::Number(_) => Type::Integer,
        Expr::Float(_) => Type::Float,
        Expr::Bool(_) => Type::Bool,
        Expr::Char(_) => Type::Char,
        Expr::VariableRef { name: _, symbol } => symbol.as_ref().unwrap().typ.clone(),
        Expr::Index {
            target,
            index,
            target_type,
        } => target_type.clone(),
        Expr::List(items) => {
            if items.is_empty() {
                return Type::Siyahi(Box::new(Type::Any)); // boş siyahı – tipi bilinmir
            }
            let item_type = get_expr_type(&items[0]);

            for item in &items[1..] {
                let t = get_expr_type(item);
                if t != item_type {
                    return Type::Siyahi(Box::new(Type::Any)); // qarışıq tiplər
                }
            }

            Type::Siyahi(Box::new(item_type)) //item_type  mismatched types
            // expected Type, found &Type (rustc E0308)
        }

        /*         Expr::StructInit(_) => Type::Istifadeci(),
         */
        _ => Type::Any,
    }
}

pub fn get_format_str_from_type<'a>(t: &Type<'_>) -> &'a str {
    match t {
        Type::Metn => "{s}",
        Type::Integer | Type::BigInteger | Type::LowInteger => "{}",
        Type::Bool => "{}",
        Type::Char => "{c}",
        Type::Float => "{d}",
        Type::Void => "",
        Type::Any => "{any}",
        Type::Siyahi(_) => "{any} ",
        Type::Istifadeci(_) => "{any}",
    }
}
use std::borrow::Cow;

pub fn map_type<'a>(typ: &'a Type<'a>, is_const: bool) -> Cow<'a, str> {
    match typ {
        Type::Integer => Cow::Borrowed("isize"),
        Type::Any => Cow::Borrowed("any"),
        Type::Void => Cow::Borrowed("void"),
        Type::Float => Cow::Borrowed("f64"),
        Type::BigInteger => {
            if is_const {
                Cow::Borrowed("const i128")
            } else {
                Cow::Borrowed("i128")
            }
        }
        Type::Char => Cow::Borrowed("u8"),
        Type::LowInteger => {
            if is_const {
                Cow::Borrowed("const u8")
            } else {
                Cow::Borrowed("u8")
            }
        }
        Type::Metn => {
            if is_const {
                Cow::Borrowed("[]const u8")
            } else {
                Cow::Borrowed("[]u8")
            }
        }
        Type::Bool => Cow::Borrowed("bool"),
        Type::Siyahi(inner) => {
            let inner_str = map_type(inner, is_const);
            inner_str
        }
        Type::Istifadeci(name) => {
            Cow::Borrowed(name) // əgər `name: &'a str`-dirsə.
        }
    }
}

pub fn is_semicolon_needed(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Assignment { .. }
            | Expr::Break
            | Expr::Continue
            | Expr::Return(_)
            | Expr::Decl { .. }
            | Expr::Call { .. }
            | Expr::BuiltInCall { .. }
            | Expr::VariableRef { .. }
            | Expr::Index { .. }
            | Expr::BinaryOp { .. }
    )
}

pub fn transpile_function_def<'a>(
    name: &'a str,
    params: &'_ [Parameter<'a>],
    body: &'_ [Expr<'a>],
    return_type: &Option<Type<'_>>,
    parent: Option<&'a str>,
    ctx: &mut TranspileContext<'a>,
) -> String {
    let params_str: Vec<String> = params.iter().map(transpile_param).collect();

    let ret_type = return_type.as_ref().unwrap_or(&Type::Void);
    let ret_type_str = map_type(ret_type, true);

    let mut body_lines = Vec::new();
    for expr in body {
        let mut line = transpile_expr(expr, ctx);
        if is_semicolon_needed(expr) && !line.trim_start().starts_with("//") {
            line.push(';');
        }
        body_lines.push(format!("    {}", line));
    }

    format!(
        "fn {}({}) {} {{\n{}\n}}",
        name,
        params_str.join(", "),
        ret_type_str,
        body_lines.join("\n")
    )
}

fn transpile_param(param: &Parameter) -> String {
    let zig_type = map_type(&param.typ, !param.is_mutable);
    if param.is_mutable {
        format!("{}: *{}", param.name, zig_type)
    } else {
        format!("{}: {}", param.name, zig_type)
    }
}
