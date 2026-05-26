#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use parser::{
        ast::{Expr, FunctionDef, Parameter, Program, Statement, Symbol},
        shared_ast::Type,
    };

    use crate::{Validator, validate};

    #[test]
    fn function_call_with_argument() {
        let function = FunctionDef {
            params: vec![Parameter {
                name: "a".to_string(),
                typ: Type::Integer,
                is_mutable: false,
                is_pointer: false,
            }],
            body: vec![Statement::Expr(Expr::Return(Box::new(Expr::VariableRef {
                name: "a".to_string(),
                symbol: None,
            })))],
            return_type: Some(Type::Integer),
        };
        let mut functions = HashMap::new();
        functions.insert("Hello".to_string(), function);
        let mut program = Program {
            functions,
            expressions: vec![],
        };

        let mut validator = Validator::new();
        validator.validate(&mut program);
        assert_eq!(
            program.functions.get("Hello"),
            Some(&FunctionDef {
                params: vec![Parameter {
                    name: "a".to_string(),
                    typ: Type::Integer,
                    is_mutable: false,
                    is_pointer: false,
                }],
                body: vec![Statement::Expr(Expr::Return(Box::new(Expr::VariableRef {
                    name: "a".to_string(),
                    symbol: Some(Symbol {
                        typ: Type::Integer,
                        is_mutable: false,
                        is_pointer: false,
                        is_used: true,
                        is_changed: false,
                    })
                })))],
                return_type: Some(Type::Integer),
            })
        )
    }
}
