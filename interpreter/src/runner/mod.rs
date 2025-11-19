use std::{collections::HashMap, rc::Rc};
mod builtin;
mod helpers;
mod runner;
use crate::parser::ast::{Expr, MethodType, Program, Type};
mod eval;
mod handlers;

#[derive(Debug)]
struct Variable<'a> {
    value: Expr<'a>,
    typ: Type<'a>,
    is_mutable: bool,
}

#[derive(Debug)]
struct Method<'a> {
    name: &'a str,
    params: Vec<(String, Type<'a>)>, // bunları da istəsən arena-dan ala bilərsən
    body: Vec<Expr<'a>>,
    return_type: Option<Type<'a>>,
}

#[derive(Debug)]
pub struct StructDef<'a> {
    name: &'a str,
    fields: Vec<(&'a str, Type<'a>, Option<Expr<'a>>)>,
    methods: Vec<Method<'a>>,
}

#[derive(Debug)]
pub struct FunctionDef<'a> {
    params: Vec<(String, Type<'a>)>,
    body: Rc<Vec<Expr<'a>>>,
    return_type: Type<'a>,
}

#[derive(Debug)]
pub struct UnionType<'a> {
    name: &'a str,
    fields: Vec<(&'a str, Type<'a>)>,
    methods: Vec<Method<'a>>,
}

#[derive(Debug)]
pub struct Runner<'a> {
    variables: HashMap<String, Variable<'a>>,
    structdefs: HashMap<String, StructDef<'a>>,
    functions: HashMap<String, FunctionDef<'a>>,
    uniontypes: HashMap<String, UnionType<'a>>,
}

impl<'a> Runner<'a> {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            structdefs: HashMap::new(),
            functions: HashMap::new(),
            uniontypes: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: Program<'a>) {
        for expr in program.expressions.into_iter() {
            runner::runner_interpretator(self, expr);
        }
    }
}
