use std::{collections::HashMap, rc::Rc};
mod builtin;
//mod function_call;
mod helpers;
mod runner;
use parser::{
    ast::{Expr, FunctionDef, Parameter, Program, Statement},
    shared_ast::Type,
};

use crate::runner::runner::Value;
mod binary_op;
mod handlers;

#[derive(Debug)]
pub struct Variable {
    value: Value,
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
pub struct UnionType {
    name: String,
    fields: Vec<(String, Type)>,
    methods: Vec<Method>,
}

#[derive(Debug)]
pub struct Runner {
    variables: HashMap<String, Variable>,
    structdefs: HashMap<String, StructDef>,
    pub functions: HashMap<String, FunctionDef>,
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

    pub fn run(&mut self, expr: Statement) {
        runner::runner_interpretator(self, expr);
    }
}
