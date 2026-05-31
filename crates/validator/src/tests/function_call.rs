#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use parser::{
        ast::{Expr, FunctionDef, Parameter, Statement, Symbol},
        shared_ast::Type,
    };

    use crate::Validator;

    #[test]
    fn function_call_with_argument() {
        let function = vec![Statement::FunctionDef {
            name: "Hello".to_string(),
            params: vec![Parameter {
                name: "a".to_string(),
                typ: Type::Integer,
                is_pointer: false,
            }],
            body: vec![Statement::Expr(Expr::Return(Box::new(Expr::VariableRef {
                name: "a".to_string(),
                symbol: None,
            })))],
            return_typ: Type::Integer,
        }];

        let mut validator = Validator::default();
        let result = validator.validate(function).expect("Validate olunamadı");
    }
}
