use crate::parser::ast::{Expr, Type};

pub fn get_expr_type<'a>(expr: &Expr<'a>) -> Type<'a> {
    match expr {
        Expr::String(_) => Type::Metn,
        Expr::Number(_) => Type::Integer,
        Expr::Float(_) => Type::Float,
        Expr::Bool(_) => Type::Bool,
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
