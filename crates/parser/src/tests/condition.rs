#[cfg(test)]
mod tests {
    use crate::ast::{Expr, Statement};
    use crate::condition::parse_if_expr;
    use tokenizer::iterator::{SourceSpan, Tokens};
    use tokenizer::tokens::Token;

    fn create_tokens(tokens_vec: Vec<Token>) -> Tokens {
        let mut tokens = Tokens::default();
        for token in tokens_vec {
            tokens.push(
                token,
                SourceSpan {
                    start: 0,
                    end: 0,
                    line: 0,
                },
            );
        }
        tokens
    }

    #[test]
    fn test_parse_if_expr_simple() {
        // Token::Conditional is already consumed by expressions.rs
        let mut tokens = create_tokens(vec![
            Token::True,
            Token::Colon,
            Token::Newline,
            Token::Indent,
            Token::Number(1),
            Token::Newline,
            Token::Dedent,
        ]);
        let result = parse_if_expr(&mut tokens).expect("Failed to parse if expr");
        if let Statement::Condition { main, elif, other } = result {
            assert_eq!(*main.condition, Expr::Bool(true));
            assert_eq!(main.body, vec![Expr::Number(1)]);
            assert!(elif.is_empty());
            assert!(other.is_none());
        } else {
            panic!("Expected Condition statement");
        }
    }

    #[test]
    fn test_parse_if_elif_else() {
        let mut tokens = create_tokens(vec![
            Token::True,
            Token::Colon,
            Token::Newline,
            Token::Indent,
            Token::Number(1),
            Token::Newline,
            Token::Dedent,
            Token::ElseIf,
            Token::False,
            Token::Colon,
            Token::Newline,
            Token::Indent,
            Token::Number(2),
            Token::Newline,
            Token::Dedent,
            Token::Else,
            Token::Colon,
            Token::Newline,
            Token::Indent,
            Token::Number(3),
            Token::Newline,
            Token::Dedent,
        ]);
        let result = parse_if_expr(&mut tokens).expect("Failed to parse if-elif-else expr");
        if let Statement::Condition { main, elif, other } = result {
            assert_eq!(*main.condition, Expr::Bool(true));
            assert_eq!(elif.len(), 1);
            assert_eq!(*elif[0].condition, Expr::Bool(false));
            assert_eq!(elif[0].body, vec![Expr::Number(2)]);
            assert!(other.is_some());
            assert_eq!(other.unwrap().body, vec![Expr::Number(3)]);
        } else {
            panic!("Expected Condition statement");
        }
    }

    #[test]
    fn test_parse_multi_elif() {
        let mut tokens = create_tokens(vec![
            Token::True,
            Token::Colon,
            Token::Newline,
            Token::Indent,
            Token::Number(1),
            Token::Newline,
            Token::Dedent,
            Token::ElseIf,
            Token::False,
            Token::Colon,
            Token::Newline,
            Token::Indent,
            Token::Number(2),
            Token::Newline,
            Token::Dedent,
            Token::ElseIf,
            Token::True,
            Token::Colon,
            Token::Newline,
            Token::Indent,
            Token::Number(3),
            Token::Newline,
            Token::Dedent,
        ]);
        let result = parse_if_expr(&mut tokens).expect("Failed to parse multi-elif expr");
        if let Statement::Condition { elif, .. } = result {
            assert_eq!(elif.len(), 2);
            assert_eq!(elif[1].body, vec![Expr::Number(3)]);
        } else {
            panic!("Expected Condition statement");
        }
    }

    #[test]
    fn test_parse_if_missing_colon() {
        let mut tokens = create_tokens(vec![
            Token::True,
            Token::Newline,
            Token::Indent,
            Token::Number(1),
            Token::Newline,
            Token::Dedent,
        ]);
        let result = parse_if_expr(&mut tokens);
        assert!(result.is_err());
    }
}
