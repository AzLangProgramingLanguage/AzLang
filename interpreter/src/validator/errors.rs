use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum ValidatorError {
    UnknownType(String),
    AlreadyDecl(String),
    DeclTypeMismatch {
        name: String,
        expected: String,
        found: String,
    },
    DeclTypeUnknown(String),
    AssignmentToImmutableVariable(String),
    AssignmentTypeMismatch {
        name: String,
        expected: String,
        found: String,
    },
    UndefinedVariable(String),
    DuplicateUnion(String),
    InvalidArgumentCount {
        name: String,
        expected: usize,
        found: usize,
    },
    TypeMismatch {
        expected: String,
        found: String,
    },
    UnknownStruct(String),
    DuplicateStruct(String),
    DuplicateEnum(String),
    IfConditionTypeUnknown,
    IfConditionTypeMismatch(String),
    LoopIterableTypeNotFound,
    LoopRequiresList,
    UnionNotFound(String),
    FunctionNotFound(String),
    FunctionArgCountMismatch {
        name: String,
        expected: usize,
        found: usize,
    },
    IndexTargetTypeNotFound,
    NestedFunctionDefinition,
}

impl Display for ValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidatorError::UnknownType(string) => write!(f, "Gözlənilməyən token '{string}'"),
            ValidatorError::AlreadyDecl(string) => write!(f, "'{string}' Dəyəri onsuzda var.  "),
            ValidatorError::DeclTypeMismatch {
                name,
                expected,
                found,
            } => write!(
                f,
                "'{name}' Dəyəri '{expected}' tipində olmalıdır, ancak '{found}' tipində var."
            ),
            ValidatorError::DeclTypeUnknown(string) => {
                write!(f, "'{string}' Dəyəri tipi bildirilməyib.")
            }
            ValidatorError::AssignmentToImmutableVariable(string) => {
                write!(f, "'{string}' Dəyəri dəyişən olmalıdır.")
            }
            ValidatorError::AssignmentTypeMismatch {
                name,
                expected,
                found,
            } => write!(
                f,
                "'{name}' Dəyəri '{expected}' tipində olmalıdır, ancak '{found}' tipində yazılıb."
            ),
            ValidatorError::UndefinedVariable(string) => {
                write!(f, "'{string}' Dəyəri bildirilməyib.")
            }
            ValidatorError::DuplicateUnion(string) => {
                write!(f, "'{string}' Union tərifi onsuzda var.")
            }
            ValidatorError::InvalidArgumentCount {
                name,
                expected,
                found,
            } => {
                write!(
                    f,
                    "'{name}' funksiyası '{expected}' argumenti olmalıdır, ancak '{found}' argumenti var."
                )
            }
            ValidatorError::TypeMismatch { expected, found } => {
                write!(
                    f,
                    "'{expected}' tipində olmalıdır, ancak '{found}' tipində var."
                )
            }
            ValidatorError::UnknownStruct(string) => {
                write!(f, "'{string}' Struct tərifi bildirilməyib.")
            }
            ValidatorError::DuplicateStruct(string) => {
                write!(f, "'{string}' Struct tərifi onsuzda var.")
            }
            ValidatorError::DuplicateEnum(string) => {
                write!(f, "'{string}' Enum tərifi onsuzda var.")
            }
            ValidatorError::IfConditionTypeUnknown => {
                write!(f, "Şərt tipi müəyyən edilə bilmədi.")
            }
            ValidatorError::IfConditionTypeMismatch(typ) => {
                write!(f, "Şərt '{typ}' tipində olmalıdır.")
            }
            ValidatorError::LoopIterableTypeNotFound => {
                write!(f, "Dövr iterable tipi müəyyən edilə bilmədi.")
            }
            ValidatorError::LoopRequiresList => {
                write!(f, "Dövr iterable tipi müəyyən edilə bilmədi.")
            }
            ValidatorError::UnionNotFound(string) => {
                write!(f, "'{string}' Union tərifi bildirilməyib.")
            }
            ValidatorError::FunctionNotFound(string) => {
                write!(f, "'{string}' funksiyası bildirilməyib.")
            }
            ValidatorError::FunctionArgCountMismatch {
                name,
                expected,
                found,
            } => {
                write!(
                    f,
                    "'{name}' funksiyası '{expected}' argumenti olmalıdır, ancak '{found}' argumenti var."
                )
            }
            ValidatorError::IndexTargetTypeNotFound => {
                write!(f, "Indeks tipi müəyyən edilə bilmədi.")
            }
            ValidatorError::NestedFunctionDefinition => {
                write!(f, "Funksiya tərifi onsuzda var.")
            }
        }
    }
}
