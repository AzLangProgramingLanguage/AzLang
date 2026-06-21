#[cfg(test)]
mod tests {
    use crate::{Validator, errors::ValidatorError};
    use parser::{
        ast::{Atom, Expr, Statement},
        shared_ast::Type,
    };
    use std::rc::Rc;

    #[test]
    fn test_while_loop_valid_bool_condition() {
        let validator = Validator::default();
        let program = vec![Statement::While {
            condition: Box::new(Expr::Bool(true)),
            body: vec![Statement::Expr(Expr::Number(1))],
        }];
        let result = validator.validate(program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_while_loop_invalid_condition_type() {
        let validator = Validator::default();
        let program = vec![Statement::While {
            condition: Box::new(Expr::Number(42)),
            body: vec![],
        }];
        let result = validator.validate(program);
        assert!(matches!(
            result,
            Err(ValidatorError::TypeMismatch { .. })
        ));
    }

    #[test]
    fn test_while_loop_body_with_decl() {
        let validator = Validator::default();
        let program = vec![
            Statement::Decl {
                name: Atom::from("x"),
                typ: Rc::new(Type::Integer),
                is_mutable: true,
                value: Box::new(Expr::Number(0)),
            },
            Statement::While {
                condition: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::VariableRef {
                        name: Atom::from("x"),
                        symbol: None,
                    }),
                    right: Box::new(Expr::Number(5)),
                    op: parser::ast::Operation::Less,
                }),
                body: vec![
                    Statement::Assignment {
                        name: Atom::from("x"),
                        value: Box::new(Expr::BinaryOp {
                            left: Box::new(Expr::VariableRef {
                                name: Atom::from("x"),
                                symbol: None,
                            }),
                            right: Box::new(Expr::Number(1)),
                            op: parser::ast::Operation::Add,
                        }),
                    },
                ],
            },
            Statement::Expr(Expr::VariableRef {
                name: Atom::from("x"),
                symbol: None,
            }),
        ];
        let result = validator.validate(program);
        assert!(result.is_ok());
    }
}
