use std::{collections::HashMap, rc::Rc};
pub mod function_call;
mod helpers;
mod runner;
mod tests;
use parser::ast::Expr as ParserExpr;
use parser::{
    ast::{FunctionDef, Parameter, Statement},
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

#[derive(Debug, Clone)]
pub struct ExternalFunction {
    pub library: String,
    pub symbol: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
}

#[derive(Debug)]
pub struct Runner {
    variables: HashMap<String, Variable>,
    structdefs: HashMap<String, StructDef>,
    pub functions: HashMap<String, Function>,
    uniontypes: HashMap<String, UnionType>,
    pub external_functions: HashMap<String, ExternalFunction>,
    current_return: Expr,
    pub should_break: bool,
    pub should_continue: bool,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            structdefs: HashMap::new(),
            functions: HashMap::new(),
            uniontypes: HashMap::new(),
            external_functions: HashMap::new(),
            current_return: Expr::Void,
            should_break: false,
            should_continue: false,
        }
    }

    pub fn run(&mut self, expr: Ast) {
        runner::runner_interpretator(self, expr);
    }
}
