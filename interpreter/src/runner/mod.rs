use std::{collections::HashMap, rc::Rc};
mod builtin;
mod function_call;
mod helpers;
mod runner;
use parser::{
    ast::{Expr, Parameter, Program},
    shared_ast::Type,
};
mod binary_op;
mod handlers;

#[derive(Debug)]
struct Variable {
    value: Rc<Expr>,
    typ: Rc<Type>,
    is_mutable: bool,
}

#[derive(Debug)]
struct Method {
    name: String,
    params: Vec<(String, Type)>,
    body: Vec<Expr>,
    return_type: Option<Type>,
}

#[derive(Debug)]
pub struct StructDef {
    name: String,
    fields: Vec<(String, Type, Option<Expr>)>,
    methods: Vec<Method>,
}

#[derive(Debug)]
pub struct FunctionDef {
    params: Rc<Vec<Parameter>>,
    body: Rc<Vec<Expr>>,
    return_type: Type,
}

#[derive(Debug)]
pub struct UnionType {
    name: String,
    fields: Vec<(String, Type)>,
    methods: Vec<Method>,
}

#[derive(Debug)]
pub struct Runner {
    variables: HashMap<String, Variable>,
    structdefs: HashMap<String, StructDef>,
    functions: HashMap<String, FunctionDef>,
    uniontypes: HashMap<String, UnionType>,
    current_return: Expr,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            structdefs: HashMap::new(),
            functions: HashMap::new(),
            uniontypes: HashMap::new(),
            current_return: Expr::Void,
        }
    }

    pub fn run(&mut self, expr: Expr) {
        runner::runner_interpretator(self, expr);
    }
}

