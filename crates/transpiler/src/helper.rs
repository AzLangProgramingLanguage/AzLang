use parser::{ast::Expr, shared_ast::Type};

pub fn get_expr_type<'a>(expr: &Expr<'a>) -> Type<'a> {
    match expr {
        Expr::String(_) => Type::String,
        Expr::Number(_) => Type::Integer,
        Expr::Float(_) => Type::Float,
        Expr::Bool(_) => Type::Bool,
        Expr::Char(_) => Type::Char,
        Expr::UnaryOp { op, expr: _ } => match *op {
            "!" => Type::Bool,
            "-" => Type::Integer,
            "+" => Type::Natural,
            _ => {
                panic!("get_expr_type de bilinmeyen unaryOp tipi geldi ")
            }
        },

        Expr::Call {
            target,
            name,
            args,
            returned_type,
        } => returned_type.as_ref().unwrap().clone(),
        Expr::VariableRef { name: _, symbol } => symbol.as_ref().unwrap().typ.clone(),
        Expr::BuiltInCall {
            function: _,
            args: _,
            return_type,
        } => return_type.clone(),

        Expr::BinaryOp {
            left, op, right, ..
        } => {
            /*TODO:: Bu nə pis koddu belə */
            let left_type = get_expr_type(left);
            let right_type = get_expr_type(right);

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
        Expr::Index {
            target: _,
            index: _,
            target_type,
        } => target_type.clone(),
        Expr::List(items) => {
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

pub fn map_type<'a>(typ: &'a Type<'a>, is_const: bool) -> &'static str {
    match typ {
        Type::Integer => "azlangEded",
        Type::Natural => "azlangEded",
        Type::Any => "any",
        Type::Void => "void",
        Type::ZigFloat => "f64",
        Type::Float => "azlangEded",
        Type::BigInteger => {
            if is_const {
                "const i128"
            } else {
                "i128"
            }
        }
        Type::Char => "u8",
        Type::LowInteger => {
            if is_const {
                "const u8"
            } else {
                "u8"
            }
        }
        Type::String => {
            if is_const {
                "azlangYazi"
            } else {
                "azlangYazi"
            }
        }
        Type::LiteralString => "[]const u8", /* TODO: burada Literal Stringin tipi ferqli olması lazımdır. */
        Type::ZigNatural => "usize",
        Type::ZigInteger => "isize",
        Type::LiteralConstString => "[]const u8",
        Type::ZigArray => "[]usize",
        Type::ZigConstArray => "[]const usize",
        Type::Bool => "bool",
        Type::Array(inner) => {
            let inner_str = map_type(inner, is_const);
            inner_str
        }
        Type::User(_) => "any",
        Type::Allocator => "std.mem.Allocator",
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

pub fn is_primite_value(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Number(_)
            | Expr::Float(_)
            | Expr::Bool(_)
            | Expr::Char(_)
            | Expr::String(_)
            | Expr::UnaryOp { .. }
    )
}
