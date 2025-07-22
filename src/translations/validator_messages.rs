use crate::{
    lexer::Token,
    parser::ast::{Expr, Type},
};
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
}

/* use thiserror::Error;

use crate::{
    lexer::Token,
    parser::ast::{Expr, Type},
};

#[derive(Debug, Error)]
pub enum ValidatorError<'a> {
    #[error("Obyekt '{0}' artıq mövcuddur")]
    DuplicateStruct(String),

    #[error("Enum '{0}' artıq mövcuddur")]
    DuplicateEnum(String),

    #[error("Enum '{enum_name}' üçün variant '{variant}' tapılmadı")]
    UnknownEnumVariant { enum_name: String, variant: String },

    #[error("Enum '{0}' tapılmadı")]
    EnumNotFound(String),

    #[error("Uyğunlaşdırmada tip düzgün müəyyən edilmədi")]
    MatchUnkdownType(),



    #[error("Dəyişən '{0}' elan edilməyib")]
    UndefinedVariable(&'a str),

    #[error("Qeyri-qanuni pattern: {0}")]
    InvalidPattern(String),

    #[error("Tip təyin edilə bilmədi və göstərilməyib: '{0}'")]
    TypeInferenceFailed(String),

    #[error("Tip təyin edilə bilmədi: '{0}'")]
    CouldNotInferType(String),

    #[error("Dəyər enum variantı deyil")]
    NotAnEnumVariant,



    #[error("print funksiyası yalnız bir arqument qəbul edir")]
    InvalidPrintArgumentCount,

    #[error("range funksiyası yalnız 2 arqument qəbul edir")]
    InvalidRangeArgumentCount,

    #[error("Vaxt_al funksiyası arqument qəbul etmir")]
    InvalidTimerArgumentCount,

    #[error("Method çağırış üçün tip təyin edilə bilmədi")]
    MethodCallTypeResolutionFailed,

    #[error("Metod '{method}' yalnız {expected} arqument qəbul edir")]
    MethodArgCountMismatch {
        method: String,
        expected: usize,
        given: usize,
    },

    #[error("Metod '{method}' arqumentsiz olmalıdır")]
    MethodMustBeArgless { method: String, given: usize },

    #[error("Dəstəklənməyən metod: '{0}'")]
    UnsupportedMethod(String),

    #[error("Struktur tapılmadı: '{0}'")]
    StructNotFound(String),

    #[error("Struktur '{struct_name}' belə bir metoda sahib deyil: '{method}'")]
    MethodNotFound { struct_name: String, method: String },

    #[error("Tip metodları dəstəkləmir: {0:?}")]
    TypeDoesNotSupportMethods(Type<'a>),

    #[error("Dövr üçün istifadə edilən obyektin tipi təyin edilə bilmədi")]
    LoopIterableTypeNotFound,

    #[error("Dövr üçün istifadə edilən obyekt siyahı (`list`) olmalıdır")]
    LoopRequiresList,

    #[error("Return ifadəsi yalnız funksiya daxilində istifadə oluna bilər")]
    ReturnOutsideFunction,

    #[error("Funksiya təkrarlanır: {0}")]
    DuplicateFunction(&'a str),

    #[error("Siyahının ilk elementi üçün tip təyin edilə bilmədi")]
    ListFirstTypeNotFound,

    #[error("Siyahı elementi üçün tip təyin edilə bilmədi")]
    ListItemTypeNotFound,

    #[error("Siyahı daxilində tip uyğunsuzluğu: gözlənilən {expected:?}, tapılan {found:?}")]
    ListTypeMismatch { expected: Type<'a>, found: Type<'a> },

    #[error("Funksiya tapılmadı: '{0}'")]
    FunctionNotFound(String),

    #[error("Funksiya '{name}' üçün {expected} arqument gözlənilirdi, amma {found} verildi")]
    FunctionArgCountMismatch {
        name: String,
        expected: usize,
        found: usize,
    },

    #[error(
        "Funksiya '{0}' içərisində başqa funksiya təyin etmək qadağandır. Onu xaricdə təyin edin."
    )]
    NestedFunctionDefinition(String),

    #[error("Match üçün '{}' enum tərifi tapılmadı", _0)]
    EnumDefinitionNotFound(String),

    #[error("Enum match üçün uyğun olmayan pattern: {0:?}")]
    InvalidEnumMatchPattern(Token),

    #[error("String literal match üçün yalnız 1 simvol gözlənilirdi, tapıldı: {0}")]
    StringLiteralLengthMismatch(String),

    #[error("Enum olmayan match üçün qeyri-qanuni identifier: '{0}'")]
    InvalidIdentifierInMatch(String),

    #[error("Match üçün tanınmayan token: {0:?}")]
    UnknownTokenInMatch(Token),

    #[error("Sabit '{0}' dəyişdirilə bilməz")]
    ImmutableAssignment(String),

    #[error(
        "'{struct_name}' strukturu üçün {expected} arqument gözlənilirdi, lakin {found} verildi"
    )]
    StructArgumentCountMismatch {
        struct_name: String,
        expected: usize,
        found: usize,
    },

    #[error("'{field}' sahəsi üçün tip uyğunsuzluğu: gözlənilən {expected:?}, tapılan {found:?}")]
    StructFieldTypeMismatch {
        field: String,
        expected: Type<'a>,
        found: Type<'a>,
    },

    #[error("FieldAccess üçün tip təyin edilə bilmədi")]
    FieldAccessUnknownType,

    #[error("Sahəyə yalnız struktur növü üzərindən çıxış edilə bilər")]
    FieldAccessRequiresStruct,

    #[error("Struktur '{struct_name}' sahəyə sahib deyil: '{field}'")]
    FieldNotFound { struct_name: String, field: String },

    #[error("İf şərtinin tipi müəyyən edilə bilmədi")]
    IfConditionTypeUnknown,

    #[error("İf şərti `bool` olmalıdır, tapıldı: {0:?}")]
    IfConditionTypeMismatch(Type<'a>),

    #[error("Else if şərtinin tipi müəyyən edilə bilmədi")]
    ElseIfConditionTypeUnknown,

    #[error("`else if` şərti `bool` olmalıdır, tapıldı: {0:?}")]
    ElseIfConditionTypeMismatch(Type<'a>),

    #[error("Binary `{name}` operatorunda tip uyğunsuzluğu: {typ:?} və {other:?}")]
    BinaryOpTypeMismatch {
        name: String,
        typ: Type<'a>,
        other: Type<'a>,
    },

    #[error("Gözlənilməz ifadə: {0:?}")]
    UnknownExpression(&'a Expr<'a>),

    #[error("{0}")]
    Custom(String),
}
 */
