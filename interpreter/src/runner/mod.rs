use std::{collections::HashMap, rc::Rc};
mod builtin;
pub mod function_call;
mod helpers;
mod runner;
mod tests;
use parser::ast::Expr as ParserExpr;
use parser::{
    ast::{FunctionDef, Statement},
    shared_ast::Type,
};
use validator::ast::{Ast, Expr};

use crate::{Function, runner::runner::Value};
mod binary_op;
mod handlers;

#[derive(Debug)]
pub struct Variable {
    value: Value,
}

#[derive(Debug)]
struct Method {
    name: String,
    params: Vec<(String, Type)>,
    body: Vec<ParserExpr>,
    return_type: Option<Type>,
}

#[derive(Debug)]
pub struct StructDef {
    name: String,
    fields: Vec<(String, Type, Option<ParserExpr>)>,
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
    pub functions: HashMap<String, Function>,
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

    pub fn run(&mut self, expr: Ast) {
        runner::runner_interpretator(self, expr);
    }
}
