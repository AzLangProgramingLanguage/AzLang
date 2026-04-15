#[cfg(test)]
mod tests {
    use crate::ast::{Expr, Statement};
    use crate::condition::parse_if_expr;
    use crate::tests::create_tokens;
    use tokenizer::tokens::Token;

    fn expr_stmt(expr: Expr) -> Statement {
        Statement::Expr(expr)
    }

    #[test]
    fn test_parse_if_expr_simple() {
        let mut tokens = create_tokens(vec![
            Token::Conditional,
            Token::True,
            Token::Newline,
            Token::Indent,
            Token::Number(1),
            Token::Newline,
            Token::Dedent,
        ]);
        let result = parse_if_expr(&mut tokens).expect("if parse edilmədi");

        if let Statement::Condition { main, elif, other } = result {
            assert_eq!(*main.condition, Expr::Bool(true));
            assert_eq!(main.body, vec![expr_stmt(Expr::Number(1))]);
            assert!(elif.is_empty());
            assert!(other.is_none());
        } else {
            panic!("Condition statement gözlənilirdi");
        }
    }

    // #[test]
    // fn test_parse_if_elif_else() {
    //     let mut tokens = create_tokens(vec![
    //         Token::True,
    //         Token::Newline,
    //         Token::Indent,
    //         Token::Number(1),
    //         Token::Newline,
    //         Token::Dedent,
    //         Token::ElseIf,
    //         Token::False,
    //         Token::Newline,
    //         Token::Indent,
    //         Token::Number(2),
    //         Token::Newline,
    //         Token::Dedent,
    //         Token::Else,
    //         Token::Newline,
    //         Token::Indent,
    //         Token::Number(3),
    //         Token::Newline,
    //         Token::Dedent,
    //     ]);
    //     let result = parse_if_expr(&mut tokens).expect("if-elif-else parse edilmədi");
    //
    //     if let Statement::Condition { main, elif, other } = result {
    //         assert_eq!(*main.condition, Expr::Bool(true));
    //         assert_eq!(main.body, vec![expr_stmt(Expr::Number(1))]);
    //
    //         assert_eq!(elif.len(), 1);
    //         assert_eq!(*elif[0].condition, Expr::Bool(false));
    //         assert_eq!(elif[0].body, vec![expr_stmt(Expr::Number(2))]);
    //
    //         let else_branch = other.expect("else branch gözlənilirdi");
    //         assert_eq!(else_branch.body, vec![expr_stmt(Expr::Number(3))]);
    //     } else {
    //         panic!("Condition statement gözlənilirdi");
    //     }
    // }

    // #[test]
    // fn test_parse_multi_elif() {
    //     let mut tokens = create_tokens(vec![
    //         Token::True,
    //         Token::Newline,
    //         Token::Indent,
    //         Token::Number(1),
    //         Token::Newline,
    //         Token::Dedent,
    //         Token::ElseIf,
    //         Token::False,
    //         Token::Newline,
    //         Token::Indent,
    //         Token::Number(2),
    //         Token::Newline,
    //         Token::Dedent,
    //         Token::ElseIf,
    //         Token::True,
    //         Token::Newline,
    //         Token::Indent,
    //         Token::Number(3),
    //         Token::Newline,
    //         Token::Dedent,
    //     ]);
    //     let result = parse_if_expr(&mut tokens).expect("multi-elif parse edilmədi");
    //
    //     if let Statement::Condition { elif, .. } = result {
    //         assert_eq!(elif.len(), 2);
    //         assert_eq!(elif[1].body, vec![expr_stmt(Expr::Number(3))]);
    //     } else {
    //         panic!("Condition statement gözlənilirdi");
    //     }
    // }
}
