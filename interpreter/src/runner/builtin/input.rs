use std::rc::Rc;

use parser::ast::Expr;

pub fn input<'a>() -> Expr<'a> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Input girilmedi");
    Expr::DynamicString(Rc::new(input))
}
