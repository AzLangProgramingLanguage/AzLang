use crate::{
    parser::ast::{Expr, Parameter, Type},
    transpiler::{TranspileContext, transpile::transpile_expr},
};

pub fn get_expr_type<'a>(expr: &Expr<'a>) -> Type<'a> {
    match expr {
        Expr::String(_, _) => Type::Metn,
        Expr::Number(_) => Type::Integer,
        Expr::Float(_) => Type::Float,
        Expr::Bool(_) => Type::Bool,
        Expr::Char(_) => Type::Char,

        Expr::Call {
            target,
            name,
            transpiled_name,
            args,
            returned_type,
            is_allocator,
        } => returned_type.as_ref().unwrap().clone(),
        Expr::VariableRef {
            name: _,
            transpiled_name: _,
            symbol,
        } => symbol.as_ref().unwrap().typ.clone(),
        Expr::BuiltInCall {
            function: _,
            args: _,
            return_type,
        } => return_type.clone(),
        Expr::Index {
            target: _,
            index: _,
            target_type,
        } => target_type.clone(),
        Expr::List(items) => {
            if items.is_empty() {
                return Type::Siyahi(Box::new(Type::Any));
            }
            let item_type = get_expr_type(&items[0]);

            for item in &items[1..] {
                let t = get_expr_type(item);
                if t != item_type {
                    return Type::Siyahi(Box::new(Type::Any));
                }
            }

            Type::Siyahi(Box::new(item_type))
        }

        _ => Type::Any,
    }
}

pub fn get_format_str_from_type<'a>(t: &Type<'_>, is_allocator: bool) -> &'a str {
    match t {
        Type::Metn => {
            if is_allocator {
                "{!s}"
            } else {
                "{s}"
            }
        }

        Type::ZigNatural
        | Type::ZigInteger
        | Type::Integer
        | Type::BigInteger
        | Type::LowInteger => {
            if is_allocator {
                "{!}"
            } else {
                "{}"
            }
        }
        Type::Bool => {
            if is_allocator {
                "{!}"
            } else {
                "{}"
            }
        }
        Type::Char => {
            if is_allocator {
                "{!c}"
            } else {
                "{c}"
            }
        }
        Type::Float => {
            if is_allocator {
                "{!d}"
            } else {
                "{d}"
            }
        }
        Type::Void => "",
        Type::Natural => {
            if is_allocator {
                "{!}"
            } else {
                "{}"
            }
        }
        Type::Allocator => "",
        Type::Any => "{any}",
        Type::Siyahi(_) => "{any}",
        Type::Istifadeci(_, _) => {
            if is_allocator {
                "{!any}"
            } else {
                "{any}"
            }
        }
        Type::ZigString => {
            if is_allocator {
                "{!s}"
            } else {
                "{s}"
            }
        }
        Type::ZigConstArray => "{any}",
        Type::ZigArray => "{any}",
        Type::ZigConstString => {
            if is_allocator {
                "{!s}"
            } else {
                "{s}"
            }
        }
    }
}

use std::borrow::Cow;

pub fn map_type<'a>(typ: &'a Type<'a>, is_const: bool) -> Cow<'a, str> {
    match typ {
        Type::Integer => Cow::Borrowed("azlangEded"),
        Type::Natural => Cow::Borrowed("azlangEded"),
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
                Cow::Borrowed("azlangYazi")
            } else {
                Cow::Borrowed("azlangYazi")
            }
        }
        Type::ZigString => Cow::Borrowed("[]u8"),
        Type::ZigNatural => Cow::Borrowed("usize"),
        Type::ZigInteger => Cow::Borrowed("isize"),
        Type::ZigConstString => Cow::Borrowed("[]const u8"),
        Type::ZigArray => Cow::Borrowed("[]usize"),
        Type::ZigConstArray => Cow::Borrowed("[]const usize"),
        Type::Bool => Cow::Borrowed("bool"),
        Type::Siyahi(inner) => {
            let inner_str = map_type(inner, is_const);
            inner_str
        }
        Type::Istifadeci(_, s) => Cow::Borrowed(s),
        Type::Allocator => Cow::Borrowed("std.mem.Allocator"),
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
    body: &'a [Expr<'a>],
    return_type: &Option<Type<'_>>,
    _parent: Option<&'a str>,
    ctx: &mut TranspileContext<'a>,
    is_allocator: &bool,
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

pub fn is_muttable<'a>(expr: &'a Expr<'a>) -> bool {
    match expr {
        Expr::VariableRef {
            name: _,
            transpiled_name: _,
            symbol,
        } => {
            if let Some(sym) = symbol {
                return sym.is_mutable;
            }
        }
        Expr::Call { target, .. } => match target {
            Some(boxed_expr) => match &**boxed_expr {
                Expr::VariableRef {
                    name: _,
                    transpiled_name: _,
                    symbol,
                } => {
                    if let Some(sym) = symbol {
                        return sym.is_mutable;
                    }
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }
    false
}
