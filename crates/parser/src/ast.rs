use crate::shared_ast::{BuiltInFunction, Type};
use std::{collections::HashMap, fmt::Display, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct MethodType {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Vec<Expr>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Not,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IF {
    pub condition: Box<Expr>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Else {
    pub body: Vec<Statement>,
}
#[derive(Debug, PartialEq)]
pub struct FunctionDef {
    pub params: Vec<Parameter>,
    pub body: Vec<Statement>,
    pub return_type: Option<Type>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    EnumDecl {
        name: String,
        variants: Vec<String>,
    },
    Decl {
        name: String,
        typ: Rc<Type>,
        is_mutable: bool,
        value: Box<Expr>,
    },
    StructDef {
        name: String,
        fields: Vec<(String, Type, Option<Expr>)>,
        methods: Vec<MethodType>,
    },

    UnionType {
        name: String,
        fields: Vec<(String, Type)>,
        methods: Vec<MethodType>,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    Match {
        target: Box<Expr>,
        arms: Vec<(Expr, Vec<Expr>)>,
    },
    Condition {
        main: IF,
        elif: Vec<IF>,
        other: Option<Else>,
    },
    Loop {
        var_name: String,
        iterable: Box<Expr>,
        body: Vec<Expr>,
    },
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    DynamicString(Rc<String>),
    Void,
    Return(Box<Expr>),
    Time(std::time::Instant),
    String(String),
    Bool(bool),
    Number(i64),
    Char(char),
    Comment(String),
    List(Vec<Expr>),
    UnaryOp {
        op: Operation,
        expr: Box<Expr>,
    },
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
        target_type: Type,
    },

    Float(f64),
    VariableRef {
        name: String,
        symbol: Option<Symbol>,
    },
    TemplateString(Vec<TemplateChunk>),

    BuiltInCall {
        function: BuiltInFunction,
        args: Vec<Expr>,
        return_type: Type,
    },
    Call {
        target: Option<Box<Expr>>,
        name: Box<Expr>,
        args: Vec<Expr>,
        returned_type: Option<Type>,
    },

    StructInit {
        name: String,
        args: Vec<(String, Expr)>,
    },

    BinaryOp {
        left: Box<Expr>,
        right: Box<Expr>,
        op: Operation,
        return_type: Type,
    },
    Break,
    Continue,
}
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{n}"),
            Expr::String(s) => write!(f, "\"{s}\""),
            Expr::Bool(b) => write!(f, "{b}"),
            Expr::Char(c) => write!(f, "'{c}'"),
            other => write!(f, "{other:?}"),
        }
    }
}

impl Expr {
    pub fn as_number(&self) -> i64 {
        match self {
            Expr::Number(n) => *n,
            _ => 0,
        }
    }
    pub fn as_float(&self) -> f64 {
        match self {
            Expr::Float(f) => *f,
            _ => 0.0,
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub functions: HashMap<String, FunctionDef>,
    pub expressions: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
    pub typ: Type,
    pub is_mutable: bool,
    pub is_pointer: bool,
    pub is_used: bool,
    pub is_changed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateChunk {
    Literal(String),
    Expr(Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub typ: Type,
    pub is_mutable: bool,
    pub is_pointer: bool,
}
