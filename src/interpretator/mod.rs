use std::collections::HashMap;
use std::rc::Rc;
mod builtin;
mod runner_interpretator;
use crate::parser::ast::{Expr, Program, Type};

struct Variable<'a> {
    value: Box<Expr<'a>>,
    typ: Rc<Type<'a>>,
    is_mutable: bool,
}

pub struct InterPretator<'a> {
    variables: HashMap<String, Variable<'a>>,
}
impl<'a> InterPretator<'a> {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    pub fn run(&mut self, program: Program<'a>) {
        for expr in program.expressions {
            runner_interpretator::runner_interpreator(self, expr);
        }
    }
}
