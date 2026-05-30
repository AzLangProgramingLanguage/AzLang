use std::collections::HashMap;

use parser::{ast::Parameter, shared_ast::{BuiltInFunction, Type}};

struct Function {
    name: String,
    body: Vec<Ast>,
    params: Vec<Parameter>,
    return_typ: Type,
}
pub struct Program {
    pub functions: Vec<Function>,
    pub expressions: Vec<Ast>,
}

pub enum Expr {
    String(String),
    Number(i64),
    BuiltInCall {
        function: BuiltInFunction,
        args: Vec<Expr>,
        return_type: Type,
    },
}

pub struct Decl {
    pub name: String,
    pub typ: Type,
    pub value: Box<Expr>,
}
pub enum Ast {
    Decl(Decl),
    Expr(Expr),
}
