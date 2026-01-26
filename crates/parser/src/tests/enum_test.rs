/**
 *! Hey bro. I writed this code with ai.  Please check it. If you have time  
 */
#[cfg(test)]
mod tests {


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


    }

    #[test]
    fn test_parse_enum_decl_missing_name() {
        let tokens = vec![
            Token::Newline,
            Token::Indent,
            Token::Identifier("Red".into()),
            Token::Dedent,
        ];
    }

    #[test]
    fn test_parse_enum_decl_missing_newline() {
        let tokens = vec![
            Token::Identifier("Color".into()),
            Token::Identifier("Red".into()),
            Token::Dedent,
        ];
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
    }
}
