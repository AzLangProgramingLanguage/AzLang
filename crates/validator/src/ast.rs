use parser::{
    ast::{Operation, Parameter, Symbol},
    shared_ast::Type,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub body: Vec<Ast>,
    pub params: Vec<Parameter>,
    pub return_typ: Type,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ExternalFunctionDef {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_typ: Type,
    pub library: String,
    pub symbol: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub functions: Vec<Function>,
    pub expressions: Vec<Ast>,
    pub external_functions: Vec<ExternalFunctionDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IF {
    pub condition: Box<Expr>,
    pub body: Vec<Ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Else {
    pub body: Vec<Ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateChunk {
    Literal(String),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    String(String),
    Number(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    TemplateString(Vec<TemplateChunk>),
    List(Vec<Expr>),
    Void,
    Return(Box<Expr>),
    VariableRef {
        name: String,
        symbol: Symbol,
    },
    BinaryOp {
        left: Box<Expr>,
        right: Box<Expr>,
        op: Operation,
        return_type: Type,
    },
    Call {
        target: Option<Box<Expr>>,
        name: Box<Expr>,
        args: Vec<Expr>,
        returned_type: Type,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Decl {
        name: String,
        typ: Type,
        is_mutable: bool,
        value: Box<Expr>,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    Condition {
        main: IF,
        elif: Vec<IF>,
        other: Option<Else>,
    },
    Expr(Expr),
}
