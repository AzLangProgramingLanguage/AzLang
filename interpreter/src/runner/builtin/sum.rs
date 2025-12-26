use crate::runner::Runner;
use crate::runner::builtin::print;
use crate::runner::runner::runner_interpretator;
use parser::ast::{Expr, TemplateChunk};
use std::fmt::Write;
use std::mem;

pub fn sum<'a>(expr: Expr<'a>, ctx: &mut Runner<'a>) -> Expr<'a> {
    expr
}
