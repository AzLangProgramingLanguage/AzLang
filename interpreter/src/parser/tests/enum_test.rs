#[cfg(test)]
mod tests {
    use crate::parser::ast::Expr;
    use crate::parser::r#enum::parse_enum_decl;

    use super::*;
    use errors::ParserError;
    use peekmore::PeekMore;
    use peekmore::PeekMoreIterator;
    use std::borrow::Cow;
    use tokenizer::tokens::Token;

    #[test]
    fn test_parse_enum_decl_success() {
        let tokens = vec![
            Token::Identifier("Color".into()),
            Token::Newline,
            Token::Indent,
            Token::Identifier("Red".into()),
            Token::Newline,
            Token::Identifier("Green".into()),
            Token::Newline,
            Token::Identifier("Blue".into()),
            Token::Dedent,
        ];

        let mut iter = tokens.iter().peekmore();
        let result = parse_enum_decl(&mut iter);

        assert!(result.is_ok());
        if let Ok(Expr::EnumDecl(enum_decl)) = result {
            assert_eq!(enum_decl.name, Cow::Borrowed("Color"));
            assert_eq!(
                enum_decl.variants,
                vec![
                    Cow::Borrowed("Red"),
                    Cow::Borrowed("Green"),
                    Cow::Borrowed("Blue")
                ]
            );
        } else {
            panic!("Expected EnumDecl");
        }
    }

    #[test]
    fn test_parse_enum_decl_missing_name() {
        let tokens = vec![
            Token::Newline,
            Token::Indent,
            Token::Identifier("Red".into()),
            Token::Dedent,
        ];
        let mut iter = tokens.iter().peekmore();
        let result = parse_enum_decl(&mut iter);
        assert!(matches!(result, Err(ParserError::EnumDeclNameNotFound(_))));
    }

    #[test]
    fn test_parse_enum_decl_missing_newline() {
        let tokens = vec![
            Token::Identifier("Color".into()),
            Token::Identifier("Red".into()),
            Token::Dedent,
        ];
        let mut iter = tokens.iter().peekmore();
        let result = parse_enum_decl(&mut iter);
        assert!(matches!(result, Err(ParserError::EnumNewLineNotFound(_))));
    }

    #[test]
    fn test_parse_enum_decl_unexpected_token() {
        let tokens = vec![
            Token::Identifier("Color".into()),
            Token::Newline,
            Token::Indent,
            Token::Newline,
            Token::Number(42), // unexpected token
            Token::Dedent,
        ];
        let mut iter = tokens.iter().peekmore();
        let result = parse_enum_decl(&mut iter);
        assert!(matches!(
            result,
            Err(ParserError::UnexpectedToken(Token::Number(42)))
        ));
    }
}
