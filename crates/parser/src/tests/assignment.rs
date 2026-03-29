#[cfg(test)]
mod tests {
    use crate::assign::parse_assign;
    use crate::ast::{Expr, Statement};
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
    fn test_parse_assign_simple() {
        // Identifier is already consumed by the caller (expressions.rs),
        // so we start with '=' (Token::Assign).
        let mut tokens = create_tokens(vec![Token::Assign, Token::Number(42)]);
        let result = parse_assign(&mut tokens, "x".to_string()).expect("Failed to parse assign");
        if let Statement::Assignment { name, value, .. } = result {
            assert_eq!(name, "x");
            assert_eq!(*value, Expr::Number(42));
        } else {
            panic!("Expected Assignment statement");
        }
    }

    #[test]
    fn test_parse_assign_missing_equal() {
        let mut tokens = create_tokens(vec![Token::Number(42)]);
        let result = parse_assign(&mut tokens, "x".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_assign_complex_expr() {
        let mut tokens = create_tokens(vec![
            Token::Assign,
            Token::Number(10),
            Token::Add,
            Token::Number(20),
        ]);
        let result =
            parse_assign(&mut tokens, "sum".to_string()).expect("Failed to parse complex assign");
        if let Statement::Assignment { name, value, .. } = result {
            assert_eq!(name, "sum");
            if let Expr::BinaryOp {
                left, right, op, ..
            } = *value
            {
                assert_eq!(*left, Expr::Number(10));
                assert_eq!(*right, Expr::Number(20));
                assert_eq!(op, crate::ast::Operation::Add);
            } else {
                panic!("Expected BinaryOp expression");
            }
        } else {
            panic!("Expected Assignment statement");
        }
    }
}
