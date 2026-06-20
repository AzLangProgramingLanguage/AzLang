#[cfg(test)]
mod tests {
    use crate::assign::parse_assign;
    use crate::ast::{Atom, Expr, Operation, Statement};
    use crate::binary_op::parse_statement;
    use crate::decl::parse_decl;
    use crate::shared_ast::{StringEnum, Type};
    use crate::tests::{TestResult, create_tokens};
    use std::rc::Rc;
    use tokenizer::tokens::Token;

    #[test]
    fn test_parse_assign_simple() -> TestResult {
        let mut tokens = create_tokens(vec![
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Number(42),
        ]);
        let result = parse_assign(&mut tokens, "x".to_string())?;

        assert_eq!(
            result,
            Statement::Assignment {
                name: Atom::from("x"),
                value: Box::new(Expr::Number(42))
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_decl_float_mutable() -> TestResult {
        let mut tokens = create_tokens(vec![
            Token::ConstantDecl,
            Token::StringType,
            Token::Identifier("a".to_string()),
            Token::Assign,
            Token::Float(2.1),
        ]);
        let result = parse_decl(&mut tokens, false)?;

        assert_eq!(
            result,
            Statement::Decl {
                name: Atom::from("a"),
                typ: Rc::new(Type::String(StringEnum::DynamicString)),
                is_mutable: false,
                value: Box::new(Expr::Float(2.1)),
            }
        );
        Ok(())
    }
    #[test]
    fn test_parse_assign_float() -> TestResult {
        let mut tokens = create_tokens(vec![
            Token::Identifier("c".to_string()),
            Token::Assign,
            Token::Float(2.1),
        ]);
        let result = parse_statement(&mut tokens)?;
        assert_eq!(
            result,
            Statement::Assignment {
                name: Atom::from("c"),
                value: Box::new(Expr::Float(2.1))
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_assign_missing_equal() {
        let mut tokens = create_tokens(vec![Token::Number(42)]);
        let result = parse_assign(&mut tokens, "x".to_string());
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_assign_complex_expr() -> TestResult {
        let mut tokens = create_tokens(vec![
            Token::Identifier("sum".to_string()),
            Token::Assign,
            Token::Number(10),
            Token::Add,
            Token::Number(20),
        ]);
        let result = parse_assign(&mut tokens, "sum".to_string())?;
        assert_eq!(
            result,
            Statement::Assignment {
                name: Atom::from("sum"),
                value: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(10)),
                    right: Box::new(Expr::Number(20)),
                    op: Operation::Add,
                })
            }
        );
        Ok(())
    }
}
