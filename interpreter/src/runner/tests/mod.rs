#[cfg(test)]
mod binary_op;
mod tests {
    use validator::ast::{Ast, Expr};

    use crate::Function;
    use crate::runner::{Runner, function_call::function_call, runner::Value};
    use parser::shared_ast::Type;

    #[test]
    fn function_call_test() {
        let mut runner = Runner::new();
        let mut functions = std::collections::HashMap::new();
        functions.insert(
            "Hello".to_string(),
            Function {
                params: vec![],
                body: vec![],
                return_type: Type::Void,
            },
        );
        runner.functions = functions;
        let val = function_call(
            &mut runner,
            None,
            Box::new(Expr::VariableRef {
                name: "Hello".to_string(),
                symbol: None,
            }),
            vec![],
            None,
        );
        assert_eq!(val, Value::Void)
    }

    #[test]
    fn function_call_returned_value_test() {
        let mut runner = Runner::new();
        let mut functions = std::collections::HashMap::new();
        functions.insert(
            "Hello".to_string(),
            Function {
                params: vec![],
                body: vec![Ast::Expr(Expr::Return(Box::new(Expr::Number(1))))],
                return_type: Type::Integer,
            },
        );
        runner.functions = functions;
        let val = function_call(
            &mut runner,
            None,
            Box::new(Expr::VariableRef {
                name: "Hello".to_string(),
                symbol: None,
            }),
            vec![],
            Some(Type::Integer),
        );
        assert_eq!(val, Value::Number(1))
    }

    #[test]
    fn function_call_return_from_argument() {
        let mut runner = Runner::new();
        let mut functions = std::collections::HashMap::new();
        functions.insert(
            "Hello".to_string(),
            Function {
                params: vec![parser::ast::Parameter {
                    name: "a".to_string(),
                    typ: Type::Integer,
                    is_pointer: false,
                }],
                body: vec![Ast::Expr(Expr::Return(Box::new(Expr::VariableRef {
                    name: "a".to_string(),
                    symbol: None,
                })))],
                return_type: Type::Integer,
            },
        );
        runner.functions = functions;
        let val = function_call(
            &mut runner,
            None,
            Box::new(Expr::VariableRef {
                name: "Hello".to_string(),
                symbol: None,
            }),
            vec![Expr::Number(1)],
            Some(Type::Integer),
        );
        assert_eq!(val, Value::Number(1))
    }
}
