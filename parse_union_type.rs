use peekmore::PeekMoreIterator;
use std::borrow::Cow;

use super::expression::parse_expression;

pub fn parse_union_type<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    std::process::exit(0);
}
