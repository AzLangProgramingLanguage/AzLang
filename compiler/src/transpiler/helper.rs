use parser::{
    shared_ast::Type,
    typed_ast::{ParameterTyped, TypedExpr},
};

use crate::transpiler::{TranspileContext, transpile::transpile_expr};

pub fn get_expr_type<'a>(expr: &TypedExpr<'a>) -> Type<'a> {
    match expr {
        TypedExpr::String(_, _) => Type::String,
        TypedExpr::Number(_) => Type::Integer,
        TypedExpr::Float(_) => Type::Float,
        TypedExpr::Bool(_) => Type::Bool,
        TypedExpr::Char(_) => Type::Char,
        TypedExpr::UnaryOp { op, expr } => match *op {
            "!" => Type::Bool,
            "-" => Type::Integer,
            "+" => Type::Natural,
            _ => {
                panic!("get_expr_type de bilinmeyen unaryOp tipi geldi ")
            }
        },

        TypedExpr::Call {
            target,
            name,
            transpiled_name,
            args,
            returned_type,
            is_allocator,
        } => returned_type.as_ref().unwrap().clone(),
        TypedExpr::VariableRef {
            name: _,
            transpiled_name: _,
            symbol,
        } => symbol.as_ref().unwrap().typ.clone(),
        TypedExpr::BuiltInCall {
            function: _,
            args: _,
            return_type,
        } => return_type.clone(),

        TypedExpr::BinaryOp { left, op, right } => {
            let left_type = get_expr_type(left);
            let right_type = get_expr_type(right);

            // Əgər operandların tipi uyğun gəlmir

            let comparison_ops = ["==", "!=", "<", "<=", ">", ">="];
            let logic_ops = ["&&", "||"];
            let arithmetic_ops = ["+", "-", "*", "/", "%"];

            if comparison_ops.contains(&op) || logic_ops.contains(&op) {
                return Type::Bool;
            }

            if arithmetic_ops.contains(&op) {
                if left_type == Type::Integer && right_type == Type::Integer {
                    return Type::Integer;
                } else if left_type == Type::Natural && right_type == Type::Natural {
                    return Type::Natural;
                } else if left_type == Type::Float && right_type == Type::Float {
                    return Type::Float;
                } else if left_type == Type::Integer && right_type == Type::Natural {
                    return Type::Integer;
                } else if left_type == Type::Natural && right_type == Type::Integer {
                    return Type::Integer;
                } else {
                    return Type::Float;
                }
            }
            Type::Any
        }
        TypedExpr::Index {
            target: _,
            index: _,
            target_type,
        } => target_type.clone(),
        TypedExpr::List(items) => {
            if items.is_empty() {
                return Type::Array(Box::new(Type::Any));
            }
            let item_type = get_expr_type(&items[0]);

            for item in &items[1..] {
                let t = get_expr_type(item);
                if t != item_type {
                    return Type::Array(Box::new(Type::Any));
                }
            }

            Type::Array(Box::new(item_type))
        }

        _ => Type::Any,
    }
}

pub fn get_format_str_from_type<'a>(t: &Type<'_>, is_allocator: bool) -> &'a str {
    match t {
        Type::String => {
            if is_allocator {
                "{!s}"
            } else {
                "{s}"
            }
        }
        Type::ZigFloat => {
            if is_allocator {
                "{!d}"
            } else {
                "{d}"
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
        Type::Array(_) => "{any}",
        Type::User(_) => {
            if is_allocator {
                "{!any}"
            } else {
                "{any}"
            }
        }
        Type::LiteralString => {
            if is_allocator {
                "{!s}"
            } else {
                "{s}"
            }
        }
        Type::ZigConstArray => "{any}",
        Type::ZigArray => "{any}",
        Type::LiteralConstString => {
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
        Type::ZigFloat => Cow::Borrowed("f64"),
        Type::Float => Cow::Borrowed("azlangEded"),
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
        Type::String => {
            if is_const {
                Cow::Borrowed("azlangYazi")
            } else {
                Cow::Borrowed("azlangYazi")
            }
        }
        Type::LiteralString => Cow::Borrowed("[]u8"),
        Type::ZigNatural => Cow::Borrowed("usize"),
        Type::ZigInteger => Cow::Borrowed("isize"),
        Type::LiteralConstString => Cow::Borrowed("[]const u8"),
        Type::ZigArray => Cow::Borrowed("[]usize"),
        Type::ZigConstArray => Cow::Borrowed("[]const usize"),
        Type::Bool => Cow::Borrowed("bool"),
        Type::Array(inner) => {
            let inner_str = map_type(inner, is_const);
            inner_str
        }
        Type::User(_) => Cow::Borrowed("any"),
        Type::Allocator => Cow::Borrowed("std.mem.Allocator"),
    }
}

pub fn is_semicolon_needed(expr: &TypedExpr) -> bool {
    matches!(
        expr,
        TypedExpr::Assignment { .. }
            | TypedExpr::Break
            | TypedExpr::Continue
            | TypedExpr::Return(_)
            | TypedExpr::Decl { .. }
            | TypedExpr::Call { .. }
            | TypedExpr::BuiltInCall { .. }
            | TypedExpr::VariableRef { .. }
            | TypedExpr::Index { .. }
            | TypedExpr::BinaryOp { .. }
    )
}

pub fn transpile_function_def<'a>(
    name: &'a str,
    params: &'_ [ParameterTyped<'a>],
    body: &'a [TypedExpr<'a>],
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

fn transpile_param(param: &ParameterTyped) -> String {
    let zig_type = map_type(&param.typ, !param.is_mutable);
    if param.is_mutable {
        format!("{}: *{}", param.name, zig_type)
    } else {
        format!("{}: {}", param.name, zig_type)
    }
}

pub fn is_muttable<'a>(expr: &'a TypedExpr<'a>) -> bool {
    match expr {
        TypedExpr::VariableRef {
            name: _,
            transpiled_name: _,
            symbol,
        } => {
            if let Some(sym) = symbol {
                return sym.is_mutable;
            }
        }
        TypedExpr::Call { target, .. } => match target {
            Some(boxed_expr) => match &**boxed_expr {
                TypedExpr::VariableRef {
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
