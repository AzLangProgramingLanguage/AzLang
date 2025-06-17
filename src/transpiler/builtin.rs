use crate::parser::ast::{BuiltInFunction, Type};

pub fn match_builtin(name: &str) -> Option<BuiltInFunction> {
    match name {
        "çap" | "print" => Some(BuiltInFunction::Print),
        "giriş" | "input" => Some(BuiltInFunction::Input),
        "uzunluq" | "len" => Some(BuiltInFunction::Len),
        "ədəd" | "number" => Some(BuiltInFunction::Number),
        "cəm" | "sum" => Some(BuiltInFunction::Sum),
        "aralıq" | "range" => Some(BuiltInFunction::Range),
        "sonsöz" | "last_word" => Some(BuiltInFunction::LastWord),
        _ => None,
    }
}

pub fn get_format_str_from_type(typ: &Type) -> &'static str {
    match typ {
        Type::Metn => "{s}",
        Type::Integer | Type::BigInteger | Type::LowInteger => "{}",
        Type::Bool => "{}",
        Type::Char => "{c}",
        Type::Any => "{any}",
        Type::Siyahi(_) => "{any} ", // Siyahıları yazdırmaq istəmirik, amma fallback
        Type::Istifadeci(_) => "{any}", // Custom tip varsa default yazdırma
    }
}
