use std::collections::HashMap;
use std::rc::Rc;
mod runner_interpretator;
use crate::parser::ast::{Expr, Program, Type};
pub struct InterPretator<'a> {
    variables: HashMap<String, (Rc<Type<'a>>, Box<Expr<'a>>)>,
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
