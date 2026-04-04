use tokenizer::tokens::Token;

use crate::{
    ast::{Expr, Statement},
    binary_op::parse_statement,
    shared_ast::{BuiltInFunction, Type},
    tests::create_tokens,
};

#[test]
fn test_parse_statement_decl() {
    let mut tokens = create_tokens(vec![
        Token::Print,
        Token::LParen,
        Token::StringLiteral("Hello".to_string()),
        Token::RParen,
        Token::Newline,
    ]);
    let result = parse_statement(&mut tokens).expect("decl parse edilmədi");
    let expected = Statement::Expr(Expr::BuiltInCall {
        function: BuiltInFunction::Print,
        args: vec![Expr::String("Hello".to_string())],
        return_type: Type::Void,
    });
    assert_eq!(result, expected);
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
