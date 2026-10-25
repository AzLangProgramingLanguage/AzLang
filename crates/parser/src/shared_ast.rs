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
            StringEnum::DynamicString => write!(f, "Dynamic String"),
            StringEnum::LiteralString => write!(f, "String"),
            StringEnum::LiteralConstString => write!(f, "Const string"),
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
            Type::Integer => write!(f, "Integer"),
            Type::Natural => write!(f, "Natural"),
            Type::BigInteger => write!(f, "Big integer"),
            Type::LowInteger => write!(f, "Small integer"),
            Type::Bool => write!(f, "Bool"),
            Type::Char => write!(f, "Char"),
            Type::Allocator => write!(f, "Allocator"),
            Type::Void => write!(f, "Void"),
            Type::Any => write!(f, "Any"),
            Type::Float => write!(f, "Float"),
            Type::Function => write!(f, "Funksiya"),
        }
    }
}
