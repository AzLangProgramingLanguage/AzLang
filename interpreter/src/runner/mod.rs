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
struct Variable<'a> {
    value: Rc<Expr<'a>>,
    typ: Rc<Type<'a>>,
    is_mutable: bool,
}

#[derive(Debug)]
struct Method<'a> {
    name: &'a str,
    params: Vec<(String, Type<'a>)>,
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
    params: Rc<Vec<Parameter<'a>>>,
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
    current_return: Expr<'a>,
    output: &'a mut String,
}

impl<'a> Runner<'a> {
    pub fn new(output: &'a mut String) -> Self {
        Self {
            variables: HashMap::new(),
            structdefs: HashMap::new(),
            functions: HashMap::new(),
            uniontypes: HashMap::new(),
            current_return: Expr::Void,
            output,
        }
    }

    pub fn run(&mut self, program: Program<'a>) {
        for expr in program.expressions {
            runner::runner_interpretator(self, expr);
        }
    }
}
