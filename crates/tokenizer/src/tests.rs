#[cfg(test)]
mod tests {
    use crate::{Lexer, tokens::Token};

    #[test]
    fn lex() {
        let mut lexer = Lexer::new("let a = 1");
        let tokens = lexer.tokenize().unwrap();

        let first = tokens.into_iter().next().unwrap();
        dbg!(first);

        assert!(true);
    }
    #[test]
    fn test_template_string() {
        let mut lexer = Lexer::new("`hello`");
        let tokens: Vec<Token> = lexer.tokenize().unwrap().map(|x| x.token).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Backtick,
                Token::StringLiteral("hello".into()),
                Token::Backtick,
            ]
        );
    }
}

