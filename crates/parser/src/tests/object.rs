use crate::{helpers::expect_token, tests::create_tokens};
use tokenizer::tokens::Token;

#[test]
pub fn create_an_object() {
    let mut tokens = create_tokens(vec![Token::Object, Token::Identifier(String::from("Adam"))]);

    assert!(expect_token(&mut tokens, Token::Object).is_ok());
    assert_eq!(
        tokens.next().map(|token| token.token),
        Some(Token::Identifier(String::from("Adam")))
    );
}
