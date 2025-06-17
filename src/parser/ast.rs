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
        else_branch: Option<Vec<Expr>>,
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
        params: Vec<(String, Type, bool)>,
        body: Vec<Expr>,
    },
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
    Any,
}
