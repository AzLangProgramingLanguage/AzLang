#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use parser::{
        ast::{Expr, Statement},
        shared_ast::Type,
    };

    use crate::TranspileContext;

    #[test]
    fn variable_decl_test() {
        let statement = Statement::Decl {
            name: 'a'.to_string(),
            typ: Rc::new(Type::Natural),
            is_mutable: false,
            value: Box::new(Expr::Number(1)),
        };
    }
}
