use crate::{context::Parameter, lexer::Token};

#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct MatchExpr {
    pub target: Box<Expr>,             // uyğun dəyişən
    pub arms: Vec<(Token, Vec<Expr>)>, // "variant adı" və ifadə bloku
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInFunction {
    Print,
    Input,
    Len,
    Number,
    Sum,
    Range,
    LastWord,
}

#[derive(Debug, Clone)]
pub enum Expr {
    String(String),
    Number(i64),
    Bool(bool),
    Break,
    Continue,
    If {
        condition: Box<Expr>,
        then_branch: Vec<Expr>,
        else_branch: Vec<Expr>, // Else və ElseIf ardıcıllığı daxil olan blok
    },
    ElseIf {
        condition: Box<Expr>,
        then_branch: Vec<Expr>,
    },

    Else {
        then_branch: Vec<Expr>,
    },
    MethodCall {
        target: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    StructDef {
        name: String,
        fields: Vec<(String, Type)>,
        methods: Vec<(String, Vec<Parameter>, Vec<Expr>, Option<Type>)>,
    },
    EnumDecl(EnumDecl),

    StructInit {
        name: String,
        args: Vec<Expr>,
    },
    Match(Box<MatchExpr>),
    FieldAccess {
        target: Box<Expr>,
        field: String,
    },
    TemplateString(Vec<TemplateChunk>),
    Loop {
        var_name: String,
        iterable: Box<Expr>,
        body: Vec<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    Return(Box<Expr>),
    BuiltInCall {
        func: BuiltInFunction,
        args: Vec<Expr>,
        resolved_type: Option<Type>,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    MutableDecl {
        name: String,
        typ: Option<Type>,
        value: Box<Expr>,
    },
    ConstantDecl {
        name: String,
        typ: Option<Type>,
        value: Box<Expr>,
    },
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
    VariableRef(String),
    List(Vec<Expr>),
    FunctionDef {
        name: String,
        params: Vec<Parameter>,
        body: Vec<Expr>,
        return_type: Option<Type>,
    },
}

#[derive(Debug, Clone)]
pub enum TemplateChunk {
    Literal(String),
    Expr(Box<Expr>),
}

#[derive(Debug)]
pub struct Program {
    pub expressions: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Metn,
    Siyahi(Box<Type>),
    Istifadeci(String),
    Integer,
    BigInteger,
    LowInteger,
    Bool,
    Char,
    Void,
    Any,
}
