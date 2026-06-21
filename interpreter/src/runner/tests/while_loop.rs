#[cfg(test)]
mod tests {
    use crate::Runner;
    use crate::runner::runner::{get_primitive_value, Value, runner_interpretator};
    use validator::ast::{Ast, Expr};
    use parser::shared_ast::Type;
    use parser::ast::{Operation, Symbol};

    #[test]
    fn while_loop_runs_zero_times_when_false() {
        let mut runner = Runner::new();
        let stmt = Ast::While {
            condition: Box::new(Expr::Bool(false)),
            body: vec![Ast::Expr(Expr::Return(Box::new(Expr::Number(42))))],
        };
        runner_interpretator(&mut runner, stmt);
        assert_eq!(runner.current_return, Expr::Void);
    }

    #[test]
    fn while_loop_runs_until_condition_false() {
        let mut runner = Runner::new();

        let decl = Ast::Decl {
            name: "x".to_string(),
            typ: Type::Integer,
            is_mutable: true,
            value: Box::new(Expr::Number(0)),
        };
        runner_interpretator(&mut runner, decl);

        let condition = Expr::BinaryOp {
            left: Box::new(Expr::VariableRef {
                name: "x".to_string(),
                symbol: Symbol {
                    typ: Type::Integer,
                    is_mutable: true,
                    is_used: false,
                    is_changed: false,
                },
            }),
            right: Box::new(Expr::Number(3)),
            op: Operation::Less,
            return_type: Type::Bool,
        };

        let body_assign = Ast::Assignment {
            name: "x".to_string(),
            value: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::VariableRef {
                    name: "x".to_string(),
                    symbol: Symbol {
                        typ: Type::Integer,
                        is_mutable: true,
                        is_used: false,
                        is_changed: false,
                    },
                }),
                right: Box::new(Expr::Number(1)),
                op: Operation::Add,
                return_type: Type::Integer,
            }),
        };

        let while_stmt = Ast::While {
            condition: Box::new(condition),
            body: vec![body_assign],
        };
        runner_interpretator(&mut runner, while_stmt);

        let final_x = get_primitive_value(
            &mut runner,
            Expr::VariableRef {
                name: "x".to_string(),
                symbol: Symbol {
                    typ: Type::Integer,
                    is_mutable: true,
                    is_used: false,
                    is_changed: false,
                },
            },
            None,
        );
        assert_eq!(final_x, Value::Number(3));
    }

    #[test]
    fn while_loop_break_terminates_early() {
        let mut runner = Runner::new();

        let decl_x = Ast::Decl {
            name: "x".to_string(),
            typ: Type::Integer,
            is_mutable: true,
            value: Box::new(Expr::Number(0)),
        };
        runner_interpretator(&mut runner, decl_x);

        let condition = Expr::Bool(true);

        let inc_x = Ast::Assignment {
            name: "x".to_string(),
            value: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::VariableRef {
                    name: "x".to_string(),
                    symbol: Symbol {
                        typ: Type::Integer,
                        is_mutable: true,
                        is_used: false,
                        is_changed: false,
                    },
                }),
                right: Box::new(Expr::Number(1)),
                op: Operation::Add,
                return_type: Type::Integer,
            }),
        };

        let check_x = Expr::BinaryOp {
            left: Box::new(Expr::VariableRef {
                name: "x".to_string(),
                symbol: Symbol {
                    typ: Type::Integer,
                    is_mutable: true,
                    is_used: false,
                    is_changed: false,
                },
            }),
            right: Box::new(Expr::Number(2)),
            op: Operation::Equal,
            return_type: Type::Bool,
        };

        let break_if_x_eq_2 = Ast::Condition {
            main: validator::ast::IF {
                condition: Box::new(check_x),
                body: vec![Ast::Expr(Expr::Break)],
            },
            elif: vec![],
            other: None,
        };

        let while_stmt = Ast::While {
            condition: Box::new(condition),
            body: vec![inc_x, break_if_x_eq_2],
        };
        runner_interpretator(&mut runner, while_stmt);

        let final_x = get_primitive_value(
            &mut runner,
            Expr::VariableRef {
                name: "x".to_string(),
                symbol: Symbol {
                    typ: Type::Integer,
                    is_mutable: true,
                    is_used: false,
                    is_changed: false,
                },
            },
            None,
        );
        assert_eq!(final_x, Value::Number(2));
    }

    #[test]
    fn while_loop_continue_skips_rest_of_body() {
        let mut runner = Runner::new();

        let decl_x = Ast::Decl {
            name: "x".to_string(),
            typ: Type::Integer,
            is_mutable: true,
            value: Box::new(Expr::Number(0)),
        };
        runner_interpretator(&mut runner, decl_x);

        let decl_y = Ast::Decl {
            name: "y".to_string(),
            typ: Type::Integer,
            is_mutable: true,
            value: Box::new(Expr::Number(0)),
        };
        runner_interpretator(&mut runner, decl_y);

        let condition = Expr::BinaryOp {
            left: Box::new(Expr::VariableRef {
                name: "x".to_string(),
                symbol: Symbol {
                    typ: Type::Integer,
                    is_mutable: true,
                    is_used: false,
                    is_changed: false,
                },
            }),
            right: Box::new(Expr::Number(5)),
            op: Operation::Less,
            return_type: Type::Bool,
        };

        let var_ref_x = || Expr::VariableRef {
            name: "x".to_string(),
            symbol: Symbol {
                typ: Type::Integer,
                is_mutable: true,
                is_used: false,
                is_changed: false,
            },
        };

        let var_ref_y = || Expr::VariableRef {
            name: "y".to_string(),
            symbol: Symbol {
                typ: Type::Integer,
                is_mutable: true,
                is_used: false,
                is_changed: false,
            },
        };

        let inc_x = Ast::Assignment {
            name: "x".to_string(),
            value: Box::new(Expr::BinaryOp {
                left: Box::new(var_ref_x()),
                right: Box::new(Expr::Number(1)),
                op: Operation::Add,
                return_type: Type::Integer,
            }),
        };

        let x_eq_3 = Expr::BinaryOp {
            left: Box::new(var_ref_x()),
            right: Box::new(Expr::Number(3)),
            op: Operation::Equal,
            return_type: Type::Bool,
        };

        let continue_if_x_eq_3 = Ast::Condition {
            main: validator::ast::IF {
                condition: Box::new(x_eq_3),
                body: vec![Ast::Expr(Expr::Continue)],
            },
            elif: vec![],
            other: None,
        };

        let inc_y = Ast::Assignment {
            name: "y".to_string(),
            value: Box::new(Expr::BinaryOp {
                left: Box::new(var_ref_y()),
                right: Box::new(Expr::Number(1)),
                op: Operation::Add,
                return_type: Type::Integer,
            }),
        };

        let while_stmt = Ast::While {
            condition: Box::new(condition),
            body: vec![inc_x, continue_if_x_eq_3, inc_y],
        };
        runner_interpretator(&mut runner, while_stmt);

        let final_y = get_primitive_value(&mut runner, var_ref_y(), None);
        assert_eq!(final_y, Value::Number(4));
    }

    #[test]
    fn while_loop_full_pipeline_break_continue() {
        let src = "dəyişən ədəd i = 0\nolduqca i < 10\n    i = i + 1\n    əgər i == 3\n        davam\n    əgər i == 7\n        dayan\n";

        let parsed = parser::parser(src.to_string()).expect("parse error");
        let validator = validator::Validator::default();
        let (_validator, program) = validator.validate(parsed).expect("validate error");

        let mut runner = Runner::new();
        for stmt in program.expressions {
            runner_interpretator(&mut runner, stmt);
        }

        let final_i = get_primitive_value(
            &mut runner,
            Expr::VariableRef {
                name: "i".to_string(),
                symbol: Symbol {
                    typ: Type::Integer,
                    is_mutable: true,
                    is_used: false,
                    is_changed: false,
                },
            },
            None,
        );
        assert_eq!(final_i, Value::Number(7));
    }

    #[test]
    fn while_loop_continue_break_together() {
        let mut runner = Runner::new();

        let decl = Ast::Decl {
            name: "i".to_string(),
            typ: Type::Integer,
            is_mutable: true,
            value: Box::new(Expr::Number(0)),
        };
        runner_interpretator(&mut runner, decl);

        let condition = Expr::BinaryOp {
            left: Box::new(Expr::VariableRef {
                name: "i".to_string(),
                symbol: Symbol {
                    typ: Type::Integer, is_mutable: true, is_used: false, is_changed: false,
                },
            }),
            right: Box::new(Expr::Number(10)),
            op: Operation::Less,
            return_type: Type::Bool,
        };

        let inc_i = Ast::Assignment {
            name: "i".to_string(),
            value: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::VariableRef {
                    name: "i".to_string(),
                    symbol: Symbol {
                        typ: Type::Integer, is_mutable: true, is_used: false, is_changed: false,
                    },
                }),
                right: Box::new(Expr::Number(1)),
                op: Operation::Add,
                return_type: Type::Integer,
            }),
        };

        let i_eq_3 = Expr::BinaryOp {
            left: Box::new(Expr::VariableRef {
                name: "i".to_string(),
                symbol: Symbol {
                    typ: Type::Integer, is_mutable: true, is_used: false, is_changed: false,
                },
            }),
            right: Box::new(Expr::Number(3)),
            op: Operation::Equal,
            return_type: Type::Bool,
        };
        let continue_if = Ast::Condition {
            main: validator::ast::IF {
                condition: Box::new(i_eq_3),
                body: vec![Ast::Expr(Expr::Continue)],
            },
            elif: vec![],
            other: None,
        };

        let i_eq_7 = Expr::BinaryOp {
            left: Box::new(Expr::VariableRef {
                name: "i".to_string(),
                symbol: Symbol {
                    typ: Type::Integer, is_mutable: true, is_used: false, is_changed: false,
                },
            }),
            right: Box::new(Expr::Number(7)),
            op: Operation::Equal,
            return_type: Type::Bool,
        };
        let break_if = Ast::Condition {
            main: validator::ast::IF {
                condition: Box::new(i_eq_7),
                body: vec![Ast::Expr(Expr::Break)],
            },
            elif: vec![],
            other: None,
        };

        let while_stmt = Ast::While {
            condition: Box::new(condition),
            body: vec![inc_i, continue_if, break_if],
        };

        runner_interpretator(&mut runner, while_stmt);

        let final_i = get_primitive_value(
            &mut runner,
            Expr::VariableRef {
                name: "i".to_string(),
                symbol: Symbol {
                    typ: Type::Integer, is_mutable: true, is_used: false, is_changed: false,
                },
            },
            None,
        );
        assert_eq!(final_i, Value::Number(7));
    }
}
