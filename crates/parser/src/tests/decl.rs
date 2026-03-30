use tokenizer::tokens::Token;

use crate::{ast::Statement, binary_op::parse_statement, tests::create_tokens};

#[test]
fn test_parse_statement_decl() {
    let mut tokens = create_tokens(vec![
        Token::ConstantDecl,
        Token::IntegerType,
        Token::Identifier("x".to_string()),
        Token::Assign,
        Token::Number(42),
    ]);
    let result = parse_statement(&mut tokens).expect("decl parse edilmədi");
    assert!(matches!(
        result,
        Statement::Decl {
            is_mutable: false,
            ..
        }
    ));
}

#[test]
fn test_parse_statement_mutable_decl() {
    let mut tokens = create_tokens(vec![
        Token::MutableDecl,
        Token::IntegerType,
        Token::Identifier("y".to_string()),
        Token::Assign,
        Token::Number(10),
        Token::Newline,
    ]);
    let result = parse_statement(&mut tokens).expect("mutable decl parse edilmədi");
    assert!(matches!(
        result,
        Statement::Decl {
            is_mutable: true,
            ..
        }
    ));
}
