use crate::parser::ast::Type;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidatorError<'a> {
    #[error("Tip təyin edilə bilmədi və göstərilməyib: '{0}'")]
    TypeInferenceFailed(&'a str),

    #[error("Tip uyğunsuzluğu: gözlənilən '{expected}', tapılan '{found}'")]
    TypeMismatch { expected: String, found: String },

    #[error("Tip uyğunsuzluğu: '{name}' üçün gözlənilən {expected:?}, tapılan {found:?}")]
    DeclTypeMismatch {
        name: String,
        expected: String,
        found: String,
    },
    #[error("Gözlənilməz ifadə")]
    UnknownExpression,

    #[error(" '{0}' Dəyəri onsuzda var. ")]
    AlreadyDecl(String),

    #[error("Dəyər enum variantı deyil")]
    NotAnEnumVariant,

    #[error("Enum '{0}' artıq mövcuddur")]
    DuplicateEnum(String),

    #[error("cəm funksiyası yalnız ədəd tipli siyahı qəbul edir")]
    InvalidSumArgumentType,

    #[error("'{name}' funksiyası yalnız bir arqument qəbul edir")]
    InvalidOneArgumentCount { name: String },

    #[error("'{name}' funksiyası yalnız iki arqument qəbul edir")]
    InvalidTwoArgumentCount { name: String },

    #[error("'{name}' funksiyası arqument qəbul etmir")]
    InvalidZeroArgumentCount { name: String },

    #[error("'{name}' funksiyası yalnız {expected} arqument qəbul edir")]
    InvalidArgumentCount {
        name: String,
        expected: usize,
        found: usize,
    },
    #[error("Union '{0}' artıq mövcuddur")]
    DuplicateUnion(String),
    #[error("Dəyişən '{0}' elan edilməyib")]
    UndefinedVariable(String),

    #[error("Şərt tipi `bool` olmalıdır, tapıldı: {0:?}")]
    IfConditionTypeMismatch(Type<'a>),

    #[error("Şərt tipi müəyyən edilə bilmədi")]
    IfConditionTypeUnknown,

    #[error("`else if` şərti `bool` olmalıdır, tapıldı: {0:?}")]
    ElseIfConditionTypeMismatch(Type<'a>),

    #[error("Dövr üçün istifadə edilən obyektin tipi təyin edilə bilmədi")]
    LoopIterableTypeNotFound,

    #[error("Dövr üçün istifadə edilən obyekt siyahı (`list`) olmalıdır")]
    LoopRequiresList,

    #[error("Funksiya tapılmadı: '{0}'")]
    FunctionNotFound(&'a str),

    #[error("Funksiya '{name}' üçün {expected} arqument gözlənilirdi, lakin {found} verildi")]
    FunctionArgCountMismatch {
        name: String,
        expected: usize,
        found: usize,
    },

    #[error("İç funksiya təyin etmək qadağandır")]
    NestedFunctionDefinition,

    #[error("Funksiya '{name}' üçün tip təyin edilməyib")]
    FunctionReturnTypeNotFound { name: String },

    #[error("Struct '{0}' artıq mövcuddur")]
    DuplicateStruct(&'a str),

    #[error("Indeksli ifadə üçün tip təyin edilməyib")]
    IndexTargetTypeNotFound,

    #[error("Dəyişənin tipi müəyyən edilməyib")]
    DeclTypeUnknown,

    #[error("Tip '{0}' tapılmadı")]
    UnknownType(String),

    #[error("Import funksiyası yalnız string arqument qəbul edir")]
    ImportError,
    #[error("Struct '{0}' tapılmadı")]
    UnknownStruct(String),
}
