use std::borrow::Cow;

use crate::{
    parser::ast::{Expr, Type},
    validator::ValidatorContext,
};

pub fn get_type<'a>(
    value: &Expr<'a>,
    ctx: &ValidatorContext<'a>,
    typ: Option<&Type<'a>>,
) -> Option<Type<'a>> {
    match value {
        Expr::Number(_) => Some(Type::Natural),
        Expr::UnaryOp { op, expr } => {
            get_type(expr, ctx, typ)?;
            match &**op {
                "-" => Some(Type::Integer),
                "!" => Some(Type::Bool),
                _ => None,
            }
        }
        Expr::Bool(_) => Some(Type::Bool),
        Expr::String(_) => Some(Type::Metn),
        Expr::List(items) => {
            if (items.len() > 0) {
                let item_type = get_type(&items[0], ctx, typ)?;
                for item in &items[1..] {
                    let t = get_type(item, ctx, typ)?;
                    if t != item_type {
                        return Some(Type::Siyahi(Box::new(Type::Any))); // qarışıq tiplər
                    }
                }

                Some(Type::Siyahi(Box::new(item_type)))
            } else {
                Some(Type::Any)
            }
        }
        Expr::Index {
            target: _,
            index: _,
            target_type,
        } => Some(target_type.clone()),
        Expr::VariableRef {
            name,
            transpiled_name: _,
            symbol,
        } => {
            if let Some(s) = ctx.lookup_variable(name) {
                return Some(s.typ.clone());
            }

            if let Some(t) = typ {
                if let Type::Istifadeci(enum_name) = t {
                    if let Some(variants) = ctx.enum_defs.get(enum_name) {
                        if variants.contains(name) {
                            return Some(t.clone());
                        }
                    }
                }
            }
            return Some(symbol.as_ref().unwrap().typ.clone());
        }
        Expr::StructInit { name, .. } => {
            if let Some(_) = ctx.struct_defs.get(name.as_ref()) {
                Some(Type::Istifadeci(Cow::Owned(name.to_string())))
            } else {
                None
            }
        }
        Expr::BuiltInCall { return_type, .. } => Some(return_type.clone()),
        Expr::Call { returned_type, .. } => {
            dbg!(returned_type);

            returned_type.clone()
        }
        Expr::BinaryOp { left, op, right } => {
            let left_type = get_type(left, ctx, typ)?;
            let right_type = get_type(right, ctx, typ)?;

            // Əgər operandların tipi uyğun gəlmir
            if left_type != right_type {
                return None;
            }

            let comparison_ops = ["==", "!=", "<", "<=", ">", ">="];
            let logic_ops = ["&&", "||"];
            let arithmetic_ops = ["+", "-", "*", "/", "%"];

            if comparison_ops.contains(&op) || logic_ops.contains(&op) {
                return Some(Type::Bool);
            }

            if arithmetic_ops.contains(&op) {
                // Sadəcə ədədlər üçün arifmetik operatorlar keçərlidir
                if left_type == Type::Integer {
                    return Some(Type::Integer);
                } else {
                    return None;
                }
            }

            // Tanınmayan operatorlar üçün
            None
        }
        _ => None,
    }
}

// AST-də və ya parse modulunun hər hansı yerinə əlavə edin
pub fn transpile_az_chars<'a>(name: &'a str) -> Cow<'a, str> {
    // ASCII olmayan hərflərin hamısı 2 bayt olduğu üçün
    // 16 simvoldan uzun string-lər üçün də 1-2 dəfə alloc olacaq.
    if name.bytes().all(|b| b.is_ascii()) {
        return Cow::Borrowed(name); // kopyalamırıq
    }

    // 1:1 translit cədvəli
    let azlang = "azlang";
    let mut out = String::with_capacity(name.len() + azlang.len());
    out.push_str(azlang);

    for ch in name.chars() {
        match ch {
            'ə' => out.push('e'),
            'ö' => out.push('o'),
            'ü' => out.push('u'),
            'ğ' => out.push('g'),
            'ç' => out.push('c'),
            'ş' => out.push('s'),
            'ı' => out.push('i'),
            _ => out.push(ch),
        }
    }
    Cow::Owned(out)
}
