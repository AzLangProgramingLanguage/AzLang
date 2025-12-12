/**
 *! Hey bro. I writed this code with ai.  Please check it. If you have time  
 */
#[cfg(test)]
mod tests {

    use crate::ast::Expr;
    use crate::r#enum::parse_enum_decl;
    use crate::errors::ParserError;

    use peekmore::PeekMore;
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
        if let Ok(Expr::EnumDecl { name, variants }) = result {
            assert_eq!(name, Cow::Borrowed("Color"));
            assert_eq!(
                variants,
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
            Token::Number(42),
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
