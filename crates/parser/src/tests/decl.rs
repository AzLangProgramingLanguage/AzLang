use std::rc::Rc;

use tokenizer::tokens::Token;

use crate::{
    ast::{Expr, Statement},
    binary_op::parse_statement,
    shared_ast::{StringEnum, Type},
    tests::create_tokens,
};

#[test]
fn test_parse_statement_decl() {
    let mut tokens = create_tokens(vec![
        Token::ConstantDecl,
        Token::IntegerType,
        Token::Identifier("x".to_string()),
        Token::Assign,
        Token::Number(42),
        Token::Newline,
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
fn test_parse_string_decl() {
    let mut tokens = create_tokens(vec![
        Token::ConstantDecl,
        Token::StringType,
        Token::Identifier('y'.to_string()),
        Token::Assign,
        Token::StringLiteral("Salam".to_string()),
    ]);
    let mut tokens2 = create_tokens(vec![
        Token::MutableDecl,
        Token::StringType,
        Token::Identifier('y'.to_string()),
        Token::Assign,
        Token::StringLiteral("Salam".to_string()),
    ]);
    let result = parse_statement(&mut tokens).expect("String testdə problem oldu");
    let result2 = parse_statement(&mut tokens2).expect("String testdə problem oldu");
    assert_eq!(
        result2,
        Statement::Decl {
            name: "y".to_string(),
            typ: Rc::new(Type::String(StringEnum::LiteralString)),
            is_mutable: true,
            value: Box::new(Expr::String("Salam".to_string()))
        }
    );

    assert_eq!(
        result,
        Statement::Decl {
            name: "y".to_string(),
            typ: Rc::new(Type::String(StringEnum::LiteralConstString)),
            is_mutable: false,
            value: Box::new(Expr::String("Salam".to_string()))
        }
    )
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
