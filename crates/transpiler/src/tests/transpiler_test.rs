#[cfg(test)]
mod tests {
    use std::{collections::HashMap, rc::Rc};

    use parser::{
        ast::{Expr, Program, Statement},
        shared_ast::Type,
    };

    use crate::TranspileContext;

    #[test]
    fn transpiler_none() {
        let program = Program {
            functions: HashMap::new(),
            expressions: vec![],
        };
        let mut ctx = TranspileContext::default();
        assert_eq!(ctx.transpile(program), "")
    }
    #[test]
    fn transpile_variable() {
        let statement = Statement::Decl {
            name: "a".to_string(),
            typ: Rc::new(Type::Natural),
            is_mutable: false,
            value: Box::new(Expr::Number(1)),
        };
        let program = Program {
            functions: HashMap::new(),
            expressions: vec![statement],
        };
    }
}
