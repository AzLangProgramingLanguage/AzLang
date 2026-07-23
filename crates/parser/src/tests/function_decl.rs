use std::rc::Rc;

use crate::{
    ast::{Atom, Expr, Parameter, Statement},
    binary_op::parse_statement,
    shared_ast::Type,
    tests::create_tokens,
};
use tokenizer::tokens::Token;

#[test]
fn test_parse_function_def() {
    let mut tokens = create_tokens(vec![
        Token::FunctionDef,
        Token::Identifier("topla".to_string()),
        Token::LParen,
        Token::ConstantDecl,
        Token::IntegerType,
        Token::Identifier("a".to_string()),
        Token::Comma,
        Token::MutableDecl,
        Token::IntegerType,
        Token::Identifier("b".to_string()),
        Token::RParen,
        Token::Colon,
        Token::IntegerType,
        Token::Newline,
        Token::Indent,
        Token::ConstantDecl,
        Token::IntegerType,
        Token::Identifier("x".to_string()),
        Token::Assign,
        Token::Number(42),
        Token::Newline,
        Token::Dedent,
    ]);

    let result = parse_statement(&mut tokens).expect("Function declaration parse edilmədi");

    let Statement::FunctionDef {
        name,
        return_typ,
        params,
        body,
    } = result
    else {
        panic!("FunctionDef statement gözlənilirdi");
    };

    assert_eq!(name, Atom::from("topla"));
    assert_eq!(return_typ, Type::Integer);
    assert_eq!(
        params,
        vec![
            Parameter {
                name: Atom::from("a"),
                typ: Type::Integer,
                is_pointer: false,
            },
            Parameter {
                name: Atom::from("b"),
                typ: Type::Integer,
                is_pointer: true,
            },
        ]
    );
    assert_eq!(
        body,
        vec![Statement::Decl {
            name: Atom::from("x"),
            typ: Rc::new(Type::Integer),
            is_mutable: false,
            value: Box::new(Expr::Number(42)),
        }]
    );
}

#[test]
fn test_parse_external_func_with_link() {
    let mut tokens = create_tokens(vec![
        Token::At,
        Token::Identifier("link".to_string()),
        Token::LParen,
        Token::StringLiteral("printlib".to_string()),
        Token::RParen,
        Token::Newline,
        Token::At,
        Token::Identifier("external".to_string()),
        Token::LParen,
        Token::StringLiteral("../build/printlib.so".to_string()),
        Token::Comma,
        Token::StringLiteral("printValue".to_string()),
        Token::RParen,
        Token::Newline,
        Token::FunctionDef,
        Token::Identifier("print".to_string()),
        Token::LParen,
        Token::ConstantDecl,
        Token::AnyType,
        Token::Identifier("val".to_string()),
        Token::RParen,
        Token::Colon,
        Token::Void,
        Token::Newline,
    ]);

    let result = parse_statement(&mut tokens).expect("External function with @link parse edilmədi");

    let Statement::ExternalFunctionDef {
        name,
        library,
        symbol,
        link_name,
        ..
    } = result
    else {
        panic!("ExternalFunctionDef statement gözlənilirdi");
    };

    assert_eq!(name, Atom::from("print"));
    assert_eq!(library, Atom::from("../build/printlib.so"));
    assert_eq!(symbol, Atom::from("printValue"));
    assert_eq!(link_name, Some(Atom::from("printlib")));
}

#[test]
fn test_parse_external_func_without_link() {
    let mut tokens = create_tokens(vec![
        Token::At,
        Token::Identifier("external".to_string()),
        Token::LParen,
        Token::StringLiteral("../build/printlib.so".to_string()),
        Token::Comma,
        Token::StringLiteral("printValue".to_string()),
        Token::RParen,
        Token::Newline,
        Token::FunctionDef,
        Token::Identifier("print".to_string()),
        Token::LParen,
        Token::ConstantDecl,
        Token::AnyType,
        Token::Identifier("val".to_string()),
        Token::RParen,
        Token::Colon,
        Token::Void,
        Token::Newline,
    ]);

    let result =
        parse_statement(&mut tokens).expect("External function without @link parse edilmədi");

    let Statement::ExternalFunctionDef {
        name,
        library,
        symbol,
        link_name,
        ..
    } = result
    else {
        panic!("ExternalFunctionDef statement gözlənilirdi");
    };

    assert_eq!(name, Atom::from("print"));
    assert_eq!(library, Atom::from("../build/printlib.so"));
    assert_eq!(symbol, Atom::from("printValue"));
    assert!(link_name.is_none());
}
