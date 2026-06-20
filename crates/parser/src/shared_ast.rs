use std::fmt::Display;
use crate::ast::Atom;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String(StringEnum),
    Array(Box<Type>),
    User(Atom),
    Integer,
    Natural,
    BigInteger,
    LowInteger,
    Bool,
    Char,
    Allocator,
    Void,
    Any,
    Float,
    ZigArray,
    ZigConstArray,
    ZigNatural,
    ZigFloat,
    ZigInteger,
    Function,
}
#[derive(Debug, Clone, PartialEq)]
pub enum StringEnum {
    DynamicString,
    LiteralString,
    LiteralConstString,
}
impl Display for StringEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringEnum::DynamicString => write!(f, "Dinamik Yazı"),
            StringEnum::LiteralString => write!(f, "Yazı"),
            StringEnum::LiteralConstString => write!(f, "Sabit yazı"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::String(typ) => write!(f, "{}", typ),
            Type::Array(_) => write!(f, "Siyahı"),
            Type::User(name) => write!(f, "İstifadəçi({name})"),
            Type::Integer => write!(f, "Tam Ədəd"),
            Type::Natural => write!(f, "Natural"),
            Type::BigInteger => write!(f, "Böyük tam ədəd"),
            Type::LowInteger => write!(f, "Kiçik tam ədəd"),
            Type::Bool => write!(f, "Şərt"),
            Type::Char => write!(f, "Simvol"),
            Type::Allocator => write!(f, "Allocator"),
            Type::Void => write!(f, "Boşluq"),
            Type::Any => write!(f, "Hərşey"),
            Type::Float => write!(f, "Onluq Ədəd"),
            Type::ZigArray => write!(f, "ZigArray"),
            Type::ZigConstArray => write!(f, "ZigConstArray"),
            Type::ZigNatural => write!(f, "ZigNatural"),
            Type::ZigFloat => write!(f, "ZigFloat"),
            Type::ZigInteger => write!(f, "ZigInteger"),
            Type::Function => write!(f, "Funksiya"),
        }
    }
}
