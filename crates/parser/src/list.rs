use std::borrow::Cow;

use tokenizer::tokens::Token;

use crate::{ast::Expr, typed_ast::TypedExpr};

pub fn parse_list<'a, I>(tokens: &mut I) -> Expr<'a>
where
    I: Iterator<Item = &'a Token>,
{
    let mut elements = Vec::new();

    for token in tokens.by_ref() {
        match token {
            Token::ListEnd => break,
            Token::StringLiteral(s) => elements.push(Expr::String(s, false)),
            Token::True => elements.push(Expr::Bool(true)),
            Token::False => elements.push(Expr::Bool(false)),
            Token::Float(num) => elements.push(Expr::Float(*num)),
            Token::Number(num) => elements.push(Expr::Number(*num)),
            Token::This => elements.push(Expr::VariableRef {
                name: Cow::Borrowed("self"),
                symbol: None,
            }),
            Token::Comma => continue,
            Token::Newline => continue,
            Token::Semicolon => continue,
            _ => continue,
        }
    }
    Expr::List(elements)
}

pub fn parse_list_typed<'a, I>(tokens: &mut I) -> TypedExpr<'a>
where
    I: Iterator<Item = &'a Token>,
{
    let mut elements = Vec::new();

    for token in tokens.by_ref() {
        match token {
            Token::ListEnd => break,
            Token::StringLiteral(s) => elements.push(TypedExpr::String(s, false)),
            Token::True => elements.push(TypedExpr::Bool(true)),
            Token::False => elements.push(TypedExpr::Bool(false)),
            Token::Float(num) => elements.push(TypedExpr::Float(*num)),
            Token::Number(num) => elements.push(TypedExpr::Number(*num)),
            Token::This => elements.push(TypedExpr::VariableRef {
                name: Cow::Borrowed("self"),
                transpiled_name: None,
                symbol: None,
            }),
            Token::Comma => continue,
            Token::Newline => continue,
            Token::Semicolon => continue,
            _ => continue,
        }
    }
    TypedExpr::List(elements)
}
