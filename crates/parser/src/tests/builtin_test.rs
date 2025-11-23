#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expr;
    use crate::builtin::parse_builtin;
    use crate::errors::ParserError;
    use crate::shared_ast::{BuiltInFunction, Type};
    use peekmore::PeekMore;
    use peekmore::PeekMoreIterator;
    use tokenizer::tokens::Token;

    // Helper to create iterator

    #[test]
    fn test_parse_builtin_simple_no_args() {
        let tokens = vec![Token::Print];
        let mut it = tokens.iter().peekmore();

        let first = it.peek().unwrap().clone();
        let result = parse_builtin(&mut it, first);

        assert!(result.is_ok());

        if let Ok(Expr::BuiltInCall {
            function,
            args,
            return_type,
        }) = result
        {
            assert_eq!(function, BuiltInFunction::Print);
            assert!(args.is_empty());
            assert_eq!(return_type, Type::Void);
        } else {
            panic!("Expected BuiltInCall");
        }
    }

    #[test]
    fn test_parse_builtin_with_args() {
        let tokens = vec![
            Token::Len,
            Token::LParen,
            Token::Identifier("name".into()),
            Token::RParen,
        ];

        let mut it = tokens.iter().peekmore();
        let f = it.peek().unwrap().clone();

        let result = parse_builtin(&mut it, f);
        assert!(result.is_ok());

        if let Ok(Expr::BuiltInCall { function, args, .. }) = result {
            assert_eq!(function, BuiltInFunction::Len);
            assert_eq!(args.len(), 1);
        } else {
            panic!("Expected BuiltInCall");
        }
    }

    #[test]
    fn test_parse_builtin_nested_expression() {
        // len(1 + 2)
        let tokens = vec![
            Token::Len,
            Token::LParen,
            Token::Number(1),
            Token::Operator("+".to_string()),
            Token::Number(2),
            Token::RParen,
        ];

        let mut it = tokens.iter().peekmore();
        let f = it.peek().unwrap().clone();
        let result = parse_builtin(&mut it, f);

        assert!(result.is_ok());

        if let Ok(Expr::BuiltInCall { args, .. }) = result {
            assert_eq!(args.len(), 1);
            // Ensure it's an expression tree
            match &args[0] {
                Expr::BinaryOp { .. } => {} // OK
                other => panic!("Expected BinaryOp, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_parse_builtin_unsupported() {
        let tokens = vec![Token::Identifier("UnknownFn".into())];
        let mut it = tokens.iter().peekmore();

        let first = it.peek().unwrap().clone();
        let result = parse_builtin(&mut it, first);

        assert!(matches!(
            result,
            Err(ParserError::UnsupportedBuiltInFunction(Token::Identifier(
                _
            )))
        ));
    }
}
