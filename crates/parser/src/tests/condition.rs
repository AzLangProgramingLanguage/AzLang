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
}
