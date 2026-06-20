#[cfg(test)]
use std::rc::Rc;

use crate::{
    ast::{Atom, Expr, Parameter, Statement},
    binary_op::parse_statement,
    shared_ast::Type,
    tests::create_tokens,
};
use std::assert_matches;
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

    assert_matches!(
        result,
        Statement::FunctionDef {
            ref name,
            ref return_typ,
            ref params,
            ref body,
        }
        if
            name == "topla"
            && *return_typ == Type::Integer
            && params.len() == 2
            && params[0] == Parameter {
                name: Atom::from("a"),
                typ: Type::Integer,
                is_pointer: false,
            }
            && params[1] == Parameter {
                name: Atom::from("b"),
                typ: Type::Integer,
                is_pointer: true,
            }
            && body.len() == 1
            && body[0] == Statement::Decl {
                name: Atom::from("x"),
                typ: Rc::new(Type::Integer),
                is_mutable: false,
                value: Box::new(Expr::Number(42)),
            }
    );
}
