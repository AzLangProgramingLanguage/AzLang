use crate::parser::ast::{BuiltInFunction, Type};

pub fn match_builtin(name: &str) -> Option<(BuiltInFunction, Type)> {
    match name {
        "çap" => Some((BuiltInFunction::Print, Type::Metn)),
        "giriş" => Some((BuiltInFunction::Input, Type::Metn)),
        "Ədəd" => Some((BuiltInFunction::Number, Type::Integer)),
        "cəm" => Some((BuiltInFunction::Sum, Type::Integer)),
        "uzunluq" => Some((BuiltInFunction::Len, Type::Integer)),
        "sonsöz" => Some((BuiltInFunction::LastWord, Type::Metn)),
        "aralıq" => Some((
            BuiltInFunction::Range,
            Type::Siyahi(Box::new(Type::Integer)),
        )),
        _ => None,
    }
}
