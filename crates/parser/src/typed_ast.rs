use std::borrow::Cow;
use std::rc::Rc;

use crate::shared_ast::{BuiltInFunction, Type};

#[derive(Debug)]
pub struct MethodTypeTyped<'a> {
    pub name: &'a str,
    pub transpiled_name: Option<Cow<'a, str>>,
    pub params: Vec<ParameterTyped<'a>>,
    pub body: Vec<TypedExpr<'a>>,
    pub return_type: Option<Type<'a>>,
    pub is_allocator: bool,
}

#[derive(Debug)]
pub enum TypedExpr<'a> {
    DynamicString(Rc<String>),
    String(&'a str, bool),
    Bool(bool),
    Number(i64),
    Char(char),
    EnumDecl {
        name: Cow<'a, str>,
        variants: Vec<Cow<'a, str>>,
    },
    Return(Box<TypedExpr<'a>>),
    List(Vec<TypedExpr<'a>>),
    UnaryOp {
        op: &'a str,
        expr: Box<TypedExpr<'a>>,
    },
    Index {
        target: Box<TypedExpr<'a>>,
        index: Box<TypedExpr<'a>>,
        target_type: Type<'a>,
    },
    Loop {
        var_name: &'a str,
        iterable: Box<TypedExpr<'a>>,
        body: Vec<TypedExpr<'a>>,
    },
    Assingment {
        name: &'a str,
        transpiled_name: &'a str,
        value: Box<TypedExpr<'a>>,
        is_pointer: bool,
    },
    Float(f64),
    Decl {
        name: Cow<'a, str>,
        transpiled_name: Option<String>,
        typ: Option<Rc<Type<'a>>>,
        is_mutable: bool,
        is_primitive: bool,
        value: Box<TypedExpr<'a>>,
    },
    VariableRef {
        name: Cow<'a, str>,
        transpiled_name: Option<String>,
        symbol: Option<Symbol<'a>>,
    },
    TemplateString(Vec<TypedTemplateChunk<'a>>),
    If {
        condition: Box<TypedExpr<'a>>,
        then_branch: Vec<TypedExpr<'a>>,
        else_branch: Vec<TypedExpr<'a>>,
    },
    ElseIf {
        condition: Box<TypedExpr<'a>>,
        then_branch: Vec<TypedExpr<'a>>,
    },
    Else {
        then_branch: Vec<TypedExpr<'a>>,
    },
    BuiltInCall {
        function: BuiltInFunction,
        args: Vec<TypedExpr<'a>>,
        return_type: Type<'a>,
    },
    Call {
        target: Option<Box<TypedExpr<'a>>>,
        name: &'a str,
        transpiled_name: Option<String>,
        args: Vec<TypedExpr<'a>>,
        returned_type: Option<Type<'a>>,
        is_allocator: bool,
    },
    StructDef {
        name: &'a str,
        transpiled_name: Option<Cow<'a, str>>,
        fields: Vec<(&'a str, Type<'a>, Option<TypedExpr<'a>>)>,
        methods: Vec<MethodTypeTyped<'a>>,
    },
    FunctionDef {
        name: &'a str,
        transpiled_name: Option<Cow<'a, str>>,
        params: Vec<ParameterTyped<'a>>,
        body: Vec<TypedExpr<'a>>,
        return_type: Option<Type<'a>>,
        is_allocator: bool,
    },
    UnionType {
        name: &'a str,
        transpiled_name: Option<Cow<'a, str>>,
        fields: Vec<(&'a str, Type<'a>)>,
        methods: Vec<MethodTypeTyped<'a>>,
    },
    StructInit {
        name: Cow<'a, str>,
        transpiled_name: Option<Cow<'a, str>>,
        args: Vec<(&'a str, TypedExpr<'a>)>,
    },

    Assignment {
        name: Cow<'a, str>,
        value: Box<TypedExpr<'a>>,
        symbol: Option<Symbol<'a>>,
    },
    BinaryOp {
        left: Box<TypedExpr<'a>>,
        op: &'a str,
        right: Box<TypedExpr<'a>>,
    },
    Comment(&'a str),
    Break,
    Continue,
    Match {
        target: Box<TypedExpr<'a>>,
        arms: Vec<(TypedExpr<'a>, Vec<TypedExpr<'a>>)>,
    },
}

#[derive(Debug)]
pub struct CompiledProgram<'a> {
    pub expressions: Vec<TypedExpr<'a>>,
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
pub enum TypedTemplateChunk<'a> {
    Literal(&'a str),
    TypedExpr(Box<TypedExpr<'a>>),
}

#[derive(Clone, Debug)]
pub struct ParameterTyped<'a> {
    pub name: String,
    pub typ: Type<'a>,
    pub is_mutable: bool,
    pub is_pointer: bool,
}
