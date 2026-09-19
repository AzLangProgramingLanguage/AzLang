use string_cache::Atom;
use tokenizer::tokens::Token;

use crate::{
    ast::{
        Expr,
        Statement::{self},
    },
    errors::ParserError,
    r#loop::parse_loop,
    tests::create_tokens,
};

#[test]
fn parse_loop_test() -> Result<(), ParserError> {
    let mut tokens = create_tokens(vec![
        Token::Loop,
        Token::Indent,
        Token::Identifier("Hello".to_string()),
        Token::LParen,
        Token::RParen,
    ]);
    let parseloop = parse_loop(&mut tokens)?;
    assert_eq!(
        Statement::Loop {
            body: vec![Expr::Call {
                target: None,
                name: Box::new(Expr::VariableRef {
                    name: Atom::from("Hello"),
                    symbol: None
                }),
                args: vec![]
            }]
        },
        parseloop
    );
    Ok(())
}
