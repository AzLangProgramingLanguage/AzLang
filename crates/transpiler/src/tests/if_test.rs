#[cfg(test)]
mod tests {

    use crate::{TranspileContext, builtin::print, transpile::transpile_stmt};
    use parser::{
        ast::{Expr, IF, Operation, Statement, Symbol, TemplateChunk},
        shared_ast::Type,
    };
    #[test]
    fn if_bool_test() {
        let mut ctx = TranspileContext::default();
        let expr = Expr::Bool(true);
        let expr2 = Expr::Bool(false);
        let result2 = transpile_stmt(
            Statement::Condition {
                main: IF {
                    condition: Box::new(expr2),
                    body: vec![],
                },
                elif: vec![],
                other: None,
            },
            &mut ctx,
        );
        let result = transpile_stmt(
            Statement::Condition {
                main: IF {
                    condition: Box::new(expr),
                    body: vec![],
                },
                elif: vec![],
                other: None,
            },
            &mut ctx,
        );
        assert_eq!(result, "if(true\n) {  }      \n      ");
        assert_eq!(result2, "if(false\n) {  }      \n      ");
    }
    #[test]
    fn if_binary_op_test() {
        let mut ctx = TranspileContext::default();
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Number(1)),
            right: Box::new(Expr::Number(1)),
            op: Operation::Equal,
            return_type: Type::Integer,
        };
        let expr2 = Expr::Bool(false);
        let result2 = transpile_stmt(
            Statement::Condition {
                main: IF {
                    condition: Box::new(expr2),
                    body: vec![],
                },
                elif: vec![],
                other: None,
            },
            &mut ctx,
        );
        let result = transpile_stmt(
            Statement::Condition {
                main: IF {
                    condition: Box::new(expr),
                    body: vec![],
                },
                elif: vec![],
                other: None,
            },
            &mut ctx,
        );
        assert_eq!(result, "if(1 == 1\n) {  }      \n      ");
        assert_eq!(result2, "if(false\n) {  }      \n      ");
    }
}
