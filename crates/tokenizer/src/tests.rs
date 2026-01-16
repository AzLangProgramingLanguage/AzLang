#[cfg(test)]
mod tests {
    use crate::{iterator::SpannedToken, Lexer};

    #[test]
    fn lex() -> SpannedToken {
        let mut lexer = Lexer::new("let a = 1");
        let tokens = lexer.tokenize().unwrap();
        assert!(1==1);

        tokens.into_iter().next().unwrap()
    }
}

    