use string_cache::{Atom, EmptyStaticAtomSet};
use tokenizer::{iterator::SpannedToken, tokens::Token};

use crate::{ast::Statement, errors::ParserError, helpers::expect_token, tests::create_tokens};

#[test]
fn parser_enum_test() -> Result<(), ParserError> {
    let mut tokens = create_tokens(vec![
        Token::Enum,
        Token::Identifier("Color".to_string()),
        Token::Newline,
        Token::Indent,
        Token::Identifier("Red".to_string()),
    ]);
    expect_token(&mut tokens, Token::Enum)?;
    let name = match tokens.next() {
        Some(SpannedToken {
            token: Token::Identifier(n),
            ..
        }) => Atom::from(n),
        _ => panic!(),
    };
    let mut variants: Vec<Atom<EmptyStaticAtomSet>> = vec![];

    while let Some(SpannedToken {
        token: Token::Newline,
        ..
    }) = tokens.next()
    {
        expect_token(&mut tokens, Token::Indent)?;

        if let Some(SpannedToken {
            token: Token::Identifier(n),
            ..
        }) = tokens.next()
        {
            variants.push(Atom::from(n));
        }
    }

    assert_eq!(
        Statement::EnumDecl {
            name: Atom::from("Color".to_string()),
            variants: vec![Atom::from("Red".to_string())],
        },
        Statement::EnumDecl { name, variants }
    );

    Ok(())
}
