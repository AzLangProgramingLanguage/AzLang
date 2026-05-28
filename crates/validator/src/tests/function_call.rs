#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use parser::{
        ast::{Expr, FunctionDef, Parameter, Program, Statement, Symbol},
        shared_ast::Type,
    };

    use crate::{Validator, function_call::validate_function_call, validate};

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

        validator.functions = program.functions;
        assert_eq!(
            validator.functions.get("Hello"),
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
        );
        // let function_call = Statement::Expr(Expr::Call {
        //     target: None,
        //     name: Box::new(Expr::VariableRef {
        //         name: "Hello".to_string(),
        //         symbol: Some(Symbol {
        //             typ: Type::Function,
        //             is_mutable: false,
        //             is_pointer: false,
        //             is_used: true,
        //             is_changed: false,
        //         }),
        //     }),
        //     args: vec![],
        //     returned_type: None,
        // });
        assert_eq!(
            validate_function_call(
                &mut validator,
                &mut None,
                &mut vec![],
                &mut None,
                &mut Box::new(Expr::VariableRef {
                    name: "Hello".to_string(),
                    symbol: Some(Symbol {
                        typ: Type::Function,
                        is_mutable: false,
                        is_pointer: false,
                        is_used: true,
                        is_changed: false,
                    }),
                }),
            ),
            Err(crate::errors::ValidatorError::InvalidArgumentCount {
                name: "Hello".to_string(),
                expected: 1,
                found: 0
            })
        );
    }
}
