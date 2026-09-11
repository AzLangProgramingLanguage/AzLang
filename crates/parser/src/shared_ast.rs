use crate::ast::Atom;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String(StringEnum),
    Array(Box<Type>),
    User(Atom),
    Enum(String),
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
            Type::Array(_) => write!(f, "List"),
            Type::User(name) => write!(f, "User({name})"),
            Type::Enum(str) => write!(f, "{str}"),
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
            Type::Function => write!(f, "Funksiya"),
        }
    }
}
