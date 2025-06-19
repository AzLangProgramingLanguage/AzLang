pub mod array_methods;
pub mod declaration;
pub mod expr;
pub mod function;
pub mod r#loop;
pub mod main_fn;
pub mod string_methods;
pub mod utils;

use crate::{context::TranspileContext, parser::Program};
use main_fn::generate_main_fn;

pub fn transpile(
    program: &Program,
    ctx: &mut TranspileContext,
    message: &dyn Fn(&str),
) -> Result<String, String> {
    generate_main_fn(program, ctx, message)
}
