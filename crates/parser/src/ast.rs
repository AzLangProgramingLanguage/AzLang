use std::borrow::Cow;
use std::rc::Rc;

use crate::shared_ast::{BuiltInFunction, Type};

#[derive(Debug)]
pub struct FunctionDef<'a> {
    params: Vec<(String, Type<'a>)>,
    body: Rc<Vec<Expr<'a>>>,
    return_type: Type<'a>,
}

#[derive(Debug, Clone)]
pub struct MethodType<'a> {
    pub name: &'a str,
    pub params: Vec<Parameter<'a>>,
    pub body: Vec<Expr<'a>>,
    pub return_type: Option<Type<'a>>,
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

#[derive(Debug, Clone)]
pub struct IF<'a> {
    pub condition: Box<Expr<'a>>,
    pub body: Vec<Expr<'a>>,
}

#[derive(Debug, Clone)]
pub struct Else<'a> {
    pub body: Vec<Expr<'a>>,
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    DynamicString(Rc<String>),
    Void,
    Return(Box<Expr<'a>>),
    Time(std::time::Instant),
    String(String),
    Bool(bool),
    Number(i64),
    Char(char),
    EnumDecl {
        name: Cow<'a, str>,
        variants: Vec<Cow<'a, str>>,
    },
    Comment(String),
    List(Vec<Expr<'a>>),
    UnaryOp {
        op: &'a str,
        expr: Box<Expr<'a>>,
    },
    Index {
        target: Box<Expr<'a>>,
        index: Box<Expr<'a>>,
        target_type: Type<'a>,
    },
    Loop {
        var_name: &'a str,
        iterable: Box<Expr<'a>>,
        body: Vec<Expr<'a>>,
    },
    Float(f64),
    Decl {
        name: Cow<'a, str>,
        typ: Rc<Type<'a>>,
        is_mutable: bool,
        value: Box<Expr<'a>>,
    },
    VariableRef {
        name: Cow<'a, str>,
        symbol: Option<Symbol<'a>>,
    },
    TemplateString(Vec<TemplateChunk<'a>>),
    Condition {
        main: IF<'a>,
        elif: Vec<IF<'a>>,
        other: Option<Else<'a>>,
    },
    BuiltInCall {
        function: BuiltInFunction,
        args: Vec<Expr<'a>>,
        return_type: Type<'a>,
    },
    Call {
        target: Option<Box<Expr<'a>>>,
        name: String,
        args: Vec<Expr<'a>>,
        returned_type: Option<Type<'a>>,
    },
    StructDef {
        name: &'a str,
        fields: Vec<(&'a str, Type<'a>, Option<Expr<'a>>)>,
        methods: Vec<MethodType<'a>>,
    },
    FunctionDef {
        name: String,
        params: Vec<Parameter<'a>>,
        body: Vec<Expr<'a>>,
        return_type: Option<Type<'a>>,
    },
    UnionType {
        name: &'a str,
        fields: Vec<(&'a str, Type<'a>)>,
        methods: Vec<MethodType<'a>>,
    },
    StructInit {
        name: Cow<'a, str>,
        args: Vec<(&'a str, Expr<'a>)>,
    },

    Assignment {
        name: Cow<'a, str>,
        value: Box<Expr<'a>>,
        symbol: Option<Symbol<'a>>,
    },
    BinaryOp {
        left: Box<Expr<'a>>,
        right: Box<Expr<'a>>,
        op: Operation,
        return_type: Type<'a>,
    },
    Break,
    Continue,
    Match {
        target: Box<Expr<'a>>,
        arms: Vec<(Expr<'a>, Vec<Expr<'a>>)>,
    },
}
impl<'a> Expr<'a> {
    pub fn as_number(&self) -> Option<i64> {
        match self {
            Expr::Number(n) => Some(*n),
            _ => Some(0),
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Expr::Float(f) => Some(*f),
            _ => Some(0.0),
        }
    }
}

#[derive(Debug)]
pub struct Program<'a> {
    pub expressions: Vec<Expr<'a>>,
}

#[derive(Clone, Debug)]
pub struct Symbol<'a> {
    pub typ: Type<'a>,
    pub is_mutable: bool,
    pub is_pointer: bool,
    pub is_used: bool,
    pub is_changed: bool,
    //pub source_location: Option<Location>,
}

#[derive(Debug, Clone)]
pub enum TemplateChunk<'a> {
    Literal(String),
    Expr(Box<Expr<'a>>),
}

#[derive(Clone, Debug)]
pub struct Parameter<'a> {
    pub name: String,
    pub typ: Type<'a>,
    pub is_mutable: bool,
    pub is_pointer: bool,
}
