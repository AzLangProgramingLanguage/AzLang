#[cfg(test)]

mod tests {
    use std::collections::HashMap;

    use parser::{
        ast::{Expr, FunctionDef, Statement, Symbol},
        shared_ast::Type,
    };

    use crate::runner::{Runner, function_call::function_call, runner::Value};

    #[test]
    fn function_call_test() {
        let mut runner = Runner::new();
        let mut functions = HashMap::new();
        let function = FunctionDef {
            params: vec![],
            body: vec![],
            return_type: None,
        };
        functions.insert("Hello".to_string(), function);
        runner.functions = functions;
        let val = function_call(
            &mut runner,
            None,
            Box::new(Expr::VariableRef {
                name: "Hello".to_string(),
                symbol: Some(Symbol {
                    typ: Type::Function,
                    is_mutable: false,
                    is_pointer: false,
                    is_used: false,
                    is_changed: false,
                }),
            }),
            vec![],
            None,
        );
        assert_eq!(val, Value::Void)
    }
    #[test]
    fn function_call_returned_value_test() {
        let mut runner = Runner::new();
        let mut functions = HashMap::new();
        let function = FunctionDef {
            params: vec![],
            body: vec![Statement::Expr(Expr::Number(1))],
            return_type: Some(Type::Integer),
        };
        functions.insert("Hello".to_string(), function);
        runner.functions = functions;
        let val = function_call(
            &mut runner,
            None,
            Box::new(Expr::VariableRef {
                name: "Hello".to_string(),
                symbol: Some(Symbol {
                    typ: Type::Function,
                    is_mutable: false,
                    is_pointer: false,
                    is_used: false,
                    is_changed: false,
                }),
            }),
            vec![],
            Some(Type::Integer),
        );
        assert_eq!(val, Value::Number(1))
    }
}
