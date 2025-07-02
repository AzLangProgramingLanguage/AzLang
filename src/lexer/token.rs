use crate::parser::ast::Type;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    MutableDecl,
    ConstantDecl,
    FunctionDef,
    Underscore,
    Conditional,
    Float(f64),
    Backtick,
    InterpolationStart,
    InterpolationEnd,
    SiyahiKeyword,
    Object,
    Else,
    This,
    End,
    Drop,
    TypeName(Type),
    TipDecl, // tip
    Match,   // uyğun
    Arrow,   // ->
    Loop,
    Break,
    Print,
    Continue,
    Newline,
    Whitespace,
    BigInteger,
    Method,
    LowInteger,
    Integer,
    True,
    False,
    String,
    Identifier(String),
    Number(i64),
    Indent,
    Dedent,
    ElseIf,
    StringLiteral(String),
    Operator(String),
    LParen,
    Dot,
    RParen,
    LBrace,
    RBrace,
    In,
    Semicolon,
    Colon,
    Comma,
    ListStart,
    ListEnd,
    Return,
    EOF,
    And,       // və
    Or,        // və ya
    DoubleAnd, // &&
    DoubleOr,  // ||
    NumberFn,
    RangeFn,
    Input,
}
