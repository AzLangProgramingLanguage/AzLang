#[cfg(test)]
mod tests {
    use crate::ast::{Expr, Statement};
    use crate::tests::create_tokens;
    use crate::r#while_loop::parse_while_loop;
    use tokenizer::tokens::Token;

    fn expr_stmt(expr: Expr) -> Statement {
        Statement::Expr(expr)
    }

    #[test]
    fn test_parse_while_loop_bool_condition() {
        let mut tokens = create_tokens(vec![
            Token::While,
            Token::True,
            Token::Newline,
            Token::Indent,
            Token::Number(1),
            Token::Newline,
            Token::Dedent,
        ]);
        let result = parse_while_loop(&mut tokens).expect("while loop parse edilmədi");

        if let Statement::While { condition, body } = result {
            assert_eq!(*condition, Expr::Bool(true));
            assert_eq!(body, vec![expr_stmt(Expr::Number(1))]);
        } else {
            panic!("While statement gözlənilirdi");
        }
    }

    #[test]
    fn test_parse_while_loop_comparison_condition() {
        let mut tokens = create_tokens(vec![
            Token::While,
            Token::Identifier("x".to_string()),
            Token::Less,
            Token::Number(10),
            Token::Newline,
            Token::Indent,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Identifier("x".to_string()),
            Token::Add,
            Token::Number(1),
            Token::Newline,
            Token::Dedent,
        ]);
        let result = parse_while_loop(&mut tokens).expect("while loop parse edilmədi");

        if let Statement::While { condition, body } = result {
            assert_eq!(
                *condition,
                Expr::BinaryOp {
                    left: Box::new(Expr::VariableRef {
                        name: "x".into(),
                        symbol: None,
                    }),
                    right: Box::new(Expr::Number(10)),
                    op: crate::ast::Operation::Less,
                }
            );
            assert_eq!(body.len(), 1);
        } else {
            panic!("While statement gözlənilirdi");
        }
    }

    #[test]
    fn test_parse_while_loop_multi_statement_body() {
        let mut tokens = create_tokens(vec![
            Token::While,
            Token::True,
            Token::Newline,
            Token::Indent,
            Token::Number(1),
            Token::Newline,
            Token::Number(2),
            Token::Newline,
            Token::Dedent,
        ]);
        let result = parse_while_loop(&mut tokens).expect("while loop parse edilmədi");

        if let Statement::While { condition, body } = result {
            assert_eq!(*condition, Expr::Bool(true));
            assert_eq!(body.len(), 2);
            assert_eq!(body[0], expr_stmt(Expr::Number(1)));
            assert_eq!(body[1], expr_stmt(Expr::Number(2)));
        } else {
            panic!("While statement gözlənilirdi");
        }
    }
}
