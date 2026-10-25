use core::fmt;
use std::fmt::Display;

use parser::{ast::Operation, shared_ast::Type};

#[derive(Debug, PartialEq)]
pub enum ValidatorError {
    UnknownType(String),
    InlineAsmMustBeString(Type),
    InvalidFunctionCall(String),
    AlreadyDecl(String),
    InvalidOperation {
        op: Operation,
        left: Type,
        right: Type,
    },
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
    InvalidArgumentType {
        name: String,
        expected: String,
        found: String,
    },
    TypeMismatch {
        expected: Type,
        found: Type,
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
    NeverChangedMuttableVariable(String),
    FunctionAlreadyDefined(String),
    FunctionReturnTypeErr(String),
    FunctionNameType(String),
    NotUsedVariable(String),
}

impl Display for ValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidatorError::UnknownType(string) => write!(f, "Unexpected token '{string}'"),
            ValidatorError::InlineAsmMustBeString(typ) => {
                write!(f, "Inline assembly must be string but found {}", typ)
            }
            ValidatorError::InvalidFunctionCall(found) => write!(f, "{found} is not a function."),
            ValidatorError::InvalidOperation { op, left, right } => {
                write!(
                    f,
                    "'{}' operation cannot be performed on '{left}' and '{right}' types.",
                    op.as_str()
                )
            }
            ValidatorError::FunctionNameType(name) => write!(f, "'{name}' type is not a function.  "),
            ValidatorError::AlreadyDecl(string) => write!(f, "Value '{string}' already exists.  "),
            ValidatorError::DeclTypeMismatch {
                name,
                expected,
                found,
            } => write!(
                f,
                "Value '{name}' must be of type '{expected}', but it is of type '{found}'."
            ),
            ValidatorError::InvalidArgumentType {
                name,
                expected,
                found,
            } => write!(
                f,
                "Argument of function '{name}' must be of type '{expected}', but it is of type '{found}'."
            ),
            ValidatorError::DeclTypeUnknown(string) => {
                write!(f, "Type of value '{string}' is not declared.")
            }
            ValidatorError::AssignmentToImmutableVariable(string) => {
                write!(f, "Value '{string}' must be mutable.")
            }
            ValidatorError::NeverChangedMuttableVariable(string) => {
                write!(
                    f,
                    "Value '{string}' is declared as mutable but is never modified."
                )
            }

            ValidatorError::AssignmentTypeMismatch {
                name,
                expected,
                found,
            } => write!(
                f,
                "Value '{name}' must be of type '{expected}', but '{found}' was assigned."
            ),
            ValidatorError::UndefinedVariable(string) => {
                write!(f, "Value '{string}' is not declared.")
            }
            ValidatorError::DuplicateUnion(string) => {
                write!(f, "Union '{string}' is already defined.")
            }
            ValidatorError::InvalidArgumentCount {
                name,
                expected,
                found,
            } => {
                write!(
                    f,
                    "Function '{name}' must have '{expected}' argument(s), but '{found}' were provided."
                )
            }
            ValidatorError::TypeMismatch { expected, found } => {
                write!(
                    f,
                    "Must be of type '{expected}', but it is of type '{found}'."
                )
            }
            ValidatorError::UnknownStruct(string) => {
                write!(f, "Struct '{string}' is not declared.")
            }
            ValidatorError::DuplicateStruct(string) => {
                write!(f, "Struct '{string}' is already defined.")
            }
            ValidatorError::DuplicateEnum(string) => {
                write!(f, "Enum '{string}' is already defined.")
            }
            ValidatorError::IfConditionTypeUnknown => {
                write!(f, "Condition type could not be determined.")
            }
            ValidatorError::IfConditionTypeMismatch(typ) => {
                write!(f, "Condition must be of type '{typ}'.")
            }
            ValidatorError::LoopIterableTypeNotFound => {
                write!(f, "Loop iterable type could not be determined.")
            }
            ValidatorError::LoopRequiresList => {
                write!(f, "Loop requires an iterable list.")
            }
            ValidatorError::UnionNotFound(string) => {
                write!(f, "Union '{string}' is not declared.")
            }
            ValidatorError::FunctionNotFound(string) => {
                write!(f, "Function '{string}' is not declared.")
            }
            ValidatorError::FunctionArgCountMismatch {
                name,
                expected,
                found,
            } => {
                write!(
                    f,
                    "Function '{name}' must have '{expected}' argument(s), but '{found}' were provided."
                )
            }
            ValidatorError::IndexTargetTypeNotFound => {
                write!(f, "Index type could not be determined.")
            }
            ValidatorError::NestedFunctionDefinition => {
                write!(f, "Functions cannot be defined inside functions.")
            }
            ValidatorError::FunctionAlreadyDefined(string) => {
                write!(f, "Function '{string}' already exists.")
            }
            ValidatorError::FunctionReturnTypeErr(string) => {
                write!(
                    f,
                    "Function returns an invalid value, it must be of type '{string}'  "
                )
            }
            ValidatorError::NotUsedVariable(var) => {
                write!(
                    f,
                    "Value '{var}' is never used. There is no need to declare unused values. "
                )
            }
        }
    }
}
