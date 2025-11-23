use std::borrow::Cow;
use std::rc::Rc;

use crate::shared_ast::{BuiltInFunction, Type};

#[derive(Debug)]
pub struct MethodType<'a> {
    pub name: &'a str,
    pub transpiled_name: Option<Cow<'a, str>>,
    pub params: Vec<Parameter<'a>>,
    pub body: Vec<Expr<'a>>,
    pub return_type: Option<Type<'a>>,
    pub is_allocator: bool,
}
#[derive(Debug)]
pub struct EnumDecl<'a> {
    pub name: Cow<'a, str>,
    pub variants: Vec<Cow<'a, str>>,
}

#[derive(Debug)]
pub enum Expr<'a> {
    DynamicString(Rc<String>),
    String(&'a str, bool),
    Bool(bool),
    Number(i64),
    Char(char),
    EnumDecl(EnumDecl<'a>),
    Return(Box<Expr<'a>>),
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
    Assingment {
        name: &'a str,
        transpiled_name: &'a str,
        value: Box<Expr<'a>>,
        is_pointer: bool,
    },
    Float(f64),
    Decl {
        name: Cow<'a, str>,
        transpiled_name: Option<String>,
        typ: Option<Rc<Type<'a>>>,
        is_mutable: bool,
        is_primitive: bool,
        value: Box<Expr<'a>>,
    },
    VariableRef {
        name: Cow<'a, str>,
        transpiled_name: Option<String>,
        symbol: Option<Symbol<'a>>,
    },
    TemplateString(Vec<TemplateChunk<'a>>),
    If {
        condition: Box<Expr<'a>>,
        then_branch: Vec<Expr<'a>>,
        else_branch: Vec<Expr<'a>>,
    },
    ElseIf {
        condition: Box<Expr<'a>>,
        then_branch: Vec<Expr<'a>>,
    },
    Else {
        then_branch: Vec<Expr<'a>>,
    },
    BuiltInCall {
        function: BuiltInFunction,
        args: Vec<Expr<'a>>,
        return_type: Type<'a>,
    },
    Call {
        target: Option<Box<Expr<'a>>>,
        name: &'a str,
        transpiled_name: Option<String>,
        args: Vec<Expr<'a>>,
        returned_type: Option<Type<'a>>,
        is_allocator: bool,
    },
    StructDef {
        name: &'a str,
        transpiled_name: Option<Cow<'a, str>>,
        fields: Vec<(&'a str, Type<'a>, Option<Expr<'a>>)>,
        methods: Vec<MethodType<'a>>,
    },
    FunctionDef {
        name: &'a str,
        transpiled_name: Option<Cow<'a, str>>,
        params: Vec<Parameter<'a>>,
        body: Vec<Expr<'a>>,
        return_type: Option<Type<'a>>,
        is_allocator: bool,
    },
    UnionType {
        name: &'a str,
        transpiled_name: Option<Cow<'a, str>>,
        fields: Vec<(&'a str, Type<'a>)>,
        methods: Vec<MethodType<'a>>,
    },
    StructInit {
        name: Cow<'a, str>,
        transpiled_name: Option<Cow<'a, str>>,
        args: Vec<(&'a str, Expr<'a>)>,
    },

    Assignment {
        name: Cow<'a, str>,
        value: Box<Expr<'a>>,
        symbol: Option<Symbol<'a>>,
    },
    BinaryOp {
        left: Box<Expr<'a>>,
        op: &'a str,
        right: Box<Expr<'a>>,
    },
    Break,
    Continue,
    Match {
        target: Box<Expr<'a>>,
        arms: Vec<(Expr<'a>, Vec<Expr<'a>>)>,
    },
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
    pub transpiled_name: Option<String>,
    //pub source_location: Option<Location>,
}

#[derive(Debug)]
pub enum TemplateChunk<'a> {
    Literal(&'a str),
    Expr(Box<Expr<'a>>),
}

#[derive(Clone, Debug)]
pub struct Parameter<'a> {
    pub name: String,
    pub typ: Type<'a>,
    pub is_mutable: bool,
    pub is_pointer: bool,
}
