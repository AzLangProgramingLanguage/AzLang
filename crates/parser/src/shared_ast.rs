use std::{borrow::Cow, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInFunction {
    Print,
    Input,
    Len,
    Number,
    Sum,
    Range,
    Trim,
    LastWord,
    Sqrt,
    Timer,
    Max,
    Mod,
    Min,
    Round,
    Floor,
    Ceil,
    Zig,
    StrUpper,
    StrReverse,
    StrLower,
    Allocator,
    ConvertString,
}

impl BuiltInFunction {
    pub fn expected_arg_count(&self) -> Option<usize> {
        match self {
            BuiltInFunction::Print
            | BuiltInFunction::Len
            | BuiltInFunction::Sqrt
            | BuiltInFunction::Round
            | BuiltInFunction::Floor
            | BuiltInFunction::Ceil
            | BuiltInFunction::Mod
            | BuiltInFunction::Zig
            | BuiltInFunction::Number
            | BuiltInFunction::Allocator
            | BuiltInFunction::Trim
            | BuiltInFunction::LastWord => Some(1),

            BuiltInFunction::Range => Some(2),

            BuiltInFunction::Timer => Some(0),
            BuiltInFunction::Sum => None,
            BuiltInFunction::Input => None,
            BuiltInFunction::StrUpper => Some(1),
            BuiltInFunction::StrReverse => Some(1),
            BuiltInFunction::StrLower => Some(1),
            BuiltInFunction::ConvertString => Some(1),
            BuiltInFunction::Max | BuiltInFunction::Min => None,
        }
    }
}
impl Display for BuiltInFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltInFunction::Print => write!(f, "Print"),
            BuiltInFunction::Input => write!(f, "Input"),
            BuiltInFunction::Len => write!(f, "Len"),
            BuiltInFunction::Number => write!(f, "Number"),
            BuiltInFunction::Sum => write!(f, "Sum"),
            BuiltInFunction::Range => write!(f, "Range"),
            BuiltInFunction::Trim => write!(f, "Trim"),
            BuiltInFunction::LastWord => write!(f, "LastWord"),
            BuiltInFunction::Sqrt => write!(f, "Sqrt"),
            BuiltInFunction::Timer => write!(f, "Timer"),
            BuiltInFunction::Max => write!(f, "Max"),
            BuiltInFunction::Mod => write!(f, "Mod"),
            BuiltInFunction::Min => write!(f, "Min"),
            BuiltInFunction::Round => write!(f, "Round"),
            BuiltInFunction::Floor => write!(f, "Floor"),
            BuiltInFunction::Ceil => write!(f, "Ceil"),
            BuiltInFunction::Zig => write!(f, "Zig"),
            BuiltInFunction::StrUpper => write!(f, "StrUpper"),
            BuiltInFunction::StrReverse => write!(f, "StrReverse"),
            BuiltInFunction::StrLower => write!(f, "StrLower"),
            BuiltInFunction::ConvertString => write!(f, "ConvertString"),
            BuiltInFunction::Allocator => write!(f, "Allocator"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type<'a> {
    String,
    Array(Box<Type<'a>>),
    User(Cow<'a, str>),
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
    LiteralString,
    LiteralConstString,
    ZigArray,
    ZigConstArray,
    ZigNatural,
    ZigFloat,
    ZigInteger,
}

impl Display for Type<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::String => write!(f, "Yazı"),
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
            Type::LiteralString => write!(f, "LiteralString"),
            Type::LiteralConstString => write!(f, "LiteralConstString"),
            Type::ZigArray => write!(f, "ZigArray"),
            Type::ZigConstArray => write!(f, "ZigConstArray"),
            Type::ZigNatural => write!(f, "ZigNatural"),
            Type::ZigFloat => write!(f, "ZigFloat"),
            Type::ZigInteger => write!(f, "ZigInteger"),
        }
    }
}
