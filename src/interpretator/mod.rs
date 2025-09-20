use std::collections::HashMap;
use std::rc::Rc;
mod builtin;

mod runner_interpretator;
use crate::parser::ast::{Expr, MethodType, Program, Type};

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
struct StructDef<'a> {
    name: &'a str,
    fields: Vec<(&'a str, Type<'a>, Option<Expr<'a>>)>,
    methods: Vec<Method<'a>>,
}
#[derive(Debug)]
pub struct FunctionDef<'a> {
    params: Vec<(String, Type<'a>)>,
    body: Rc<Vec<Expr<'a>>>,
    return_type: Option<Type<'a>>,
}
#[derive(Debug)]
pub struct InterPretator<'a> {
    variables: HashMap<String, Variable<'a>>,
    structdefs: HashMap<String, StructDef<'a>>,
    functions: HashMap<String, FunctionDef<'a>>,
}
impl<'a> InterPretator<'a> {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            structdefs: HashMap::new(),
            functions: HashMap::new(),
        }
    }
    pub fn run(&mut self, program: Program<'a>) {
        for expr in program.expressions {
            runner_interpretator::runner_interpretator(self, expr);
        }
    }
}
