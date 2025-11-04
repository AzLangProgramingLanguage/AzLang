use crate::lexer::Token;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    /* Variable Decl */
    #[error("Dəyişən adı gözlənilirdi,tapıldı '{0}'")]
    VariableName(String),

    #[error("'{0}' operatoru gözlənilirdi, tapıldı: '{1}'")]
    OperatorError(char, String),

    /* Type Errors */
    #[error("Siyahı üçün '{0}' gözlənilirdi, tapıldı: '{1}'")]
    ArrayError(char, String),

    #[error("Siyahı düzgün bağlanılmadı: ']' gözlənilirdi: tapıldı '{0}'")]
    ArrayEndError(String),

    #[error("Arqument siyahısında ',' və ya ')' gözlənilirdi: tapıldı '{0}'")]
    ArgsError(String),

    #[error("Tip gözlənilirdi amma tip tapılmadı")]
    TypeNotFound,

    #[error("Gözlənilməz Eof")]
    Eof,

    #[error("Sol tərəfdə dəyişən gözlənilirdi")]
    VariableNotFound,

    /* Structs Errors */
    #[error("Struct init argümentləri arasında ',' və ya '}}' gözlənilirdi")]
    StructInitError,

    #[error("Struct init argümentləri arasında ':' gözlənilirdi")]
    StructInitColonError,

    #[error("Struct adı gözlənilirdi, tapıldı '{0}'")]
    StructNameNotFound(String),

    /* Literal Errors */
    #[error("Literal gözlənilirdi, tapıldı '{0}'")]
    LiteralNotFound(String),

    /* Object Errors */
    #[error("Obyekt tipi gözlənilirdi")]
    ObjectTypeNotFound,

    #[error("Struct daxilində gözlənilməz token: {0}")]
    ObjectUnknownToken(String),

    /* Template Errors */
    #[error("Template string içində tanınmayan token: '{0}'")]
    TemplateTokenNotFound(String),

    /* Metod Errors */
    #[error("Metod adı gözlənilirdi, tapıldı '{0}'")]
    MethodName(String),

    #[error("Gözlənilirdi {0:?}, tapıldı {1:?}")]
    OtherError(Token, String),

    /* Union Types Errors */
    #[error("tip`-dən sonra identifikator gözlənilirdi, tapıldı: {0}")]
    UnionIdentifierNotFound(String),

    #[error("Birləşik tip daxilində gözlənilməz token: {0}")]
    UnionUnknownToken(String),

    #[error("Birləşik tip adı gözlənilirdi, tapıldı: {0}")]
    UnionNameNotFound(String),

    #[error("Enum tərifindən sonra `newline` gözlənilirdi, tapıldı: {0}")]
    EnumNewlineError(String),

    #[error("Enum variantı gözlənilirdi, tapıldı: {0}")]
    EnumVariantNotFound(String),

    #[error("Gözlənilməz token: {0}")]
    UnexpectedToken(String),

    #[error("Bilinməyən hazır funksiya: {0}")]
    UnknownFunction(String),
    /* Match Errors */
    #[error("Match parsing xətası: {0}")]
    MatchError(String),

    #[error("Gözlənilməz token match armında: {0}")]
    MatchUnknownToken(String),

    /* Function Errors */
    #[error("Funksiya adı gözlənilirdi, tapıldı '{0}'")]
    FunctionName(String),

    #[error("Parametr adı gözlənilirdi, tapıldı: {0}")]
    ParamNameNotFound(String),

    #[error("Parametrdən sonra ',' və ya ')' gözlənilirdi, tapıldı: {0}")]
    ParamError(String),
}
